package v1

import (
	"errors"
	"net/http"

	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	billingsvc "github.com/anthropics/agentsmesh/backend/internal/service/billing"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// DowngradeSubscriptionRequest represents a downgrade request
type DowngradeSubscriptionRequest struct {
	PlanName string `json:"plan_name" binding:"required"`
}

// DowngradeSubscription schedules a downgrade to a lower plan at period end.
// REST-only — proto.billing.v1 carries only UpgradeSubscription; downgrade has
// different effective-date semantics (end-of-period vs. immediate billing
// adjustment) that haven't been pinned in the proto SSOT yet.
func (h *BillingHandler) DowngradeSubscription(c *gin.Context) {
	tenant := c.MustGet("tenant").(*middleware.TenantContext)

	if tenant.UserRole != "owner" {
		apierr.Forbidden(c, apierr.INSUFFICIENT_PERMISSIONS, "insufficient permissions")
		return
	}

	var req DowngradeSubscriptionRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	ctx := c.Request.Context()

	sub, err := h.billingService.GetSubscription(ctx, tenant.OrganizationID)
	if err != nil {
		apierr.ResourceNotFound(c, "no active subscription")
		return
	}

	targetPlan, err := h.billingService.GetPlan(ctx, req.PlanName)
	if err != nil {
		apierr.InvalidInput(c, "invalid plan")
		return
	}

	currentPlan := sub.Plan
	if currentPlan == nil {
		currentPlan, _ = h.billingService.GetPlanByID(ctx, sub.PlanID)
	}

	if targetPlan.PricePerSeatMonthly >= currentPlan.PricePerSeatMonthly {
		apierr.BadRequest(c, apierr.VALIDATION_FAILED, "use upgrade endpoint for higher tier plans")
		return
	}

	if targetPlan.MaxUsers > 0 && sub.SeatCount > targetPlan.MaxUsers {
		apierr.RespondWithExtra(c, http.StatusBadRequest, apierr.VALIDATION_FAILED, "current seat count exceeds target plan limit", gin.H{
			"current_seats":     sub.SeatCount,
			"target_plan_limit": targetPlan.MaxUsers,
			"action_required":   "reduce seats before downgrading",
		})
		return
	}

	_, err = h.billingService.UpdateSubscription(ctx, tenant.OrganizationID, req.PlanName)
	if err != nil {
		if errors.Is(err, billingsvc.ErrSeatCountExceedsLimit) {
			apierr.RespondWithExtra(c, http.StatusBadRequest, apierr.VALIDATION_FAILED, "current seat count exceeds target plan limit", gin.H{
				"action_required": "reduce seats before downgrading",
			})
			return
		}
		apierr.InternalError(c, err.Error())
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"message":           "downgrade scheduled for end of billing period",
		"downgrade_to_plan": req.PlanName,
		"effective_date":    sub.CurrentPeriodEnd,
	})
}
