package v1

import (
	"fmt"
	"net/http"

	"github.com/anthropics/agentsmesh/backend/internal/domain/billing"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	"github.com/anthropics/agentsmesh/backend/internal/service/payment"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// CustomerPortalRequest represents a customer portal request
type CustomerPortalRequest struct {
	ReturnURL string `json:"return_url" binding:"required"`
}

// GetCustomerPortal returns a customer portal URL (Stripe or LemonSqueezy).
// REST-only — Stripe/LemonSqueezy customer portal redirects are
// provider-owned URL flows, not a domain RPC. The proto SSOT doesn't
// pin this surface.
func (h *BillingHandler) GetCustomerPortal(c *gin.Context) {
	tenant := c.MustGet("tenant").(*middleware.TenantContext)

	if tenant.UserRole != "owner" {
		apierr.Forbidden(c, apierr.INSUFFICIENT_PERMISSIONS, "insufficient permissions")
		return
	}

	var req CustomerPortalRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	sub, err := h.billingService.GetSubscription(c.Request.Context(), tenant.OrganizationID)
	if err != nil {
		apierr.ResourceNotFound(c, "no active subscription")
		return
	}

	factory := h.billingService.GetPaymentFactory()
	if factory == nil {
		apierr.ServiceUnavailable(c, apierr.SERVICE_UNAVAILABLE, "payment service not configured")
		return
	}

	provider, customerID, subscriptionID, err := resolveCustomerPortalProvider(factory, sub)
	if err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}
	if provider == nil {
		apierr.BadRequest(c, apierr.VALIDATION_FAILED, "no payment provider associated with this subscription")
		return
	}

	subProvider, ok := provider.(payment.SubscriptionProvider)
	if !ok {
		apierr.InternalError(c, "provider does not support customer portal")
		return
	}

	portalReq := &payment.CustomerPortalRequest{
		CustomerID:     customerID,
		SubscriptionID: subscriptionID,
		ReturnURL:      req.ReturnURL,
	}

	resp, err := subProvider.GetCustomerPortalURL(c.Request.Context(), portalReq)
	if err != nil {
		apierr.InternalError(c, fmt.Sprintf("failed to create portal session: %v", err))
		return
	}

	c.JSON(http.StatusOK, gin.H{"url": resp.URL})
}

// resolveCustomerPortalProvider picks the active payment provider based on
// stored subscription IDs (LemonSqueezy first, then Stripe — matches the
// historical REST ordering).
func resolveCustomerPortalProvider(factory *payment.Factory, sub *billing.Subscription) (payment.Provider, string, string, error) {
	if sub.LemonSqueezyCustomerID != nil {
		provider, err := factory.GetProvider(billing.PaymentProviderLemonSqueezy)
		if err != nil {
			return nil, "", "", err
		}
		subscriptionID := ""
		if sub.LemonSqueezySubscriptionID != nil {
			subscriptionID = *sub.LemonSqueezySubscriptionID
		}
		return provider, *sub.LemonSqueezyCustomerID, subscriptionID, nil
	}
	if sub.StripeCustomerID != nil {
		provider, err := factory.GetProvider(billing.PaymentProviderStripe)
		if err != nil {
			return nil, "", "", err
		}
		subscriptionID := ""
		if sub.StripeSubscriptionID != nil {
			subscriptionID = *sub.StripeSubscriptionID
		}
		return provider, *sub.StripeCustomerID, subscriptionID, nil
	}
	return nil, "", "", nil
}
