package agentpodsettingsconnect

import (
	"time"

	poddom "github.com/anthropics/agentsmesh/backend/internal/domain/agentpod"
	podv1 "github.com/anthropics/agentsmesh/proto/gen/go/pod/v1"
)

func toProtoSettings(s *poddom.UserAgentPodSettings) *podv1.AgentPodSettings {
	if s == nil {
		return &podv1.AgentPodSettings{}
	}
	out := &podv1.AgentPodSettings{
		DefaultAgentSlug: s.DefaultAgentSlug,
		DefaultModel:     s.DefaultModel,
		DefaultPermMode:  s.DefaultPermMode,
		TerminalTheme:    s.TerminalTheme,
	}
	if s.TerminalFontSize != nil {
		v := int32(*s.TerminalFontSize)
		out.TerminalFontSize = &v
	}
	return out
}

// toProtoProvider converts UserAIProvider — encrypted credentials NEVER leave
// the server (mirrors v1/agentpod.go:91-93 which scrubs them post-fetch).
func toProtoProvider(p *poddom.UserAIProvider) *podv1.AIProvider {
	if p == nil {
		return nil
	}
	return &podv1.AIProvider{
		Id:           p.ID,
		ProviderType: p.ProviderType,
		Name:         p.Name,
		IsDefault:    p.IsDefault,
		IsEnabled:    p.IsEnabled,
		CreatedAt:    p.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:    p.UpdatedAt.UTC().Format(time.RFC3339),
	}
}
