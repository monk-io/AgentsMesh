package extensionconnect

import (
	"encoding/json"
	"time"

	extdom "github.com/anthropics/agentsmesh/backend/internal/domain/extension"
	extensionv1 "github.com/anthropics/agentsmesh/proto/gen/go/extension/v1"
)

// toProtoSkillRegistry converts the GORM-backed domain model into the
// protobuf wire shape. Fields kept in lockstep with the .proto definition
// — every reviewer's first check is the field-count + name diff (watch
// list §6 / §8).
//
// Timestamp policy (conventions §6): time.Time → RFC 3339 string. Nullable
// time.Time pointer → omitted when nil (protobuf optional encodes "no tag
// present"). Same applies to organization_id and other optional fields.
func toProtoSkillRegistry(r *extdom.SkillRegistry) *extensionv1.SkillRegistry {
	if r == nil {
		return nil
	}
	out := &extensionv1.SkillRegistry{
		Id:            r.ID,
		RepositoryUrl: r.RepositoryURL,
		Branch:        r.Branch,
		SourceType:    r.SourceType,
		AuthType:      r.AuthType,
		SyncStatus:    r.SyncStatus,
		SkillCount:    int32(r.SkillCount),
		IsActive:      r.IsActive,
		CreatedAt:     r.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:     r.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if r.OrganizationID != nil {
		out.OrganizationId = r.OrganizationID
	}
	if r.DetectedType != "" {
		dt := r.DetectedType
		out.DetectedType = &dt
	}
	if r.LastSyncedAt != nil {
		s := r.LastSyncedAt.UTC().Format(time.RFC3339)
		out.LastSyncedAt = &s
	}
	if r.LastCommitSha != "" {
		s := r.LastCommitSha
		out.LastCommitSha = &s
	}
	if r.SyncError != "" {
		s := r.SyncError
		out.SyncError = &s
	}
	// compatible_agents is stored as JSON in the DB; the domain helper
	// returns nil for "all agents allowed" (the catch-all default).
	if agents := r.GetCompatibleAgents(); len(agents) > 0 {
		out.CompatibleAgents = agents
	}
	// If GetCompatibleAgents returned nil but the raw bytes look like a
	// valid JSON array we shouldn't lose info — re-parse defensively.
	// (Handles the legacy case where DB has '[]' explicitly.)
	if out.CompatibleAgents == nil && len(r.CompatibleAgents) > 0 {
		var parsed []string
		if err := json.Unmarshal(r.CompatibleAgents, &parsed); err == nil {
			out.CompatibleAgents = parsed
		}
	}
	return out
}

func toProtoSkillRegistryOverride(o *extdom.SkillRegistryOverride) *extensionv1.SkillRegistryOverride {
	if o == nil {
		return nil
	}
	return &extensionv1.SkillRegistryOverride{
		Id:             o.ID,
		OrganizationId: o.OrganizationID,
		RegistryId:     o.RegistryID,
		IsDisabled:     o.IsDisabled,
		CreatedAt:      o.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:      o.UpdatedAt.UTC().Format(time.RFC3339),
	}
}
