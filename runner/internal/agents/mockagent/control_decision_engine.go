package mockagent

import (
	"encoding/json"
	"log/slog"
)

// controlScript is the decision program the e2e test injects via the autopilot
// controller's control_prompt_template. The control agent receives it as the
// first prompt, parses it once, then replays one decision per iteration.
type controlScript struct {
	Observe   bool              `json:"observe"`
	Decisions []controlDecision `json:"decisions"`
}

type controlDecision struct {
	Type      string `json:"type"`       // continue | completed | need_help | give_up | error
	Reasoning string `json:"reasoning"`
	SendInput string `json:"send_input"` // optional: text to drive the target pod this round
}

// controlDecisionEngine turns the injected script into the structured decision
// JSON autopilot's DecisionParser expects, observing/driving the target pod via
// MCP along the way. It is a long-lived per-process state machine (the control
// agent is a persistent claude-stream session), so a plain iteration counter
// is sufficient.
type controlDecisionEngine struct {
	mcp    *mcpClient
	script controlScript
	parsed bool
	iter   int
	logger *slog.Logger
}

func newControlDecisionEngine(mcpConfigPath string, logger *slog.Logger) *controlDecisionEngine {
	return &controlDecisionEngine{
		mcp:    newMCPClientFromConfig(mcpConfigPath),
		logger: logger,
	}
}

// next parses the script from the first prompt, then for each call observes +
// drives the target pod via MCP and returns the structured decision JSON.
func (e *controlDecisionEngine) next(prompt string) string {
	if !e.parsed {
		if err := json.Unmarshal([]byte(prompt), &e.script); err != nil {
			e.logger.Warn("control script parse failed; defaulting to continue", "error", err)
		}
		e.parsed = true
	}

	d := e.decisionForIteration()
	e.iter++

	// "error" produces empty output, which RunControlProcess treats as a failed
	// decision — drives the controller's consecutive-error circuit breaker.
	if d.Type == "error" {
		return ""
	}

	if e.mcp != nil {
		if e.script.Observe {
			if _, err := e.mcp.getPodSnapshot(50); err != nil {
				e.logger.Warn("get_pod_snapshot failed", "error", err)
			}
			if _, err := e.mcp.getPodStatus(); err != nil {
				e.logger.Warn("get_pod_status failed", "error", err)
			}
		}
		if d.SendInput != "" {
			if _, err := e.mcp.sendPodInput(d.SendInput); err != nil {
				e.logger.Warn("send_pod_input failed", "error", err)
			}
		}
	}

	return buildDecisionJSON(d)
}

func (e *controlDecisionEngine) decisionForIteration() controlDecision {
	if e.iter < len(e.script.Decisions) {
		return e.script.Decisions[e.iter]
	}
	// Script exhausted → keep continuing so max_iterations can terminate us.
	return controlDecision{Type: "continue", Reasoning: "script exhausted; continuing"}
}

func buildDecisionJSON(d controlDecision) string {
	typ := d.Type
	if typ == "" {
		typ = "continue"
	}
	out := map[string]any{
		"decision": map[string]any{
			"type":       typ,
			"confidence": 1.0,
			"reasoning":  d.Reasoning,
		},
	}
	if d.SendInput != "" {
		out["action"] = map[string]any{
			"type":    "send_input",
			"content": d.SendInput,
			"reason":  d.Reasoning,
		}
	}
	b, _ := json.Marshal(out)
	return string(b)
}
