package agentconnect

import (
	"context"

	domainagent "github.com/anthropics/agentsmesh/backend/internal/domain/agent"
	agentservice "github.com/anthropics/agentsmesh/backend/internal/service/agent"
)

// compositeProvider mirrors REST's compositeProvider (agents.go:36).
// Combines agentSvc + credentialSvc to satisfy AgentConfigProvider, which
// the ConfigBuilder consumes for schema extraction.
type compositeProvider struct {
	agentSvc      *agentservice.AgentService
	credentialSvc *agentservice.CredentialProfileService
}

func (p *compositeProvider) GetAgent(ctx context.Context, slug string) (*domainagent.Agent, error) {
	return p.agentSvc.GetAgent(ctx, slug)
}

func (p *compositeProvider) GetEffectiveCredentialsForPod(
	ctx context.Context, userID int64, agentSlug string, profileID *int64,
) (domainagent.EncryptedCredentials, bool, error) {
	return p.credentialSvc.GetEffectiveCredentialsForPod(ctx, userID, agentSlug, profileID)
}

func (p *compositeProvider) ResolveCredentialsByName(
	ctx context.Context, userID int64, agentSlug, profileName string,
) (domainagent.EncryptedCredentials, bool, error) {
	return p.credentialSvc.ResolveCredentialsByName(ctx, userID, agentSlug, profileName)
}
