package v1

import (
	envbundleService "github.com/anthropics/agentsmesh/backend/internal/service/envbundle"
	"github.com/gin-gonic/gin"
)

// EnvBundleHandler serves the user-scoped env-bundle CRUD endpoints.
// Base path: /api/v1/users/env-bundles
//
// Handler internals are split across files:
//   - env_bundles_types.go — wire request shapes + small helpers
//   - env_bundles_crud.go  — List / Create / Get / Update / Delete / SetPrimary
type EnvBundleHandler struct {
	svc *envbundleService.Service
}

// NewEnvBundleHandler wires a handler.
func NewEnvBundleHandler(svc *envbundleService.Service) *EnvBundleHandler {
	return &EnvBundleHandler{svc: svc}
}

// RegisterRoutes registers the env-bundle routes under the user RouterGroup.
func (h *EnvBundleHandler) RegisterRoutes(rg *gin.RouterGroup) {
	bundles := rg.Group("/env-bundles")
	{
		bundles.GET("", h.List)
		bundles.POST("", h.Create)
		bundles.GET("/:id", h.Get)
		bundles.PUT("/:id", h.Update)
		bundles.DELETE("/:id", h.Delete)
		bundles.POST("/:id/set-primary", h.SetPrimary)
	}
}
