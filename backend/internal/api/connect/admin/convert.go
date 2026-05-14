package adminconnect

import (
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/organization"
	"github.com/anthropics/agentsmesh/backend/internal/domain/user"
	adminv1 "github.com/anthropics/agentsmesh/proto/gen/go/admin/v1"
)

func toProtoAdminUser(u *user.User) *adminv1.AdminUser {
	if u == nil {
		return nil
	}
	out := &adminv1.AdminUser{
		Id:              u.ID,
		Email:           u.Email,
		Username:        u.Username,
		IsActive:        u.IsActive,
		IsSystemAdmin:   u.IsSystemAdmin,
		IsEmailVerified: u.IsEmailVerified,
		CreatedAt:       u.CreatedAt.Format(time.RFC3339),
		UpdatedAt:       u.UpdatedAt.Format(time.RFC3339),
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
		v := u.LastLoginAt.Format(time.RFC3339)
		out.LastLoginAt = &v
	}
	return out
}

func toProtoAdminOrganization(o *organization.Organization) *adminv1.AdminOrganization {
	if o == nil {
		return nil
	}
	out := &adminv1.AdminOrganization{
		Id:                 o.ID,
		Name:               o.Name,
		Slug:               o.Slug,
		SubscriptionPlan:   o.SubscriptionPlan,
		SubscriptionStatus: o.SubscriptionStatus,
		CreatedAt:          o.CreatedAt.Format(time.RFC3339),
		UpdatedAt:          o.UpdatedAt.Format(time.RFC3339),
	}
	if o.LogoURL != nil {
		v := *o.LogoURL
		out.LogoUrl = &v
	}
	return out
}

func toProtoAdminOrganizationMember(m *organization.Member) *adminv1.AdminOrganizationMember {
	out := &adminv1.AdminOrganizationMember{
		Id:       m.ID,
		UserId:   m.UserID,
		OrgId:    m.OrganizationID,
		Role:     m.Role,
		JoinedAt: m.JoinedAt.Format(time.RFC3339),
	}
	if m.User != nil {
		summary := &adminv1.AdminUserSummary{
			Id:       m.User.ID,
			Email:    m.User.Email,
			Username: m.User.Username,
		}
		if m.User.Name != nil {
			v := *m.User.Name
			summary.Name = &v
		}
		if m.User.AvatarURL != nil {
			v := *m.User.AvatarURL
			summary.AvatarUrl = &v
		}
		out.User = summary
	}
	return out
}
