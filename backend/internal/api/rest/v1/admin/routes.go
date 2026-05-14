package admin

import (
	"github.com/anthropics/agentsmesh/backend/internal/config"
	"github.com/anthropics/agentsmesh/backend/internal/infra/database"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	"github.com/anthropics/agentsmesh/backend/internal/service/admin"
	"github.com/anthropics/agentsmesh/backend/internal/service/auth"
	"github.com/anthropics/agentsmesh/backend/internal/service/billing"
	"github.com/anthropics/agentsmesh/backend/internal/service/relay"
	ssoservice "github.com/anthropics/agentsmesh/backend/internal/service/sso"
	"github.com/anthropics/agentsmesh/backend/internal/service/supportticket"

	"github.com/gin-gonic/gin"
)

// Services contains all admin-related services
type Services struct {
	Auth          *auth.Service
	Admin         *admin.Service
	Billing       *billing.Service
	SSO           *ssoservice.Service
	RelayManager  *relay.Manager
	SupportTicket *supportticket.Service
}

// RegisterRoutes registers all admin console routes
func RegisterRoutes(router *gin.Engine, cfg *config.Config, db database.DB, svc *Services) {
	// Admin API v1 routes
	adminAPI := router.Group("/api/v1/admin")

	// Auth routes (public - no middleware)
	authHandler := NewAuthHandler(svc.Auth, cfg)
	authHandler.RegisterRoutes(adminAPI)

	// Protected routes (require auth + admin privileges)
	protected := adminAPI.Group("")
	protected.Use(middleware.AuthMiddleware(cfg.JWT.Secret))
	protected.Use(middleware.AdminMiddleware(db))

	// Get current admin user
	protected.GET("/me", authHandler.GetMe)

	// Dashboard
	dashboardHandler := NewDashboardHandler(svc.Admin)
	dashboardHandler.RegisterRoutes(protected)

	// Users + Organizations moved to Connect-RPC. See
	// backend/internal/api/connect/admin/server.go for the AdminService
	// surface (proto.admin.v1.AdminService). The Connect handlers run
	// behind the same admin gate via interceptors.ResolveSystemAdmin.

	// Runners
	runnerHandler := NewRunnerHandler(svc.Admin)
	runnerHandler.RegisterRoutes(protected)

	// Audit Logs
	auditLogHandler := NewAuditLogHandler(svc.Admin)
	auditLogHandler.RegisterRoutes(protected)

	// Promo Codes
	promoCodeHandler := NewPromoCodeHandler(svc.Admin)
	promoCodeHandler.RegisterRoutes(protected)

	// Subscriptions moved to Connect-RPC
	// (backend/internal/api/connect/admin/subscription/server.go,
	// proto.billing.v1.SubscriptionAdminService).

	// Relays (optional - only if relay manager is available)
	if svc.RelayManager != nil {
		relayHandler := NewRelayHandler(svc.Admin, svc.RelayManager)
		relayHandler.RegisterRoutes(protected)
	}

	// Skill Registries moved to Connect-RPC
	// (backend/internal/api/connect/admin/skill_registry/server.go,
	// proto.extension.v1.SkillRegistryAdminService). The mount keeps the
	// same ExtensionRepo != nil gate via mountAdminServices in cmd/server.

	// SSO Configs (optional - only if SSO service is available)
	if svc.SSO != nil {
		ssoHandler := NewSSOHandler(svc.SSO, svc.Admin)
		ssoHandler.RegisterRoutes(protected)
	}

	// Support Tickets (optional - only if support ticket service is available)
	if svc.SupportTicket != nil {
		supportTicketHandler := NewSupportTicketHandler(svc.SupportTicket, svc.Admin)
		supportTicketHandler.RegisterRoutes(protected)
	}
}
