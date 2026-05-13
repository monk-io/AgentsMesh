package meshconnect

import (
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/agentpod"
	domainmesh "github.com/anthropics/agentsmesh/backend/internal/domain/mesh"
	meshv1 "github.com/anthropics/agentsmesh/proto/gen/go/mesh/v1"
)

// toProtoMeshNode converts the GORM-backed domain projection to its
// protobuf wire shape. Fields kept in lockstep with the .proto definition.
//
// Timestamp policy (conventions §6): *time.Time → optional RFC 3339 string;
// omitted when nil. agent_slug stays empty when the projection didn't pull
// it (REST never populated it either — the field is on Rust types only
// for renderer display).
//
// nil-pointer scalars on the domain side map to absent on the wire, which
// proto3 `optional` represents as `Option<T>` on Rust and `Optional<T>` on
// generated TS. The renderer surface (TS interface in stores/mesh.ts) was
// already optional everywhere, so this matches the existing read shape.
func toProtoMeshNode(n domainmesh.MeshNode) *meshv1.MeshNode {
	out := &meshv1.MeshNode{
		PodKey:       n.PodKey,
		Status:       n.Status,
		AgentStatus:  n.AgentStatus,
		CreatedById:  n.CreatedByID,
		RunnerId:     n.RunnerID,
		RunnerNodeId: n.RunnerNodeID,
		RunnerStatus: n.RunnerStatus,
	}
	if n.Model != nil {
		out.Model = n.Model
	}
	if n.Title != nil {
		out.Title = n.Title
	}
	if n.Alias != nil {
		out.Alias = n.Alias
	}
	if n.TicketID != nil {
		out.TicketId = n.TicketID
	}
	if n.TicketSlug != nil {
		out.TicketSlug = n.TicketSlug
	}
	if n.TicketTitle != nil {
		out.TicketTitle = n.TicketTitle
	}
	if n.RepositoryID != nil {
		out.RepositoryId = n.RepositoryID
	}
	if n.StartedAt != nil {
		s := n.StartedAt.UTC().Format(time.RFC3339)
		out.StartedAt = &s
	}
	return out
}

func toProtoMeshEdge(e domainmesh.MeshEdge) *meshv1.MeshEdge {
	return &meshv1.MeshEdge{
		Id:            e.ID,
		Source:        e.Source,
		Target:        e.Target,
		GrantedScopes: e.GrantedScopes,
		PendingScopes: e.PendingScopes,
		Status:        e.Status,
	}
}

func toProtoChannelInfo(c domainmesh.ChannelInfo) *meshv1.ChannelInfo {
	out := &meshv1.ChannelInfo{
		Id:           c.ID,
		Name:         c.Name,
		PodKeys:      c.PodKeys,
		MessageCount: int32(c.MessageCount),
		IsArchived:   c.IsArchived,
	}
	if c.Description != nil {
		out.Description = c.Description
	}
	return out
}

func toProtoRunnerInfo(r domainmesh.RunnerInfo) *meshv1.RunnerInfo {
	return &meshv1.RunnerInfo{
		Id:                r.ID,
		NodeId:            r.NodeID,
		Status:            r.Status,
		MaxConcurrentPods: int32(r.MaxConcurrentPods),
		CurrentPods:       int32(r.CurrentPods),
	}
}

func toProtoTopology(t *domainmesh.MeshTopology) *meshv1.MeshTopology {
	if t == nil {
		return &meshv1.MeshTopology{}
	}
	out := &meshv1.MeshTopology{
		Nodes:    make([]*meshv1.MeshNode, 0, len(t.Nodes)),
		Edges:    make([]*meshv1.MeshEdge, 0, len(t.Edges)),
		Channels: make([]*meshv1.ChannelInfo, 0, len(t.Channels)),
		Runners:  make([]*meshv1.RunnerInfo, 0, len(t.Runners)),
	}
	for _, n := range t.Nodes {
		out.Nodes = append(out.Nodes, toProtoMeshNode(n))
	}
	for _, e := range t.Edges {
		out.Edges = append(out.Edges, toProtoMeshEdge(e))
	}
	for _, c := range t.Channels {
		out.Channels = append(out.Channels, toProtoChannelInfo(c))
	}
	for _, r := range t.Runners {
		out.Runners = append(out.Runners, toProtoRunnerInfo(r))
	}
	return out
}

func toProtoMeshNodes(nodes []domainmesh.MeshNode) []*meshv1.MeshNode {
	out := make([]*meshv1.MeshNode, 0, len(nodes))
	for _, n := range nodes {
		out = append(out, toProtoMeshNode(n))
	}
	return out
}

// podToProtoMeshNode mirrors service/mesh/service.go:podToNode but emits the
// proto wire shape directly. Used by CreatePodForTicket — the underlying
// mesh service returns the raw agentpod.Pod from the orchestrator (not the
// domain projection), so the Connect handler does the projection itself.
//
// Field set matches toProtoMeshNode exactly so renderer callers see the
// same shape from both topology reads and pod-creation responses.
func podToProtoMeshNode(p *agentpod.Pod) *meshv1.MeshNode {
	if p == nil {
		return nil
	}
	out := &meshv1.MeshNode{
		PodKey:      p.PodKey,
		Status:      p.Status,
		AgentStatus: p.AgentStatus,
		AgentSlug:   p.AgentSlug,
		CreatedById: p.CreatedByID,
		RunnerId:    p.RunnerID,
	}
	if p.Model != nil {
		out.Model = p.Model
	}
	if p.Title != nil {
		out.Title = p.Title
	}
	if p.Alias != nil {
		out.Alias = p.Alias
	}
	if p.TicketID != nil {
		out.TicketId = p.TicketID
	}
	if p.RepositoryID != nil {
		out.RepositoryId = p.RepositoryID
	}
	if p.StartedAt != nil {
		s := p.StartedAt.UTC().Format(time.RFC3339)
		out.StartedAt = &s
	}
	if p.Runner != nil {
		out.RunnerNodeId = p.Runner.NodeID
		out.RunnerStatus = p.Runner.Status
	}
	if p.Ticket != nil {
		slug := p.Ticket.Slug
		title := p.Ticket.Title
		out.TicketSlug = &slug
		out.TicketTitle = &title
	}
	return out
}
