package licenseconnect

import (
	"github.com/anthropics/agentsmesh/backend/internal/domain/billing"
	licenseservice "github.com/anthropics/agentsmesh/backend/internal/service/license"
	licensev1 "github.com/anthropics/agentsmesh/proto/gen/go/license/v1"
)

// toProtoStatus converts the domain status struct (billing.LicenseStatus —
// the same struct REST returned from /status) into the proto wire shape.
// Field set + tag numbers locked by license_proto.rs; reviewer's first
// check is the field-count diff vs the .proto.
//
// Timestamp policy (conventions §6): time.Time pointer → optional ISO-8601
// string, omitted when nil.
func toProtoStatus(s *billing.LicenseStatus) *licensev1.LicenseStatus {
	if s == nil {
		return &licensev1.LicenseStatus{}
	}
	out := &licensev1.LicenseStatus{
		IsActive:         s.IsActive,
		LicenseKey:       s.LicenseKey,
		OrganizationName: s.OrganizationName,
		Plan:             s.Plan,
		MaxUsers:         int32(s.MaxUsers),
		MaxRunners:       int32(s.MaxRunners),
		MaxRepositories:  int32(s.MaxRepositories),
		MaxPodMinutes:    int32(s.MaxPodMinutes),
		Features:         s.Features,
		Message:          s.Message,
	}
	if s.ExpiresAt != nil {
		ts := s.ExpiresAt.UTC().Format(rfc3339)
		out.ExpiresAt = &ts
	}
	return out
}

// toProtoLimits converts the service-layer LicenseLimits (a separate type
// from billing.LicenseStatus because limits travel with the parsed
// preview, not the activated record). The two share field names; the
// conversion is straight numeric widening.
func toProtoLimits(l licenseservice.LicenseLimits) *licensev1.LicenseLimits {
	return &licensev1.LicenseLimits{
		MaxUsers:        int32(l.MaxUsers),
		MaxRunners:      int32(l.MaxRunners),
		MaxRepositories: int32(l.MaxRepositories),
		MaxPodMinutes:   int32(l.MaxPodMinutes),
	}
}

// toProtoValidated converts the parsed-but-not-activated license preview
// returned by ParseAndVerify into the proto wire shape. ValidatedLicense
// is intentionally different from LicenseStatus — preview shows raw
// plan / limits / features, status shows the activation flags.
func toProtoValidated(l *licenseservice.LicenseData) *licensev1.ValidatedLicense {
	if l == nil {
		return &licensev1.ValidatedLicense{Valid: false}
	}
	return &licensev1.ValidatedLicense{
		Valid:            true,
		LicenseKey:       l.LicenseKey,
		OrganizationName: l.OrganizationName,
		ContactEmail:     l.ContactEmail,
		Plan:             l.Plan,
		Limits:           toProtoLimits(l.Limits),
		Features:         l.Features,
		IssuedAt:         l.IssuedAt.UTC().Format(rfc3339),
		ExpiresAt:        l.ExpiresAt.UTC().Format(rfc3339),
	}
}

// rfc3339 mirrors time.RFC3339. Hoisted to package-level constant so the
// timestamp format is one place if we ever drift to a sub-second variant.
const rfc3339 = "2006-01-02T15:04:05Z07:00"
