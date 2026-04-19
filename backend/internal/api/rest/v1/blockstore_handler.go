package v1

import (
	"errors"
	"log/slog"
	"net/http"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	blockstoreservice "github.com/anthropics/agentsmesh/backend/internal/service/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// BlockstoreHandler serves the `/blocks/*` family of endpoints.
// All routes require tenant context; actor is derived from the authenticated user.
type BlockstoreHandler struct {
	service *blockstoreservice.Service
}

func NewBlockstoreHandler(svc *blockstoreservice.Service) *BlockstoreHandler {
	return &BlockstoreHandler{service: svc}
}

// actorFrom builds the service-layer ActorContext from the gin context.
// Phase 1 treats every authenticated caller as a user; Agent / Runner signed
// tokens can override ActorType in Phase 2 when we add token-bound middleware.
func actorFrom(c *gin.Context) (blockstoreservice.ActorContext, bool) {
	tc := middleware.GetTenant(c)
	if tc == nil {
		apierr.AbortUnauthorized(c, apierr.AUTH_REQUIRED, "tenant context missing")
		return blockstoreservice.ActorContext{}, false
	}
	return blockstoreservice.ActorContext{
		UserID:    tc.UserID,
		OrgID:     tc.OrganizationID,
		ActorType: blockstore.ActorUser,
		ActorID:   tc.UserID,
	}, true
}

// translateErr maps domain errors to HTTP apierr responses. Uses errors.Is
// so wrapped errors (service-layer %w-formatted with context) still map to
// the right HTTP status — switch-by-identity would otherwise fall through to
// InternalError and mask legitimate 4xx validation failures as 500s.
// Callers return immediately after a non-nil return.
func translateErr(c *gin.Context, err error) bool {
	if err == nil {
		return false
	}
	switch {
	case errors.Is(err, blockstore.ErrWorkspaceNotFound),
		errors.Is(err, blockstore.ErrBlockNotFound),
		errors.Is(err, blockstore.ErrRefNotFound):
		apierr.AbortNotFound(c, apierr.RESOURCE_NOT_FOUND, err.Error())
	case errors.Is(err, blockstore.ErrOrgMismatch),
		errors.Is(err, blockstore.ErrBlockForbidden):
		apierr.AbortForbidden(c, apierr.INSUFFICIENT_PERMISSIONS, err.Error())
	case errors.Is(err, blockstore.ErrUnknownBlockType),
		errors.Is(err, blockstore.ErrUnknownOpKind),
		errors.Is(err, blockstore.ErrInvalidRel),
		errors.Is(err, blockstore.ErrOrderKeyRequired),
		errors.Is(err, blockstore.ErrMissingRequiredKey),
		errors.Is(err, blockstore.ErrColumnValueInvalid),
		errors.Is(err, blockstore.ErrChildNotAllowed),
		errors.Is(err, blockstore.ErrCrossWorkspaceRef),
		errors.Is(err, blockstore.ErrApplyOpsEmpty),
		errors.Is(err, blockstore.ErrEmbeddingDisabled):
		apierr.AbortBadRequest(c, apierr.VALIDATION_FAILED, err.Error())
	case errors.Is(err, blockstore.ErrSingleNestParent),
		errors.Is(err, blockstore.ErrNestCycle),
		errors.Is(err, blockstore.ErrStaleUpdate),
		errors.Is(err, blockstore.ErrWorkspaceAlreadyExists):
		c.AbortWithStatusJSON(http.StatusConflict, gin.H{
			"error": apierr.VALIDATION_FAILED, "message": err.Error(),
		})
	default:
		// Unknown errors must not leak internals to callers. Log the full
		// message for operators and return a generic 500 so attackers can't
		// probe the system by inspecting error bodies (driver name, SQL
		// fragments, file paths, etc.).
		slog.Warn("blockstore.internal_error", "err", err.Error())
		apierr.InternalError(c, "internal error")
	}
	return true
}
