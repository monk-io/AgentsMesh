package usercredentialconnect

import (
	"context"
	"errors"
	"time"

	"connectrpc.com/connect"

	domainuser "github.com/anthropics/agentsmesh/backend/internal/domain/user"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	ucv1 "github.com/anthropics/agentsmesh/proto/gen/go/user_credential/v1"
)

// requireUserID is the user-scoped equivalent of interceptors.ResolveOrgScope.
// Returns CodeUnauthenticated if the auth interceptor didn't populate UserID
// — mirrors what AuthMiddleware does for REST and matches conventions §3.5.
func requireUserID(ctx context.Context) (int64, error) {
	tenant := middleware.GetTenant(ctx)
	if tenant == nil || tenant.UserID == 0 {
		return 0, connect.NewError(connect.CodeUnauthenticated, errors.New("authentication required"))
	}
	return tenant.UserID, nil
}

// toProtoGitCredential mirrors domainuser.GitCredential.ToResponse().
// SENSITIVE fields (PATEncrypted, PrivateKeyEncrypted) are intentionally
// omitted — the GORM `json:"-"` tag is the REST equivalent.
func toProtoGitCredential(c *domainuser.GitCredential) *ucv1.GitCredential {
	if c == nil {
		return nil
	}
	out := &ucv1.GitCredential{
		Id:                   c.ID,
		Name:                 c.Name,
		CredentialType:       c.CredentialType,
		RepositoryProviderId: c.RepositoryProviderID,
		IsDefault:            c.IsDefault,
		CreatedAt:            c.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:            c.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if c.RepositoryProvider != nil {
		name := c.RepositoryProvider.Name
		out.ProviderName = &name
	}
	if c.PublicKey != nil {
		v := *c.PublicKey
		out.PublicKey = &v
	}
	if c.Fingerprint != nil {
		v := *c.Fingerprint
		out.Fingerprint = &v
	}
	if c.HostPattern != nil {
		v := *c.HostPattern
		out.HostPattern = &v
	}
	return out
}

// virtualRunnerLocalCredential mirrors REST's RunnerLocalCredentialResponse —
// a zero-id placeholder always returned in lists (user_git_credentials.go:67).
// Used here for list response item composition.
func virtualRunnerLocalCredential(isDefault bool) *ucv1.GitCredential {
	return &ucv1.GitCredential{
		Id:             0,
		Name:           "Runner Local",
		CredentialType: domainuser.CredentialTypeRunnerLocal,
		IsDefault:      isDefault,
		CreatedAt:      "",
		UpdatedAt:      "",
	}
}

// toProtoRepositoryProvider mirrors domainuser.RepositoryProvider.ToResponse().
// SENSITIVE fields (ClientSecretEncrypted, BotTokenEncrypted) are
// intentionally absent — `has_client_id` / `has_bot_token` boolean flags
// signal whether the secret is configured without leaking the value.
func toProtoRepositoryProvider(p *domainuser.RepositoryProvider) *ucv1.RepositoryProvider {
	if p == nil {
		return nil
	}
	hasIdentity := p.IdentityID != nil &&
		p.Identity != nil &&
		p.Identity.AccessTokenEncrypted != nil &&
		*p.Identity.AccessTokenEncrypted != ""
	return &ucv1.RepositoryProvider{
		Id:           p.ID,
		ProviderType: p.ProviderType,
		Name:         p.Name,
		BaseUrl:      p.BaseURL,
		HasClientId:  p.ClientID != nil && *p.ClientID != "",
		HasBotToken:  p.BotTokenEncrypted != nil && *p.BotTokenEncrypted != "",
		HasIdentity:  hasIdentity,
		IsDefault:    p.IsDefault,
		IsActive:     p.IsActive,
		CreatedAt:    p.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:    p.UpdatedAt.UTC().Format(time.RFC3339),
	}
}
