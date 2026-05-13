package v1

import (
	billingsvc "github.com/anthropics/agentsmesh/backend/internal/service/billing"
	"github.com/gin-gonic/gin"
)

// RegisterBillingHandlers registers REST billing routes that have no Connect-RPC
// equivalent. Connect-RPC owns the subscription / checkout / seats / overview /
// invoices / plans / public-pricing / deployment-info surface; the routes here
// are the remaining gaps until proto coverage catches up.
func RegisterBillingHandlers(rg *gin.RouterGroup, billingService *billingsvc.Service) {
	handler := NewBillingHandler(billingService)

	// Plan pricing (multi-currency). Connect's ListPlans returns flat plan
	// records; the per-currency price tables stay on REST.
	rg.GET("/plans/prices", handler.ListPlansWithPrices)
	rg.GET("/plans/:name/prices", handler.GetPlanPrices)
	rg.GET("/plans/:name/all-prices", handler.GetAllPlanPrices)

	// Usage + quota. No Connect mirror — usage rolls up into GetOverview but
	// the granular endpoints stay here for the dashboard drilldown.
	rg.GET("/usage", handler.GetUsage)
	rg.GET("/usage/history", handler.GetUsageHistory)
	rg.POST("/quota", handler.SetCustomQuota)
	rg.GET("/quota/check", handler.CheckQuota)

	// Provider-side flows. Stripe / LemonSqueezy customer-portal redirect is
	// a provider-owned URL flow, not a domain RPC — staying REST.
	rg.POST("/customer-portal", handler.GetCustomerPortal)
}

// RegisterPublicConfigRoutes registers public REST config routes — Connect's
// BillingPublicService.GetPublicDeploymentInfo owns this in the new wire, but
// the renderer pricing card still reads the legacy `/api/v1/config/deployment`
// during the dual-track window. Delete this once the renderer is on Connect.
func RegisterPublicConfigRoutes(rg *gin.RouterGroup, billingService *billingsvc.Service) {
	handler := NewBillingHandler(billingService)
	rg.GET("/deployment", handler.GetDeploymentInfo)
}
