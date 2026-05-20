package v1

import (
	"github.com/anthropics/agentsmesh/backend/internal/service/agent"
)

// AgentHandler handles agent-related requests
type AgentHandler struct {
	agentSvc      *agent.AgentService
	userConfigSvc *agent.UserConfigService
}

// NewAgentHandler creates a new agent handler
func NewAgentHandler(
	agentSvc *agent.AgentService,
	userConfigSvc *agent.UserConfigService,
) *AgentHandler {
	return &AgentHandler{
		agentSvc:      agentSvc,
		userConfigSvc: userConfigSvc,
	}
}

// CreateCustomAgentRequest represents custom agent creation request.
// When AgentfileSource is provided, LaunchCommand becomes optional (extracted from AgentFile).
type CreateCustomAgentRequest struct {
	// Slug format is enforced by slugkit.Validate in CreateCustomAgent
	// (handler entry). Drop the `alphanum` binding tag — it rejects
	// hyphens which slugkit permits as the canonical word separator.
	Slug            string `json:"slug" binding:"required,min=2,max=50"`
	Name            string `json:"name" binding:"required,min=2,max=100"`
	Description     string `json:"description"`
	AgentfileSource string `json:"agentfile_source"`
	LaunchCommand   string `json:"launch_command"`
	DefaultArgs     string `json:"default_args"`
}

// SetUserAgentConfigRequest represents a request to set user's personal config
type SetUserAgentConfigRequest struct {
	ConfigValues map[string]interface{} `json:"config_values" binding:"required"`
}
