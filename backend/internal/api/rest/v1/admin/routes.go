package admin

import (
	"github.com/anthropics/agentsmesh/backend/internal/config"
	"github.com/anthropics/agentsmesh/backend/internal/infra/database"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	"github.com/anthropics/agentsmesh/backend/internal/service/admin"
	"github.com/anthropics/agentsmesh/backend/internal/service/auth"
	"github.com/anthropics/agentsmesh/backend/internal/service/billing"

	"github.com/gin-gonic/gin"
)

// Services contains all admin-related services
type Services struct {
	Auth    *auth.Service
	Admin   *admin.Service
	Billing *billing.Service
}

// RegisterRoutes mounts the only remaining REST surface for the admin
// console: login + /me. Everything else (dashboard, audit-logs,
// promo-codes, users, organizations, runners, relays, subscriptions,
// skill-registries, sso, support-tickets) lives on Connect-RPC under
// backend/internal/api/connect/admin/*.
func RegisterRoutes(router *gin.Engine, cfg *config.Config, db database.DB, svc *Services) {
	adminAPI := router.Group("/api/v1/admin")

	authHandler := NewAuthHandler(svc.Auth, cfg)
	authHandler.RegisterRoutes(adminAPI)

	protected := adminAPI.Group("")
	protected.Use(middleware.AuthMiddleware(cfg.JWT.Secret))
	protected.Use(middleware.AdminMiddleware(db))
	protected.GET("/me", authHandler.GetMe)
}
