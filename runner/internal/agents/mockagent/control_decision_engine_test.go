package mockagent

import (
	"encoding/json"
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func newControlTestLogger() *slog.Logger {
	return slog.New(slog.NewTextHandler(io.Discard, nil))
}

type parsedDecision struct {
	Decision struct {
		Type       string  `json:"type"`
		Confidence float64 `json:"confidence"`
		Reasoning  string  `json:"reasoning"`
	} `json:"decision"`
	Action struct {
		Type    string `json:"type"`
		Content string `json:"content"`
		Reason  string `json:"reason"`
	} `json:"action"`
}

func decodeDecision(t *testing.T, s string) parsedDecision {
	t.Helper()
	var d parsedDecision
	require.NoError(t, json.Unmarshal([]byte(s), &d))
	return d
}

func TestControlDecisionEngine_ReplaysScriptByIteration(t *testing.T) {
	e := newControlDecisionEngine("", newControlTestLogger())
	script := `{"decisions":[{"type":"continue","reasoning":"step1","send_input":"echo hi\n"},{"type":"completed","reasoning":"done"}]}`

	d1 := decodeDecision(t, e.next(script))
	assert.Equal(t, "continue", d1.Decision.Type)
	assert.Equal(t, "step1", d1.Decision.Reasoning)
	assert.Equal(t, "send_input", d1.Action.Type)
	assert.Equal(t, "echo hi\n", d1.Action.Content)

	// Resume prompt (iteration 2) is ignored — the parsed script drives it.
	d2 := decodeDecision(t, e.next("resume prompt"))
	assert.Equal(t, "completed", d2.Decision.Type)
	assert.Equal(t, "done", d2.Decision.Reasoning)
}

func TestControlDecisionEngine_ExhaustedScriptContinues(t *testing.T) {
	e := newControlDecisionEngine("", newControlTestLogger())
	d1 := decodeDecision(t, e.next(`{"decisions":[{"type":"give_up","reasoning":"r1"}]}`))
	assert.Equal(t, "give_up", d1.Decision.Type)

	d2 := decodeDecision(t, e.next("x"))
	assert.Equal(t, "continue", d2.Decision.Type)
	assert.Contains(t, d2.Decision.Reasoning, "exhausted")
}

func TestControlDecisionEngine_ParseFailDefaultsContinue(t *testing.T) {
	e := newControlDecisionEngine("", newControlTestLogger())
	d := decodeDecision(t, e.next("not json at all"))
	assert.Equal(t, "continue", d.Decision.Type)
}

func TestControlDecisionEngine_ErrorYieldsEmptyOutput(t *testing.T) {
	e := newControlDecisionEngine("", newControlTestLogger())
	assert.Equal(t, "", e.next(`{"decisions":[{"type":"error"}]}`))
}

// TestControlDecisionEngine_DrivesPodViaMCP proves the control agent really
// observes + drives the target pod over MCP (the full-chain requirement).
func TestControlDecisionEngine_DrivesPodViaMCP(t *testing.T) {
	type call struct {
		name string
		args map[string]any
	}
	var calls []call
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		assert.Equal(t, "pod-target", r.Header.Get("X-Pod-Key"))
		var body struct {
			Params struct {
				Name      string         `json:"name"`
				Arguments map[string]any `json:"arguments"`
			} `json:"params"`
		}
		_ = json.NewDecoder(r.Body).Decode(&body)
		calls = append(calls, call{body.Params.Name, body.Params.Arguments})
		_ = json.NewEncoder(w).Encode(map[string]any{
			"jsonrpc": "2.0", "id": 1,
			"result": map[string]any{"content": []map[string]any{{"type": "text", "text": "ok"}}},
		})
	}))
	defer srv.Close()

	path := writeMCPConfig(t, srv.URL, "pod-target")
	e := newControlDecisionEngine(path, newControlTestLogger())
	_ = e.next(`{"observe":true,"decisions":[{"type":"continue","send_input":"echo go\n"}]}`)

	require.Len(t, calls, 3)
	assert.Equal(t, "get_pod_snapshot", calls[0].name)
	assert.Equal(t, "get_pod_status", calls[1].name)
	assert.Equal(t, "send_pod_input", calls[2].name)
	assert.Equal(t, "echo go\n", calls[2].args["text"])
	assert.Equal(t, "pod-target", calls[2].args["pod_key"])
}
