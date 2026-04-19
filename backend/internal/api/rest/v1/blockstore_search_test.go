package v1

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	blockstoreinfra "github.com/anthropics/agentsmesh/backend/internal/infra/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	blockstoreservice "github.com/anthropics/agentsmesh/backend/internal/service/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/testkit"
	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// setupBlockstoreHandler wires Handler + Service over in-memory SQLite and a
// pre-bootstrapped default workspace. Returns everything a test needs to
// drive one request with authenticated tenant context.
func setupBlockstoreHandler(t *testing.T) (*BlockstoreHandler, *blockstoreservice.Service, string) {
	db := testkit.SetupTestDB(t)
	repo := blockstoreinfra.NewRepository(db)
	svc := blockstoreservice.NewService(repo, nil)
	handler := NewBlockstoreHandler(svc)

	actor := blockstoreservice.ActorContext{
		UserID: 100, OrgID: 1,
		ActorType: blockstore.ActorUser, ActorID: 100,
	}
	ws, err := svc.EnsureDefaultWorkspace(t.Context(), actor)
	require.NoError(t, err)

	gin.SetMode(gin.TestMode)
	return handler, svc, ws.ID.String()
}

// withTenant attaches the canonical TenantContext so actorFrom succeeds.
func withTenant(c *gin.Context) {
	c.Set("tenant", &middleware.TenantContext{
		OrganizationID:   1,
		OrganizationSlug: "test-org",
		UserID:           100,
		UserRole:         "member",
	})
}

func TestBlockstoreHandler_SemanticSearch(t *testing.T) {
	handler, svc, wsID := setupBlockstoreHandler(t)
	ctx := t.Context()

	actor := blockstoreservice.ActorContext{
		UserID: 100, OrgID: 1,
		ActorType: blockstore.ActorUser, ActorID: 100,
	}
	_, err := svc.ApplyOps(ctx, actor, blockstoreservice.ApplyOpsInput{
		WorkspaceID: wsID,
		Ops: []blockstoreservice.OpEnvelope{
			{Op: blockstore.OpCreateBlock, Payload: map[string]any{
				"type": blockstore.BlockTypeParagraph,
				"text": "deploy golang backend with docker compose",
			}},
			{Op: blockstore.OpCreateBlock, Payload: map[string]any{
				"type": blockstore.BlockTypeParagraph,
				"text": "plant tomato seedlings in spring",
			}},
		},
	})
	require.NoError(t, err)
	svc.FlushEmbeddings()

	router := gin.New()
	router.POST("/:ws_id/search", func(c *gin.Context) {
		withTenant(c)
		handler.SemanticSearch(c)
	})

	body, _ := json.Marshal(map[string]any{"query": "go docker", "top_k": 5})
	w := httptest.NewRecorder()
	req := httptest.NewRequest("POST", "/"+wsID+"/search", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	c, _ := gin.CreateTestContext(w)
	c.Request = req
	c.Params = gin.Params{{Key: "ws_id", Value: wsID}}
	withTenant(c)
	handler.SemanticSearch(c)

	require.Equal(t, http.StatusOK, w.Code, "body=%s", w.Body.String())
	var resp struct {
		Hits []blockstoreservice.SearchHit `json:"hits"`
	}
	require.NoError(t, json.Unmarshal(w.Body.Bytes(), &resp))
	require.NotEmpty(t, resp.Hits, "expected hits for a go docker query")
	assert.NotContains(t, resp.Hits[0].Snippet, "tomato",
		"top hit should be topical")
}

func TestBlockstoreHandler_MemoryRetrieve(t *testing.T) {
	handler, svc, wsID := setupBlockstoreHandler(t)
	ctx := t.Context()

	actor := blockstoreservice.ActorContext{
		UserID: 100, OrgID: 1,
		ActorType: blockstore.ActorUser, ActorID: 100,
	}
	_, err := svc.ApplyOps(ctx, actor, blockstoreservice.ApplyOpsInput{
		WorkspaceID: wsID,
		Ops: []blockstoreservice.OpEnvelope{
			{Op: blockstore.OpCreateBlock, Payload: map[string]any{
				"type": blockstore.BlockTypeTask,
				"data": map[string]any{"title": "ship release", "status": "todo"},
				"text": "ship release to production",
			}},
		},
	})
	require.NoError(t, err)
	svc.FlushEmbeddings()

	body, _ := json.Marshal(map[string]any{"query": "ship release", "k": 3})
	w := httptest.NewRecorder()
	req := httptest.NewRequest("POST", "/"+wsID+"/memory/retrieve", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	c, _ := gin.CreateTestContext(w)
	c.Request = req
	c.Params = gin.Params{{Key: "ws_id", Value: wsID}}
	withTenant(c)
	handler.MemoryRetrieve(c)

	require.Equal(t, http.StatusOK, w.Code, "body=%s", w.Body.String())
	var resp struct {
		Memories []blockstoreservice.SearchHit `json:"memories"`
	}
	require.NoError(t, json.Unmarshal(w.Body.Bytes(), &resp))
	require.NotEmpty(t, resp.Memories)
	assert.Contains(t, resp.Memories[0].Snippet, "ship")
}

// NOTE: Previous CallTool / ListTools REST tests were removed along with the
// deleted /blocks/mcp/* endpoints (see commit replacing the REST MCP shim with
// gRPC dispatch in runner_adapter_mcp_block.go). Agent-facing tool coverage
// lives in backend/internal/api/grpc/runner_adapter_mcp_block_test.go and
// runner/internal/mcp/http_tools_block_test.go.
