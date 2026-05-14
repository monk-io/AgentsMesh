package adminconnect

import (
	"encoding/json"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/organization"
	"github.com/anthropics/agentsmesh/backend/internal/domain/runner"
	adminv1 "github.com/anthropics/agentsmesh/proto/gen/go/admin/v1"
)

// toProtoAdminRunner mirrors REST `runnerResponse` (runners.go:113) while
// serializing `HostInfo` (map[string]any) as JSON. Proto stays binary-safe
// by exposing the host metadata as a single string field — wire parity with
// the JSON envelope REST emitted is preserved.
func toProtoAdminRunner(r *runner.Runner, org *organization.Organization) *adminv1.AdminRunner {
	if r == nil {
		return nil
	}
	out := &adminv1.AdminRunner{
		Id:                r.ID,
		OrganizationId:    r.OrganizationID,
		NodeId:            r.NodeID,
		Status:            r.Status,
		IsEnabled:         r.IsEnabled,
		CurrentPods:       int32(r.CurrentPods),
		MaxConcurrentPods: int32(r.MaxConcurrentPods),
		AvailableAgents:   []string(r.AvailableAgents),
		CreatedAt:         r.CreatedAt.Format(time.RFC3339),
		UpdatedAt:         r.UpdatedAt.Format(time.RFC3339),
	}
	if r.Description != "" {
		v := r.Description
		out.Description = &v
	}
	if r.RunnerVersion != nil {
		v := *r.RunnerVersion
		out.RunnerVersion = &v
	}
	if r.LastHeartbeat != nil {
		v := r.LastHeartbeat.Format(time.RFC3339)
		out.LastHeartbeat = &v
	}
	if len(r.HostInfo) > 0 {
		if buf, err := json.Marshal(r.HostInfo); err == nil {
			v := string(buf)
			out.HostInfoJson = &v
		}
	}
	if org != nil {
		out.Organization = &adminv1.AdminOrganizationSummary{
			Id:   org.ID,
			Name: org.Name,
			Slug: org.Slug,
		}
	}
	return out
}
