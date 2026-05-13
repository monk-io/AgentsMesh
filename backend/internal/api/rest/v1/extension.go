package v1

import (
	"errors"
	"net/http"
	"strconv"

	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	extensionservice "github.com/anthropics/agentsmesh/backend/internal/service/extension"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// maxSkillUploadSize is the maximum file size allowed for skill uploads (50MB)
const maxSkillUploadSize = 50 << 20

// handleServiceError maps service-layer errors to appropriate HTTP error responses
func handleServiceError(c *gin.Context, err error, fallbackMsg string) {
	switch {
	case errors.Is(err, extensionservice.ErrNotFound):
		apierr.ResourceNotFound(c, err.Error())
	case errors.Is(err, extensionservice.ErrForbidden):
		apierr.ForbiddenAdmin(c)
	case errors.Is(err, extensionservice.ErrInvalidScope), errors.Is(err, extensionservice.ErrInvalidInput):
		apierr.ValidationError(c, err.Error())
	case errors.Is(err, extensionservice.ErrAlreadyInstalled):
		apierr.Conflict(c, apierr.ALREADY_EXISTS, err.Error())
	default:
		apierr.InternalError(c, fallbackMsg)
	}
}

// requireOrgAdmin checks if the current user has admin or owner role.
// Returns true if the user is authorized, false otherwise (and sends 403 response).
func requireOrgAdmin(c *gin.Context) bool {
	tenant := middleware.GetTenant(c)
	if tenant.UserRole != "admin" && tenant.UserRole != "owner" {
		apierr.ForbiddenAdmin(c)
		return false
	}
	return true
}

// ExtensionHandler handles extension-related API endpoints
type ExtensionHandler struct {
	extensionSvc *extensionservice.Service
}

// NewExtensionHandler creates a new extension handler
func NewExtensionHandler(extensionSvc *extensionservice.Service) *ExtensionHandler {
	return &ExtensionHandler{
		extensionSvc: extensionSvc,
	}
}

// --- Marketplace ---

// ListMarketSkills returns skills available in the marketplace
// GET /api/v1/organizations/:slug/market/skills?q=&category=
func (h *ExtensionHandler) ListMarketSkills(c *gin.Context) {
	tenant := middleware.GetTenant(c)
	query := c.Query("q")
	category := c.Query("category")

	skills, err := h.extensionSvc.ListMarketSkills(c.Request.Context(), tenant.OrganizationID, query, category)
	if err != nil {
		handleServiceError(c, err, "Failed to list market skills")
		return
	}

	c.JSON(http.StatusOK, gin.H{"skills": skills})
}

// ListMarketMcpServers returns MCP server templates from the marketplace
// GET /api/v1/organizations/:slug/market/mcp-servers?q=&category=&limit=50&offset=0
func (h *ExtensionHandler) ListMarketMcpServers(c *gin.Context) {
	query := c.Query("q")
	category := c.Query("category")

	limit := 50
	if l, err := strconv.Atoi(c.Query("limit")); err == nil && l > 0 {
		limit = l
	}
	offset := 0
	if o, err := strconv.Atoi(c.Query("offset")); err == nil && o >= 0 {
		offset = o
	}

	servers, total, err := h.extensionSvc.ListMarketMcpServers(c.Request.Context(), query, category, limit, offset)
	if err != nil {
		handleServiceError(c, err, "Failed to list market MCP servers")
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"mcp_servers": servers,
		"total":       total,
		"limit":       limit,
		"offset":      offset,
	})
}

