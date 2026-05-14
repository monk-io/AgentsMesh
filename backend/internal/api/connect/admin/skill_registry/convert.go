package skillregistryadminconnect

import (
	"encoding/json"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/extension"
	extensionv1 "github.com/anthropics/agentsmesh/proto/gen/go/extension/v1"
)

// toProtoAdminSkillRegistry mirrors REST's inline json.Marshal of the
// GORM-backed model. Platform scope: OrganizationID is always nil so the
// field is intentionally omitted from the proto message.
//
// Timestamp policy (conventions §6): time.Time → RFC 3339 string. Nullable
// time.Time pointer → omitted when nil. Optional string fields use the
// "empty string = omitted" rule to mirror REST's `omitempty` JSON tags.
func toProtoAdminSkillRegistry(r *extension.SkillRegistry) *extensionv1.AdminSkillRegistry {
	if r == nil {
		return nil
	}
	out := &extensionv1.AdminSkillRegistry{
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
	if r.DetectedType != "" {
		v := r.DetectedType
		out.DetectedType = &v
	}
	if r.LastSyncedAt != nil {
		v := r.LastSyncedAt.UTC().Format(time.RFC3339)
		out.LastSyncedAt = &v
	}
	if r.LastCommitSha != "" {
		v := r.LastCommitSha
		out.LastCommitSha = &v
	}
	if r.SyncError != "" {
		v := r.SyncError
		out.SyncError = &v
	}
	if agents := r.GetCompatibleAgents(); len(agents) > 0 {
		out.CompatibleAgents = agents
	}
	// Defensive re-parse for the legacy "[]" raw bytes case — same logic as
	// the org-scoped converter (extension/skill_registry_convert.go:62).
	if out.CompatibleAgents == nil && len(r.CompatibleAgents) > 0 {
		var parsed []string
		if err := json.Unmarshal(r.CompatibleAgents, &parsed); err == nil {
			out.CompatibleAgents = parsed
		}
	}
	return out
}
