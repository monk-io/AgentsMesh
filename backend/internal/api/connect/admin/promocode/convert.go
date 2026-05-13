package promocodeadminconnect

import (
	"strings"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/organization"
	"github.com/anthropics/agentsmesh/backend/internal/domain/promocode"
	"github.com/anthropics/agentsmesh/backend/internal/domain/user"
	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
	promocodev1 "github.com/anthropics/agentsmesh/proto/gen/go/promocode/v1"
)

func toProtoPromoCode(p *promocode.PromoCode) *promocodev1.PromoCode {
	if p == nil {
		return nil
	}
	out := &promocodev1.PromoCode{
		Id:             p.ID,
		Code:           p.Code,
		Name:           p.Name,
		Description:    p.Description,
		Type:           string(p.Type),
		PlanName:       p.PlanName,
		DurationMonths: int32(p.DurationMonths),
		UsedCount:      int32(p.UsedCount),
		MaxUsesPerOrg:  int32(p.MaxUsesPerOrg),
		StartsAt:       p.StartsAt.Format(time.RFC3339),
		IsActive:       p.IsActive,
		CreatedAt:      p.CreatedAt.Format(time.RFC3339),
		UpdatedAt:      p.UpdatedAt.Format(time.RFC3339),
	}
	if p.MaxUses != nil {
		v := int32(*p.MaxUses)
		out.MaxUses = &v
	}
	if p.ExpiresAt != nil {
		s := p.ExpiresAt.Format(time.RFC3339)
		out.ExpiresAt = &s
	}
	if p.CreatedByID != nil {
		out.CreatedById = p.CreatedByID
	}
	return out
}

func toProtoRedemptionDetail(d *adminservice.RedemptionWithDetails) *promocodev1.RedemptionDetail {
	out := &promocodev1.RedemptionDetail{
		Id:             d.ID,
		PromoCodeId:    d.PromoCodeID,
		OrganizationId: d.OrganizationID,
		UserId:         d.UserID,
		PlanName:       d.PlanName,
		DurationMonths: int32(d.DurationMonths),
		NewPeriodEnd:   d.NewPeriodEnd.Format(time.RFC3339),
		IpAddress:      d.IPAddress,
		CreatedAt:      d.CreatedAt.Format(time.RFC3339),
	}
	if u := d.User; u != nil {
		setUserFields(out, u)
	}
	if o := d.Organization; o != nil {
		setOrgFields(out, o)
	}
	return out
}

func setUserFields(out *promocodev1.RedemptionDetail, u *user.User) {
	email := u.Email
	out.UserEmail = &email
	username := u.Username
	out.UserUsername = &username
}

func setOrgFields(out *promocodev1.RedemptionDetail, o *organization.Organization) {
	name := o.Name
	out.OrganizationName = &name
	slug := o.Slug
	out.OrganizationSlug = &slug
}

// parseTime parses an RFC3339 string, returning zero time + nil err for empty
// input so callers can opt into "use server-now" semantics by omitting the
// field.
func parseTime(s string) (time.Time, error) {
	if s == "" {
		return time.Time{}, nil
	}
	return time.Parse(time.RFC3339, s)
}

// normalizeCode mirrors REST's strings.ToUpper(req.Code) at promo_codes.go:131.
func normalizeCode(s string) string { return strings.ToUpper(s) }
