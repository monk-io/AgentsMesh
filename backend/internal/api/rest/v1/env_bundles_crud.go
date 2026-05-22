package v1

import (
	"errors"
	"net/http"

	"github.com/anthropics/agentsmesh/backend/internal/domain/envbundle"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	envbundleService "github.com/anthropics/agentsmesh/backend/internal/service/envbundle"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// List GET /api/v1/users/env-bundles?kind=&agent_slug=
func (h *EnvBundleHandler) List(c *gin.Context) {
	userID := middleware.GetUserID(c)

	filter := envbundle.OwnerFilter{
		OwnerScope: envbundle.OwnerScopeUser,
		OwnerID:    userID,
		Kind:       c.Query("kind"),
	}
	if agentSlug, ok := c.GetQuery("agent_slug"); ok {
		filter.AgentSlug = &agentSlug
	}

	bundles, err := h.svc.List(c.Request.Context(), filter)
	if err != nil {
		apierr.InternalError(c, "Failed to list env bundles")
		return
	}

	items := make([]*envbundle.Response, 0, len(bundles))
	for _, b := range bundles {
		resp, _ := h.svc.ResponseWithValues(b)
		items = append(items, resp)
	}
	c.JSON(http.StatusOK, gin.H{"items": items})
}

// Create POST /api/v1/users/env-bundles
func (h *EnvBundleHandler) Create(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req CreateEnvBundleRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	bundle, err := h.svc.Create(c.Request.Context(), &envbundleService.CreateParams{
		OwnerScope:  envbundle.OwnerScopeUser,
		OwnerID:     userID,
		AgentSlug:   req.AgentSlug,
		Name:        req.Name,
		Description: req.Description,
		Kind:        req.Kind,
		KindPrimary: req.KindPrimary,
		Data:        req.Data,
	})
	if err != nil {
		switch {
		case errors.Is(err, envbundleService.ErrNameExists):
			apierr.Conflict(c, apierr.ALREADY_EXISTS, "Bundle with this name already exists")
		case errors.Is(err, envbundleService.ErrInvalidKind),
			errors.Is(err, envbundleService.ErrInvalidScope):
			apierr.ValidationError(c, err.Error())
		default:
			apierr.InternalError(c, "Failed to create bundle: "+err.Error())
		}
		return
	}

	resp, _ := h.svc.ResponseWithValues(bundle)
	c.JSON(http.StatusCreated, gin.H{"bundle": resp})
}

// Get GET /api/v1/users/env-bundles/:id
func (h *EnvBundleHandler) Get(c *gin.Context) {
	userID := middleware.GetUserID(c)
	id, err := parseInt64Param(c, "id")
	if err != nil {
		return
	}

	bundle, err := h.svc.Get(c.Request.Context(), envbundle.OwnerScopeUser, userID, id)
	if err != nil {
		if errors.Is(err, envbundleService.ErrNotFound) {
			apierr.ResourceNotFound(c, "Bundle not found")
			return
		}
		apierr.InternalError(c, "Failed to get bundle")
		return
	}
	resp, _ := h.svc.ResponseWithValues(bundle)
	c.JSON(http.StatusOK, gin.H{"bundle": resp})
}

// Update PUT /api/v1/users/env-bundles/:id
func (h *EnvBundleHandler) Update(c *gin.Context) {
	userID := middleware.GetUserID(c)
	id, err := parseInt64Param(c, "id")
	if err != nil {
		return
	}

	var req UpdateEnvBundleRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	bundle, err := h.svc.Update(c.Request.Context(), envbundle.OwnerScopeUser, userID, id, &envbundleService.UpdateParams{
		Name:        req.Name,
		Description: req.Description,
		Kind:        req.Kind,
		KindPrimary: req.KindPrimary,
		Data:        req.Data,
		IsActive:    req.IsActive,
	})
	if err != nil {
		switch {
		case errors.Is(err, envbundleService.ErrNotFound):
			apierr.ResourceNotFound(c, "Bundle not found")
		case errors.Is(err, envbundleService.ErrNameExists):
			apierr.Conflict(c, apierr.ALREADY_EXISTS, "Bundle with this name already exists")
		default:
			apierr.InternalError(c, "Failed to update bundle: "+err.Error())
		}
		return
	}
	resp, _ := h.svc.ResponseWithValues(bundle)
	c.JSON(http.StatusOK, gin.H{"bundle": resp})
}

// Delete DELETE /api/v1/users/env-bundles/:id
func (h *EnvBundleHandler) Delete(c *gin.Context) {
	userID := middleware.GetUserID(c)
	id, err := parseInt64Param(c, "id")
	if err != nil {
		return
	}

	if err := h.svc.Delete(c.Request.Context(), envbundle.OwnerScopeUser, userID, id); err != nil {
		if errors.Is(err, envbundleService.ErrNotFound) {
			apierr.ResourceNotFound(c, "Bundle not found")
			return
		}
		apierr.InternalError(c, "Failed to delete bundle")
		return
	}
	c.JSON(http.StatusOK, gin.H{"message": "Bundle deleted"})
}

// SetPrimary POST /api/v1/users/env-bundles/:id/set-primary
func (h *EnvBundleHandler) SetPrimary(c *gin.Context) {
	userID := middleware.GetUserID(c)
	id, err := parseInt64Param(c, "id")
	if err != nil {
		return
	}

	bundle, err := h.svc.SetPrimary(c.Request.Context(), envbundle.OwnerScopeUser, userID, id)
	if err != nil {
		if errors.Is(err, envbundleService.ErrNotFound) {
			apierr.ResourceNotFound(c, "Bundle not found")
			return
		}
		apierr.InternalError(c, "Failed to set primary")
		return
	}
	resp, _ := h.svc.ResponseWithValues(bundle)
	c.JSON(http.StatusOK, gin.H{"bundle": resp})
}
