package v1

import (
	"net/http"
	"strconv"

	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// GetBlock GET /blocks/:id
func (h *BlockstoreHandler) GetBlock(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	id, err := uuid.Parse(c.Param("id"))
	if err != nil {
		apierr.ValidationError(c, "invalid block id")
		return
	}
	b, err := h.service.GetBlock(c.Request.Context(), actor, id)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, b)
}

// ListChildren GET /blocks/:id/children?rel=nest
func (h *BlockstoreHandler) ListChildren(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	id, err := uuid.Parse(c.Param("id"))
	if err != nil {
		apierr.ValidationError(c, "invalid block id")
		return
	}
	rel := c.DefaultQuery("rel", "nest")
	res, err := h.service.ListChildren(c.Request.Context(), actor, id, rel)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, res)
}

// ListBacklinks GET /blocks/:id/backlinks
func (h *BlockstoreHandler) ListBacklinks(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	id, err := uuid.Parse(c.Param("id"))
	if err != nil {
		apierr.ValidationError(c, "invalid block id")
		return
	}
	refs, err := h.service.ListBacklinks(c.Request.Context(), actor, id)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"refs": refs})
}

// GetSubtree GET /blocks/workspaces/:ws_id/subtree?root=<uuid>&max_depth=N
func (h *BlockstoreHandler) GetSubtree(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	rootID, err := uuid.Parse(c.Query("root"))
	if err != nil {
		apierr.ValidationError(c, "invalid root id")
		return
	}
	maxDepth, _ := strconv.Atoi(c.DefaultQuery("max_depth", "64"))
	res, err := h.service.ListSubtree(c.Request.Context(), actor, wsID, rootID, maxDepth)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, res)
}

// StreamOps GET /blocks/workspaces/:ws_id/ops?after=<id>&limit=N
// Used by clients to catch up missed ops after reconnect.
func (h *BlockstoreHandler) StreamOps(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	after, _ := strconv.ParseInt(c.DefaultQuery("after", "0"), 10, 64)
	limit, _ := strconv.Atoi(c.DefaultQuery("limit", "200"))
	ops, err := h.service.StreamOps(c.Request.Context(), actor, wsID, after, limit)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"ops": ops})
}

// ExportWorkspace GET /blocks/workspaces/:ws_id/export
// Returns the full workspace contents (blocks + refs + ops) as a single JSON
// document. Callers that want to stream can page through /subtree + /ops
// themselves; this endpoint is for backup / template / inspection.
func (h *BlockstoreHandler) ExportWorkspace(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	out, err := h.service.ExportWorkspace(c.Request.Context(), actor, wsID)
	if translateErr(c, err) {
		return
	}
	c.Header("Content-Disposition",
		`attachment; filename="blockstore-`+wsID.String()+`.json"`)
	c.JSON(http.StatusOK, out)
}

// GetBlockAt GET /blocks/:id/at?op_id=N
// Returns the block's reconstructed state at op N (inclusive). op_id=0
// returns the earliest known snapshot (the initial createBlock only).
func (h *BlockstoreHandler) GetBlockAt(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	id, err := uuid.Parse(c.Param("id"))
	if err != nil {
		apierr.ValidationError(c, "invalid block id")
		return
	}
	opID, _ := strconv.ParseInt(c.DefaultQuery("op_id", "0"), 10, 64)
	snap, err := h.service.GetBlockAt(c.Request.Context(), actor, id, opID)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, snap)
}

// ListTypeDefs GET /blocks/workspaces/:ws_id/type-defs
// Returns every block_type_def in the workspace as raw Block rows. The
// frontend scans these to build a live indicator registry — type_defs live
// outside the nest hierarchy so they'd otherwise never reach the store on
// first load.
func (h *BlockstoreHandler) ListTypeDefs(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	blocks, err := h.service.ListTypeDefBlocks(c.Request.Context(), actor, wsID)
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"blocks": blocks})
}
