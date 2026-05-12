package runnerconnect

import (
	"encoding/json"
	"time"

	rundom "github.com/anthropics/agentsmesh/backend/internal/domain/runner"
	runnerlog "github.com/anthropics/agentsmesh/backend/internal/service/runnerlog"
	runner "github.com/anthropics/agentsmesh/backend/internal/service/runner"
	runnerapiv1 "github.com/anthropics/agentsmesh/proto/gen/go/runner_api/v1"
)

// toProtoRunner converts the GORM-backed domain model into the protobuf
// wire shape. host_info is serialized as JSON because the .proto carries
// it as a string field (conventions §6 rationale: avoid pulling
// google.protobuf.Struct + prost-types into the wasm graph).
func toProtoRunner(r *rundom.Runner) *runnerapiv1.Runner {
	if r == nil {
		return nil
	}
	out := &runnerapiv1.Runner{
		Id:                r.ID,
		NodeId:            r.NodeID,
		Description:       r.Description,
		Status:            r.Status,
		CurrentPods:       int32(r.CurrentPods),
		MaxConcurrentPods: int32(r.MaxConcurrentPods),
		IsEnabled:         r.IsEnabled,
		Visibility:        r.Visibility,
		AvailableAgents:   []string(r.AvailableAgents),
		Tags:              []string(r.Tags),
		CreatedAt:         r.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:         r.UpdatedAt.UTC().Format(time.RFC3339),
		OrganizationId:    r.OrganizationID,
		HostInfoJson:      marshalHostInfo(r.HostInfo),
		AgentVersions:     toProtoAgentVersions(r.AgentVersions),
	}
	if r.LastHeartbeat != nil {
		s := r.LastHeartbeat.UTC().Format(time.RFC3339)
		out.LastHeartbeat = &s
	}
	if r.RunnerVersion != nil {
		v := *r.RunnerVersion
		out.RunnerVersion = &v
	}
	if r.RegisteredByUserID != nil {
		out.RegisteredByUserId = r.RegisteredByUserID
	}
	return out
}

func toProtoAgentVersions(versions rundom.AgentVersionSlice) []*runnerapiv1.AgentVersion {
	if len(versions) == 0 {
		return nil
	}
	out := make([]*runnerapiv1.AgentVersion, 0, len(versions))
	for _, v := range versions {
		out = append(out, &runnerapiv1.AgentVersion{
			Slug:    v.Slug,
			Version: v.Version,
			Path:    v.Path,
		})
	}
	return out
}

func toProtoRunners(runners []*rundom.Runner) []*runnerapiv1.Runner {
	out := make([]*runnerapiv1.Runner, 0, len(runners))
	for _, r := range runners {
		out = append(out, toProtoRunner(r))
	}
	return out
}

func toProtoRelayConnections(connections []runner.RelayConnectionInfo) []*runnerapiv1.RelayConnectionInfo {
	if len(connections) == 0 {
		return nil
	}
	out := make([]*runnerapiv1.RelayConnectionInfo, 0, len(connections))
	for _, c := range connections {
		out = append(out, &runnerapiv1.RelayConnectionInfo{
			PodKey:      c.PodKey,
			RelayUrl:    c.RelayURL,
			SessionId:   c.SessionID,
			Connected:   c.Connected,
			ConnectedAt: c.ConnectedAt.UnixMilli(),
		})
	}
	return out
}

func toProtoSandboxStatuses(in []*runner.SandboxStatus) []*runnerapiv1.SandboxStatus {
	out := make([]*runnerapiv1.SandboxStatus, 0, len(in))
	for _, s := range in {
		if s == nil {
			continue
		}
		ps := &runnerapiv1.SandboxStatus{
			PodKey:     s.PodKey,
			Exists:     s.Exists,
			CanResume:  s.CanResume,
		}
		// Optional fields: serialize zero-values as absent so client `optional`
		// semantics line up with the REST behavior (omitempty JSON tags).
		if s.SandboxPath != "" {
			v := s.SandboxPath
			ps.SandboxPath = &v
		}
		if s.RepositoryURL != "" {
			v := s.RepositoryURL
			ps.RepositoryUrl = &v
		}
		if s.BranchName != "" {
			v := s.BranchName
			ps.BranchName = &v
		}
		if s.CurrentCommit != "" {
			v := s.CurrentCommit
			ps.CurrentCommit = &v
		}
		if s.SizeBytes != 0 {
			v := s.SizeBytes
			ps.SizeBytes = &v
		}
		if s.LastModified != 0 {
			v := s.LastModified
			ps.LastModified = &v
		}
		if s.HasUncommittedChanges {
			v := s.HasUncommittedChanges
			ps.HasUncommittedChanges = &v
		}
		if s.Error != "" {
			v := s.Error
			ps.Error = &v
		}
		out = append(out, ps)
	}
	return out
}

// toProtoLogEntry serializes the runnerlog.LogEntry shape (the REST handler's
// JSON shape) into the proto wire shape.
func toProtoLogEntry(e *runnerlog.LogEntry) *runnerapiv1.RunnerLog {
	if e == nil || e.RunnerLog == nil {
		return nil
	}
	out := &runnerapiv1.RunnerLog{
		Id:            e.RunnerLog.ID,
		RunnerId:      e.RunnerLog.RunnerID,
		RequestId:     e.RunnerLog.RequestID,
		Status:        e.RunnerLog.Status,
		SizeBytes:     e.RunnerLog.SizeBytes,
		RequestedById: e.RunnerLog.RequestedByID,
	}
	if e.RunnerLog.StorageKey != "" {
		v := e.RunnerLog.StorageKey
		out.StorageKey = &v
	}
	if e.RunnerLog.ErrorMessage != "" {
		v := e.RunnerLog.ErrorMessage
		out.ErrorMessage = &v
	}
	if e.DownloadURL != "" {
		v := e.DownloadURL
		out.DownloadUrl = &v
	}
	if !e.RunnerLog.CreatedAt.IsZero() {
		v := e.RunnerLog.CreatedAt.UTC().Format(time.RFC3339)
		out.CreatedAt = &v
	}
	if e.RunnerLog.CompletedAt != nil {
		v := e.RunnerLog.CompletedAt.UTC().Format(time.RFC3339)
		out.CompletedAt = &v
	}
	return out
}

func toProtoToken(t rundom.GRPCRegistrationToken) *runnerapiv1.RunnerToken {
	out := &runnerapiv1.RunnerToken{
		Id: t.ID,
	}
	if t.Name != nil {
		out.Name = t.Name
	}
	maxUses := int32(t.MaxUses)
	usedCount := int32(t.UsedCount)
	out.MaxUses = &maxUses
	out.UsedCount = &usedCount
	if !t.ExpiresAt.IsZero() {
		v := t.ExpiresAt.UTC().Format(time.RFC3339)
		out.ExpiresAt = &v
	}
	if !t.CreatedAt.IsZero() {
		v := t.CreatedAt.UTC().Format(time.RFC3339)
		out.CreatedAt = &v
	}
	return out
}

func marshalHostInfo(hi rundom.HostInfo) string {
	if hi == nil {
		return ""
	}
	b, err := json.Marshal(hi)
	if err != nil {
		return ""
	}
	return string(b)
}
