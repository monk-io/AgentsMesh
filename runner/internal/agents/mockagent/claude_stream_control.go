package mockagent

import (
	"bufio"
	"encoding/json"
	"io"
	"log/slog"
	"os"
)

const mockControlSessionID = "mock-control-001"

// RunControlAgent drives the claude-stream control-agent runtime. Autopilot's
// AcpControlProcess speaks claude's NDJSON dialect (not standard ACP JSON-RPC),
// so this is a separate runtime from RunACP. It handshakes, then for each
// `user` prompt asks the decision engine for the next decision (which observes
// and drives the target pod via MCP) and emits it as an assistant message +
// result — exactly the wire shape the real claude CLI produces.
func RunControlAgent(mcpConfigPath string, logger *slog.Logger) int {
	return runControlAgentWithIO(mcpConfigPath, os.Stdin, os.Stdout, logger)
}

func runControlAgentWithIO(mcpConfigPath string, in io.Reader, out io.Writer, logger *slog.Logger) int {
	engine := newControlDecisionEngine(mcpConfigPath, logger)
	w := &ndjsonWriter{out: out}

	scanner := bufio.NewScanner(in)
	scanner.Buffer(make([]byte, 0, 1024*1024), 10*1024*1024)
	for scanner.Scan() {
		var msg struct {
			Type      string `json:"type"`
			RequestID string `json:"request_id"`
			Message   struct {
				Content string `json:"content"`
			} `json:"message"`
		}
		if err := json.Unmarshal(scanner.Bytes(), &msg); err != nil {
			continue
		}
		switch msg.Type {
		case "control_request":
			// Handshake: system/init seeds the session id; a control_response
			// whose request_id isn't a tracked outgoing request closes the
			// runner's initCh (see claude/handler.go handleControlResponse).
			w.write(map[string]any{"type": "system", "subtype": "init", "session_id": mockControlSessionID})
			w.write(map[string]any{
				"type":     "control_response",
				"response": map[string]any{"subtype": "success", "request_id": msg.RequestID},
			})
		case "user":
			decision := engine.next(msg.Message.Content)
			if decision != "" {
				w.write(assistantTextMessage(decision))
			}
			w.write(map[string]any{"type": "result", "subtype": "success"})
		}
	}
	return 0
}

type ndjsonWriter struct{ out io.Writer }

func (n *ndjsonWriter) write(v any) {
	b, err := json.Marshal(v)
	if err != nil {
		return
	}
	_, _ = n.out.Write(append(b, '\n'))
}

func assistantTextMessage(text string) map[string]any {
	return map[string]any{
		"type": "assistant",
		"message": map[string]any{
			"content": []map[string]any{{"type": "text", "text": text}},
		},
	}
}
