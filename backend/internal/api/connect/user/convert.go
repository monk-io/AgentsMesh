package userconnect

import (
	"context"
	"errors"
	"time"

	"connectrpc.com/connect"

	domainuser "github.com/anthropics/agentsmesh/backend/internal/domain/user"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	userservice "github.com/anthropics/agentsmesh/backend/internal/service/user"
	userv1 "github.com/anthropics/agentsmesh/proto/gen/go/user/v1"
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

// toProtoUser mirrors the REST `c.JSON(http.StatusOK, gin.H{"user": u})`
// shape, with the password_hash and verification/reset token fields
// omitted (mirrors `json:"-"` on the GORM struct).
func toProtoUser(u *domainuser.User) *userv1.User {
	if u == nil {
		return nil
	}
	out := &userv1.User{
		Id:                u.ID,
		Email:             u.Email,
		Username:          u.Username,
		IsActive:          u.IsActive,
		IsSystemAdmin:     u.IsSystemAdmin,
		IsEmailVerified:   u.IsEmailVerified,
		CreatedAt:         u.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:         u.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if u.Name != nil {
		v := *u.Name
		out.Name = &v
	}
	if u.AvatarURL != nil {
		v := *u.AvatarURL
		out.AvatarUrl = &v
	}
	if u.LastLoginAt != nil {
		v := u.LastLoginAt.UTC().Format(time.RFC3339)
		out.LastLoginAt = &v
	}
	if u.DefaultGitCredentialID != nil {
		v := *u.DefaultGitCredentialID
		out.DefaultGitCredentialId = &v
	}
	return out
}

// toProtoUserSummary is the search-result shape (subset of toProtoUser
// with sensitive / privileged fields dropped).
func toProtoUserSummary(u *domainuser.User) *userv1.UserSummary {
	if u == nil {
		return nil
	}
	out := &userv1.UserSummary{
		Id:       u.ID,
		Email:    u.Email,
		Username: u.Username,
	}
	if u.Name != nil {
		v := *u.Name
		out.Name = &v
	}
	if u.AvatarURL != nil {
		v := *u.AvatarURL
		out.AvatarUrl = &v
	}
	return out
}

// toProtoIdentity mirrors domainuser.Identity. SENSITIVE fields
// (access_token_encrypted, refresh_token_encrypted) are intentionally
// omitted — the REST handler does the same scrub (users.go:144).
func toProtoIdentity(i *domainuser.Identity) *userv1.Identity {
	if i == nil {
		return nil
	}
	out := &userv1.Identity{
		Id:             i.ID,
		UserId:         i.UserID,
		Provider:       i.Provider,
		ProviderUserId: i.ProviderUserID,
		CreatedAt:      i.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:      i.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if i.ProviderUsername != nil {
		v := *i.ProviderUsername
		out.ProviderUsername = &v
	}
	if i.TokenExpiresAt != nil {
		v := i.TokenExpiresAt.UTC().Format(time.RFC3339)
		out.TokenExpiresAt = &v
	}
	return out
}

// mapUserServiceError translates user-service sentinels to Connect codes
// per conventions §10.
func mapUserServiceError(err error) error {
	switch {
	case errors.Is(err, userservice.ErrUserNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	case errors.Is(err, userservice.ErrEmailAlreadyExists),
		errors.Is(err, userservice.ErrUsernameExists):
		return connect.NewError(connect.CodeAlreadyExists, err)
	case errors.Is(err, userservice.ErrInvalidCredentials):
		return connect.NewError(connect.CodeUnauthenticated, err)
	case errors.Is(err, userservice.ErrUserInactive):
		return connect.NewError(connect.CodePermissionDenied, err)
	default:
		return connect.NewError(connect.CodeInternal, err)
	}
}
