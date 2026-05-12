package ticketrelationsconnect

import (
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/ticket"
	ticketrelationsv1 "github.com/anthropics/agentsmesh/proto/gen/go/ticket_relations/v1"
)

// toProtoRelation converts the GORM-backed domain model into proto. Preloaded
// Source/TargetTicket associations (ListRelations preloads via Preload) are
// projected to RelatedTicketSummary so the UI can render without a per-row
// resolver hop.
func toProtoRelation(r *ticket.Relation) *ticketrelationsv1.Relation {
	if r == nil {
		return nil
	}
	out := &ticketrelationsv1.Relation{
		Id:             r.ID,
		OrganizationId: r.OrganizationID,
		SourceTicketId: r.SourceTicketID,
		TargetTicketId: r.TargetTicketID,
		RelationType:   r.RelationType,
		CreatedAt:      r.CreatedAt.UTC().Format(time.RFC3339),
	}
	if r.SourceTicket != nil {
		out.SourceTicket = &ticketrelationsv1.RelatedTicketSummary{
			Id:    r.SourceTicket.ID,
			Slug:  r.SourceTicket.Slug,
			Title: r.SourceTicket.Title,
		}
	}
	if r.TargetTicket != nil {
		out.TargetTicket = &ticketrelationsv1.RelatedTicketSummary{
			Id:    r.TargetTicket.ID,
			Slug:  r.TargetTicket.Slug,
			Title: r.TargetTicket.Title,
		}
	}
	return out
}

// toProtoMergeRequest mirrors the REST handler's JSON projection. Fields kept
// in lockstep with ticket.MergeRequest (domain/ticket/merge_request.go).
func toProtoMergeRequest(mr *ticket.MergeRequest) *ticketrelationsv1.MergeRequest {
	if mr == nil {
		return nil
	}
	out := &ticketrelationsv1.MergeRequest{
		Id:             mr.ID,
		MrIid:          int32(mr.MRIID),
		Title:          mr.Title,
		State:          mr.State,
		MrUrl:          mr.MRURL,
		SourceBranch:   mr.SourceBranch,
		TargetBranch:   mr.TargetBranch,
		PipelineStatus: mr.PipelineStatus,
		PipelineId:     mr.PipelineID,
		PipelineUrl:    mr.PipelineURL,
		TicketId:       mr.TicketID,
		PodId:          mr.PodID,
		MergeCommitSha: mr.MergeCommitSHA,
		MergedById:     mr.MergedByID,
		CreatedAt:      mr.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:      mr.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if mr.MergedAt != nil {
		s := mr.MergedAt.UTC().Format(time.RFC3339)
		out.MergedAt = &s
	}
	return out
}

func toProtoCommit(c *ticket.Commit) *ticketrelationsv1.Commit {
	if c == nil {
		return nil
	}
	out := &ticketrelationsv1.Commit{
		Id:             c.ID,
		OrganizationId: c.OrganizationID,
		TicketId:       c.TicketID,
		RepositoryId:   c.RepositoryID,
		PodId:          c.PodID,
		CommitSha:      c.CommitSHA,
		CommitMessage:  c.CommitMessage,
		CommitUrl:      c.CommitURL,
		AuthorName:     c.AuthorName,
		AuthorEmail:    c.AuthorEmail,
		CreatedAt:      c.CreatedAt.UTC().Format(time.RFC3339),
	}
	if c.CommittedAt != nil {
		s := c.CommittedAt.UTC().Format(time.RFC3339)
		out.CommittedAt = &s
	}
	return out
}

func toProtoComment(c *ticket.Comment) *ticketrelationsv1.Comment {
	if c == nil {
		return nil
	}
	out := &ticketrelationsv1.Comment{
		Id:        c.ID,
		TicketId:  c.TicketID,
		UserId:    c.UserID,
		Content:   c.Content,
		ParentId:  c.ParentID,
		Mentions:  toProtoMentions(c.Mentions),
		CreatedAt: c.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt: c.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if c.User != nil {
		out.User = toProtoCommentUser(c.User)
	}
	for i := range c.Replies {
		out.Replies = append(out.Replies, toProtoComment(&c.Replies[i]))
	}
	return out
}

func toProtoMentions(in []ticket.CommentMention) []*ticketrelationsv1.CommentMention {
	if len(in) == 0 {
		return nil
	}
	out := make([]*ticketrelationsv1.CommentMention, 0, len(in))
	for i := range in {
		out = append(out, &ticketrelationsv1.CommentMention{
			UserId:   in[i].UserID,
			Username: in[i].Username,
		})
	}
	return out
}

func toProtoCommentUser(u *ticket.AssigneeUser) *ticketrelationsv1.CommentUser {
	if u == nil {
		return nil
	}
	return &ticketrelationsv1.CommentUser{
		Id:        u.ID,
		Username:  u.Username,
		Name:      u.Name,
		AvatarUrl: u.AvatarURL,
	}
}

// fromProtoMentions inverts toProtoMentions for write-path RPCs (Create /
// Update comment).
func fromProtoMentions(in []*ticketrelationsv1.CommentMention) []ticket.CommentMention {
	if len(in) == 0 {
		return nil
	}
	out := make([]ticket.CommentMention, 0, len(in))
	for _, m := range in {
		if m == nil {
			continue
		}
		out = append(out, ticket.CommentMention{
			UserID:   m.GetUserId(),
			Username: m.GetUsername(),
		})
	}
	return out
}
