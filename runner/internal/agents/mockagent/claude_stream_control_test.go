package mockagent

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const initLine = `{"type":"control_request","request_id":"init_1","request":{"subtype":"initialize"}}`

func runControlAgent(t *testing.T, lines []string) []map[string]any {
	t.Helper()
	in := strings.NewReader(strings.Join(lines, "\n") + "\n")
	var out bytes.Buffer
	runControlAgentWithIO("", in, &out, newControlTestLogger())

	var msgs []map[string]any
	for _, line := range strings.Split(strings.TrimSpace(out.String()), "\n") {
		if line == "" {
			continue
		}
		var m map[string]any
		require.NoError(t, json.Unmarshal([]byte(line), &m))
		msgs = append(msgs, m)
	}
	return msgs
}

func userLine(t *testing.T, content string) string {
	t.Helper()
	b, err := json.Marshal(map[string]any{
		"type":    "user",
		"message": map[string]any{"role": "user", "content": content},
	})
	require.NoError(t, err)
	return string(b)
}

func assistantText(t *testing.T, msg map[string]any) string {
	t.Helper()
	m := msg["message"].(map[string]any)
	content := m["content"].([]any)
	require.NotEmpty(t, content)
	return content[0].(map[string]any)["text"].(string)
}

func TestRunControlAgent_HandshakeClosesInit(t *testing.T) {
	msgs := runControlAgent(t, []string{initLine})
	require.Len(t, msgs, 2)
	assert.Equal(t, "system", msgs[0]["type"])
	assert.Equal(t, "init", msgs[0]["subtype"])
	assert.Equal(t, "control_response", msgs[1]["type"])
	resp := msgs[1]["response"].(map[string]any)
	assert.Equal(t, "success", resp["subtype"])
	assert.Equal(t, "init_1", resp["request_id"])
}

func TestRunControlAgent_PromptEmitsDecisionThenResult(t *testing.T) {
	script := `{"decisions":[{"type":"completed","reasoning":"all good"}]}`
	msgs := runControlAgent(t, []string{initLine, userLine(t, script)})
	require.Len(t, msgs, 4) // system, control_response, assistant, result

	assert.Equal(t, "assistant", msgs[2]["type"])
	var dec parsedDecision
	require.NoError(t, json.Unmarshal([]byte(assistantText(t, msgs[2])), &dec))
	assert.Equal(t, "completed", dec.Decision.Type)
	assert.Equal(t, "all good", dec.Decision.Reasoning)

	assert.Equal(t, "result", msgs[3]["type"])
	assert.Equal(t, "success", msgs[3]["subtype"])
}

// An "error" decision emits a result with NO assistant message, so the runner's
// RunControlProcess sees empty output and counts a consecutive error.
func TestRunControlAgent_ErrorDecisionEmitsNoAssistant(t *testing.T) {
	msgs := runControlAgent(t, []string{initLine, userLine(t, `{"decisions":[{"type":"error"}]}`)})
	require.Len(t, msgs, 3) // system, control_response, result (no assistant)
	assert.Equal(t, "result", msgs[2]["type"])
	for _, m := range msgs {
		assert.NotEqual(t, "assistant", m["type"])
	}
}
