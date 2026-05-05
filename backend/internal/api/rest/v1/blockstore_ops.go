package v1

import (
	"net/http"

	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	blockstoreservice "github.com/anthropics/agentsmesh/backend/internal/service/blockstore"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// ApplyOps POST /blocks/ops
// Request body: service.ApplyOpsInput  (workspace_id, ops[], idempotency_key?, parent_op_id?)
// Response 200: service.ApplyOpsResult (op_ids[], was_replay, parent_op_id?)
func (h *BlockstoreHandler) ApplyOps(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	var req blockstoreservice.ApplyOpsInput
	if err := c.ShouldBindJSON(&req); err != nil {
		translateErr(c, err)
		return
	}
	res, err := h.service.ApplyOps(c.Request.Context(), actor, req)
	if translateErr(c, err) {
		return
	}
	status := http.StatusOK
	if !res.WasReplay {
		status = http.StatusCreated
	}
	c.JSON(status, res)
}

// ListWorkspaces GET /blocks/workspaces
func (h *BlockstoreHandler) ListWorkspaces(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	list, err := h.service.ListWorkspaces(c.Request.Context(), actor)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"workspaces": list})
}

// EnsureDefaultWorkspace POST /blocks/workspaces/default
// Returns the caller org's default workspace, creating it with a root page on
// first access.
func (h *BlockstoreHandler) EnsureDefaultWorkspace(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	ws, err := h.service.EnsureDefaultWorkspace(c.Request.Context(), actor)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, ws)
}

// CreateWorkspace POST /blocks/workspaces
// Body: {slug, name?}. Provisions an additional workspace in the caller's
// org (beyond the default). Primary consumer is E2E tests wanting an
// isolated workspace per run so accumulated test data can't affect
// assertions. Production flows that want to offer users named workspaces
// also go through here.
func (h *BlockstoreHandler) CreateWorkspace(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	var req struct {
		Slug string `json:"slug"`
		Name string `json:"name"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		translateErr(c, err)
		return
	}
	ws, err := h.service.CreateWorkspace(c.Request.Context(), actor, req.Slug, req.Name)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusCreated, ws)
}

// DeleteWorkspace DELETE /blocks/workspaces/:ws_id
// Hard-deletes the workspace and every row it owns. Refuses to touch the
// org's default workspace. Primary caller: E2E fixture teardown so each
// isolated workspace is reclaimed after its test completes, preventing the
// dev DB from accumulating Playwright detritus between runs.
func (h *BlockstoreHandler) DeleteWorkspace(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	if err := h.service.DeleteWorkspace(c.Request.Context(), actor, wsID); err != nil {
		translateErr(c, err)
		return
	}
	c.Status(http.StatusNoContent)
}
