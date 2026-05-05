package v1

import (
	"net/http"
	"strconv"

	blockstoreservice "github.com/anthropics/agentsmesh/backend/internal/service/blockstore"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// SemanticSearch POST /blocks/workspaces/:ws_id/search
// Body: { "query": string, "top_k"?: int, "min_score"?: float, "type"?: string }
//
// Returns ranked block hits. Ranking uses cosine similarity over stored
// embeddings; snippets are drawn from block text for result previews. ACL
// filtering applies so private blocks never appear in another user's results.
func (h *BlockstoreHandler) SemanticSearch(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	var req struct {
		Query    string  `json:"query"`
		TopK     int     `json:"top_k,omitempty"`
		MinScore float32 `json:"min_score,omitempty"`
		Type     string  `json:"type,omitempty"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, "invalid search body")
		return
	}
	hits, err := h.service.SemanticSearch(c.Request.Context(), actor, blockstoreservice.SearchInput{
		WorkspaceID: wsID,
		Query:       req.Query,
		TopK:        req.TopK,
		MinScore:    req.MinScore,
		TypeFilter:  req.Type,
	})
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"hits": hits})
}

// MemoryRetrieve POST /blocks/workspaces/:ws_id/memory/retrieve
// Body: { "query": string, "k"?: int }
//
// Thin alias of SemanticSearch tailored to the Agent long-term-memory use
// case: limited to text/task/paragraph/comment types and defaults to top-5.
// Separate route lets Agents discover memory capability without parsing a
// generic search filter language.
func (h *BlockstoreHandler) MemoryRetrieve(c *gin.Context) {
	actor, ok := actorFrom(c)
	if !ok {
		return
	}
	wsID, err := uuid.Parse(c.Param("ws_id"))
	if err != nil {
		apierr.ValidationError(c, "invalid workspace id")
		return
	}
	var req struct {
		Query string `json:"query"`
		K     int    `json:"k,omitempty"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, "invalid memory body")
		return
	}
	k := req.K
	if k <= 0 {
		k = 5
	}
	if s := c.Query("k"); s != "" {
		if n, err := strconv.Atoi(s); err == nil && n > 0 {
			k = n
		}
	}
	hits, err := h.service.SemanticSearch(c.Request.Context(), actor, blockstoreservice.SearchInput{
		WorkspaceID: wsID,
		Query:       req.Query,
		TopK:        k,
	})
	if translateErr(c, err) {
		return
	}
	c.JSON(http.StatusOK, gin.H{"memories": hits})
}
