package v1

import (
	"errors"
	"net/http"

	billingsvc "github.com/anthropics/agentsmesh/backend/internal/service/billing"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// ListPlansWithPrices returns all available subscription plans with prices in specified currency
// GET /api/v1/billing/plans/prices?currency=USD
func (h *BillingHandler) ListPlansWithPrices(c *gin.Context) {
	currency := c.DefaultQuery("currency", "USD")

	plans, err := h.billingService.ListPlansWithPrices(c.Request.Context(), currency)
	if err != nil {
		apierr.InternalError(c, err.Error())
		return
	}

	c.JSON(http.StatusOK, gin.H{"plans": plans, "currency": currency})
}

// GetPlanPrices returns prices for a specific plan in specified currency
// GET /api/v1/billing/plans/:name/prices?currency=USD
func (h *BillingHandler) GetPlanPrices(c *gin.Context) {
	planName := c.Param("name")
	currency := c.DefaultQuery("currency", "USD")

	price, err := h.billingService.GetPlanPrice(c.Request.Context(), planName, currency)
	if err != nil {
		if errors.Is(err, billingsvc.ErrPlanNotFound) {
			apierr.ResourceNotFound(c, "plan not found")
			return
		}
		if errors.Is(err, billingsvc.ErrPriceNotFound) {
			apierr.ResourceNotFound(c, "price not found for currency")
			return
		}
		apierr.InternalError(c, err.Error())
		return
	}

	c.JSON(http.StatusOK, gin.H{"price": price, "currency": currency})
}

// GetAllPlanPrices returns all prices for a specific plan (all currencies)
// GET /api/v1/billing/plans/:name/all-prices
func (h *BillingHandler) GetAllPlanPrices(c *gin.Context) {
	planName := c.Param("name")

	prices, err := h.billingService.GetPlanPrices(c.Request.Context(), planName)
	if err != nil {
		if errors.Is(err, billingsvc.ErrPlanNotFound) {
			apierr.ResourceNotFound(c, "plan not found")
			return
		}
		apierr.InternalError(c, err.Error())
		return
	}

	c.JSON(http.StatusOK, gin.H{"prices": prices})
}

// GetDeploymentInfo returns deployment type and available payment providers.
// REST-only — kept for the public `/api/v1/config/deployment` endpoint used
// by the marketing/landing pages (no auth). Connect's BillingPublicService
// will own this once the renderer fully migrates.
func (h *BillingHandler) GetDeploymentInfo(c *gin.Context) {
	info := h.billingService.GetDeploymentInfo()
	c.JSON(http.StatusOK, info)
}
