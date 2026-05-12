package invitationconnect

import (
	"time"

	invitationdomain "github.com/anthropics/agentsmesh/backend/internal/domain/invitation"
	invitationsvc "github.com/anthropics/agentsmesh/backend/internal/service/invitation"
	invitationv1 "github.com/anthropics/agentsmesh/proto/gen/go/invitation/v1"
)

// toProtoInvitation converts the GORM-backed domain model into the wire
// shape used by the org-scoped list. Token is intentionally absent — only
// the invitee receives it via email, never via list APIs.
func toProtoInvitation(inv *invitationdomain.Invitation) *invitationv1.Invitation {
	if inv == nil {
		return nil
	}
	out := &invitationv1.Invitation{
		Id:             inv.ID,
		OrganizationId: inv.OrganizationID,
		Email:          inv.Email,
		Role:           inv.Role,
		InvitedBy:      inv.InvitedBy,
		ExpiresAt:      inv.ExpiresAt.UTC().Format(time.RFC3339),
		CreatedAt:      inv.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:      inv.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if inv.AcceptedAt != nil {
		v := inv.AcceptedAt.UTC().Format(time.RFC3339)
		out.AcceptedAt = &v
	}
	return out
}

// toProtoInvitationInfo converts the service-layer InvitationInfo (which
// already joined org + inviter + computed is_expired) to wire shape.
func toProtoInvitationInfo(info *invitationsvc.InvitationInfo) *invitationv1.InvitationInfo {
	if info == nil {
		return nil
	}
	return &invitationv1.InvitationInfo{
		Id:                info.ID,
		Email:             info.Email,
		Role:              info.Role,
		OrganizationId:    info.OrgID,
		OrganizationName:  info.OrgName,
		OrganizationSlug:  info.OrgSlug,
		InviterName:       info.InviterName,
		ExpiresAt:         info.ExpiresAt.UTC().Format(time.RFC3339),
		IsExpired:         info.IsExpired,
	}
}

// toProtoAcceptedOrg mirrors service.AcceptOrgInfo onto the wire.
func toProtoAcceptedOrg(o *invitationsvc.AcceptOrgInfo) *invitationv1.AcceptedOrgInfo {
	if o == nil {
		return nil
	}
	return &invitationv1.AcceptedOrgInfo{
		Id:   o.ID,
		Name: o.Name,
		Slug: o.Slug,
	}
}
