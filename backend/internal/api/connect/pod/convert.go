package podconnect

import (
	"time"

	poddom "github.com/anthropics/agentsmesh/backend/internal/domain/agentpod"
	podv1 "github.com/anthropics/agentsmesh/proto/gen/go/pod/v1"
)

// toProtoPod converts the GORM-backed Pod into the protobuf wire shape.
// Mirrors the JSON envelope the REST handler emits today (TS reads runner_id,
// runner.node_id, agent.name, etc.) so the wire format stays byte-identical
// from the consumer's perspective.
func toProtoPod(p *poddom.Pod) *podv1.Pod {
	if p == nil {
		return nil
	}
	out := &podv1.Pod{
		Id:              p.ID,
		PodKey:          p.PodKey,
		Status:          p.Status,
		AgentStatus:     p.AgentStatus,
		AgentSlug:       p.AgentSlug,
		InteractionMode: p.InteractionMode,
		Perpetual:       p.Perpetual,
		RestartCount:    int32(p.RestartCount),
		CreatedAt:       p.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:       p.UpdatedAt.UTC().Format(time.RFC3339),
		CreatedById:     &p.CreatedByID,
	}
	out.Alias = p.Alias
	out.Title = p.Title
	if p.RunnerID > 0 {
		rid := p.RunnerID
		out.RunnerId = &rid
	}
	if p.Prompt != "" {
		v := p.Prompt
		out.Prompt = &v
	}
	out.BranchName = p.BranchName
	out.SandboxPath = p.SandboxPath
	out.ErrorCode = p.ErrorCode
	out.ErrorMessage = p.ErrorMessage
	out.SourcePodKey = p.SourcePodKey
	out.SessionId = p.SessionID
	if p.LastRestartAt != nil {
		v := p.LastRestartAt.UTC().Format(time.RFC3339)
		out.LastRestartAt = &v
	}
	if p.StartedAt != nil {
		v := p.StartedAt.UTC().Format(time.RFC3339)
		out.StartedAt = &v
	}
	if p.FinishedAt != nil {
		v := p.FinishedAt.UTC().Format(time.RFC3339)
		out.FinishedAt = &v
	}
	if p.LastActivity != nil {
		v := p.LastActivity.UTC().Format(time.RFC3339)
		out.LastActivity = &v
	}
	out.Runner = toProtoPodRunner(p)
	out.Agent = toProtoPodAgent(p)
	out.Repository = toProtoPodRepository(p)
	out.Ticket = toProtoPodTicket(p)
	out.Loop = toProtoPodLoop(p)
	out.CreatedBy = toProtoPodCreatedBy(p)
	return out
}

func toProtoPodRunner(p *poddom.Pod) *podv1.PodRunnerInfo {
	if p.Runner == nil {
		return nil
	}
	id := p.Runner.ID
	node := p.Runner.NodeID
	status := p.Runner.Status
	return &podv1.PodRunnerInfo{Id: &id, NodeId: &node, Status: &status}
}

func toProtoPodAgent(p *poddom.Pod) *podv1.PodAgentInfo {
	if p.Agent == nil {
		return nil
	}
	name := p.Agent.Name
	slug := p.Agent.Slug
	return &podv1.PodAgentInfo{Name: &name, Slug: &slug}
}

func toProtoPodRepository(p *poddom.Pod) *podv1.PodRepositoryInfo {
	if p.Repository == nil {
		return nil
	}
	id := p.Repository.ID
	name := p.Repository.Name
	slug := p.Repository.Slug
	pt := p.Repository.ProviderType
	return &podv1.PodRepositoryInfo{Id: &id, Name: &name, Slug: &slug, ProviderType: &pt}
}

func toProtoPodTicket(p *poddom.Pod) *podv1.PodTicketInfo {
	if p.Ticket == nil {
		return nil
	}
	id := p.Ticket.ID
	slug := p.Ticket.Slug
	title := p.Ticket.Title
	return &podv1.PodTicketInfo{Id: &id, Slug: &slug, Title: &title}
}

func toProtoPodLoop(p *poddom.Pod) *podv1.PodLoopInfo {
	if p.Loop == nil {
		return nil
	}
	id := p.Loop.ID
	name := p.Loop.Name
	slug := p.Loop.Slug
	return &podv1.PodLoopInfo{Id: &id, Name: &name, Slug: &slug}
}

func toProtoPodCreatedBy(p *poddom.Pod) *podv1.PodCreatedByInfo {
	if p.CreatedBy == nil {
		return nil
	}
	id := p.CreatedBy.ID
	username := p.CreatedBy.Username
	out := &podv1.PodCreatedByInfo{Id: &id, Username: &username}
	if p.CreatedBy.Name != nil {
		v := *p.CreatedBy.Name
		out.Name = &v
	}
	return out
}

func toProtoPods(pods []*poddom.Pod) []*podv1.Pod {
	out := make([]*podv1.Pod, 0, len(pods))
	for _, p := range pods {
		out = append(out, toProtoPod(p))
	}
	return out
}
