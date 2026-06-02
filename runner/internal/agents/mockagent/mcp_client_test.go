package mockagent

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// writeMCPConfig writes a .mcp.json matching what the runner generates and
// returns its path. Shared by the engine + client tests.
func writeMCPConfig(t *testing.T, url, podKey string) string {
	t.Helper()
	cfg := map[string]any{
		"mcpServers": map[string]any{
			"autopilot-control": map[string]any{
				"type":    "http",
				"url":     url,
				"headers": map[string]string{"Content-Type": "application/json", "X-Pod-Key": podKey},
			},
		},
	}
	b, err := json.Marshal(cfg)
	require.NoError(t, err)
	path := filepath.Join(t.TempDir(), ".mcp.json")
	require.NoError(t, os.WriteFile(path, b, 0o644))
	return path
}

func TestNewMCPClientFromConfig(t *testing.T) {
	path := writeMCPConfig(t, "http://127.0.0.1:19000/mcp", "pod-abc")
	c := newMCPClientFromConfig(path)
	require.NotNil(t, c)
	assert.Equal(t, "http://127.0.0.1:19000/mcp", c.url)
	assert.Equal(t, "pod-abc", c.podKey)
}

func TestNewMCPClientFromConfig_MissingReturnsNil(t *testing.T) {
	assert.Nil(t, newMCPClientFromConfig(""))
	assert.Nil(t, newMCPClientFromConfig(filepath.Join(t.TempDir(), "nope.json")))
}

func TestMCPClient_CallToolSendsJSONRPC(t *testing.T) {
	var gotName, gotPodKey string
	var gotArgs map[string]any
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gotPodKey = r.Header.Get("X-Pod-Key")
		var body struct {
			Method string `json:"method"`
			Params struct {
				Name      string         `json:"name"`
				Arguments map[string]any `json:"arguments"`
			} `json:"params"`
		}
		require.NoError(t, json.NewDecoder(r.Body).Decode(&body))
		assert.Equal(t, "tools/call", body.Method)
		gotName = body.Params.Name
		gotArgs = body.Params.Arguments
		_ = json.NewEncoder(w).Encode(map[string]any{
			"jsonrpc": "2.0", "id": 1,
			"result": map[string]any{"content": []map[string]any{{"type": "text", "text": "snapshot text"}}},
		})
	}))
	defer srv.Close()

	c := &mcpClient{url: srv.URL, podKey: "pod-x", http: srv.Client()}
	out, err := c.getPodSnapshot(50)
	require.NoError(t, err)
	assert.Equal(t, "snapshot text", out)
	assert.Equal(t, "get_pod_snapshot", gotName)
	assert.Equal(t, "pod-x", gotPodKey)
	assert.Equal(t, "pod-x", gotArgs["pod_key"])
	assert.EqualValues(t, 50, gotArgs["lines"])
}

func TestMCPClient_GetPodStatusSendsPodKey(t *testing.T) {
	var gotName, gotPodKey string
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gotPodKey = r.Header.Get("X-Pod-Key")
		var body struct {
			Params struct {
				Name string `json:"name"`
			} `json:"params"`
		}
		require.NoError(t, json.NewDecoder(r.Body).Decode(&body))
		gotName = body.Params.Name
		_ = json.NewEncoder(w).Encode(map[string]any{
			"jsonrpc": "2.0", "id": 1,
			"result": map[string]any{"content": []map[string]any{{"type": "text", "text": "running"}}},
		})
	}))
	defer srv.Close()

	c := &mcpClient{url: srv.URL, podKey: "pod-z", http: srv.Client()}
	out, err := c.getPodStatus()
	require.NoError(t, err)
	assert.Equal(t, "running", out)
	assert.Equal(t, "get_pod_status", gotName)
	assert.Equal(t, "pod-z", gotPodKey)
}

func TestMCPClient_CallToolPropagatesError(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_ = json.NewEncoder(w).Encode(map[string]any{
			"jsonrpc": "2.0", "id": 1,
			"error": map[string]any{"code": -32602, "message": "bad pod"},
		})
	}))
	defer srv.Close()
	c := &mcpClient{url: srv.URL, podKey: "pod-x", http: srv.Client()}
	_, err := c.sendPodInput("hi")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "bad pod")
}
