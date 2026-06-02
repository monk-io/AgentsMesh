package mockagent

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"
)

// mcpClient is a minimal MCP-over-HTTP JSON-RPC 2.0 client the control agent
// uses to observe and drive the target pod, mirroring how the real claude CLI
// calls the autopilot-control MCP server.
type mcpClient struct {
	url    string
	podKey string // X-Pod-Key = the target pod this control agent supervises
	http   *http.Client
	nextID int
}

// newMCPClientFromConfig reads the .mcp.json the runner generated (via
// --mcp-config) and extracts the autopilot-control url + X-Pod-Key. Returns
// nil when the config is missing/unparseable — the control agent then runs
// blind (decisions still flow, the pod just isn't observed/driven).
func newMCPClientFromConfig(path string) *mcpClient {
	if path == "" {
		return nil
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return nil
	}
	var cfg struct {
		MCPServers map[string]struct {
			URL     string            `json:"url"`
			Headers map[string]string `json:"headers"`
		} `json:"mcpServers"`
	}
	if err := json.Unmarshal(data, &cfg); err != nil {
		return nil
	}
	srv, ok := cfg.MCPServers["autopilot-control"]
	if !ok || srv.URL == "" {
		return nil
	}
	return &mcpClient{
		url:    srv.URL,
		podKey: srv.Headers["X-Pod-Key"],
		http:   &http.Client{Timeout: 10 * time.Second},
	}
}

func (c *mcpClient) callTool(name string, args map[string]any) (string, error) {
	c.nextID++
	reqBody, _ := json.Marshal(map[string]any{
		"jsonrpc": "2.0",
		"id":      c.nextID,
		"method":  "tools/call",
		"params":  map[string]any{"name": name, "arguments": args},
	})
	req, err := http.NewRequest(http.MethodPost, c.url, bytes.NewReader(reqBody))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-Pod-Key", c.podKey)

	resp, err := c.http.Do(req)
	if err != nil {
		return "", err
	}
	defer func() { _ = resp.Body.Close() }()

	var out struct {
		Result struct {
			Content []struct {
				Text string `json:"text"`
			} `json:"content"`
		} `json:"result"`
		Error *struct {
			Message string `json:"message"`
		} `json:"error"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&out); err != nil {
		return "", err
	}
	if out.Error != nil {
		return "", fmt.Errorf("mcp error: %s", out.Error.Message)
	}
	if len(out.Result.Content) > 0 {
		return out.Result.Content[0].Text, nil
	}
	return "", nil
}

func (c *mcpClient) getPodSnapshot(lines int) (string, error) {
	return c.callTool("get_pod_snapshot", map[string]any{"pod_key": c.podKey, "lines": lines})
}

func (c *mcpClient) sendPodInput(text string) (string, error) {
	return c.callTool("send_pod_input", map[string]any{"pod_key": c.podKey, "text": text})
}

func (c *mcpClient) getPodStatus() (string, error) {
	return c.callTool("get_pod_status", map[string]any{"pod_key": c.podKey})
}
