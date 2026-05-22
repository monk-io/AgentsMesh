package adminconnect

import (
	"context"
	"errors"
	"time"

	"connectrpc.com/connect"

	"github.com/anthropics/agentsmesh/backend/internal/api/connect/interceptors"
	"github.com/anthropics/agentsmesh/backend/internal/domain/agent"
	adminv1 "github.com/anthropics/agentsmesh/proto/gen/go/admin/v1"
)

// ListDeadLetters returns dead-letter queue entries, mirroring REST GET
// /api/v1/orgs/:slug/messages/dlq. The org_slug in the REST path was
// vestigial — the underlying service.GetDeadLetters does no tenant
// filtering. The Connect surface drops the slug and gates on admin role.
func (s *Server) ListDeadLetters(
	ctx context.Context, req *connect.Request[adminv1.ListDeadLettersRequest],
) (*connect.Response[adminv1.ListDeadLettersResponse], error) {
	ctx, _, err := interceptors.ResolveSystemAdmin(ctx, s.db)
	if err != nil {
		return nil, err
	}
	if s.msgSvc == nil {
		return nil, connect.NewError(connect.CodeUnavailable,
			errors.New("agent message service not configured"))
	}

	limit, offset := normalizeDLQArgs(req.Msg.GetLimit(), req.Msg.GetOffset())
	entries, err := s.msgSvc.GetDeadLetters(ctx, int(limit), int(offset))
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}

	items := make([]*adminv1.DeadLetterEntry, 0, len(entries))
	for _, e := range entries {
		items = append(items, toProtoDeadLetter(e))
	}
	return connect.NewResponse(&adminv1.ListDeadLettersResponse{
		Items:  items,
		Total:  int64(len(items)),
		Limit:  limit,
		Offset: offset,
	}), nil
}

// ReplayDeadLetter attempts to replay a DLQ entry, mirroring REST POST
// /api/v1/orgs/:slug/messages/dlq/:id/replay.
func (s *Server) ReplayDeadLetter(
	ctx context.Context, req *connect.Request[adminv1.ReplayDeadLetterRequest],
) (*connect.Response[adminv1.ReplayDeadLetterResponse], error) {
	ctx, _, err := interceptors.ResolveSystemAdmin(ctx, s.db)
	if err != nil {
		return nil, err
	}
	if s.msgSvc == nil {
		return nil, connect.NewError(connect.CodeUnavailable,
			errors.New("agent message service not configured"))
	}
	if req.Msg.GetEntryId() == 0 {
		return nil, connect.NewError(connect.CodeInvalidArgument,
			errors.New("entry_id is required"))
	}

	msg, err := s.msgSvc.ReplayDeadLetter(ctx, req.Msg.GetEntryId())
	if err != nil {
		return nil, connect.NewError(connect.CodeNotFound, err)
	}
	return connect.NewResponse(&adminv1.ReplayDeadLetterResponse{
		Message:         "Replayed successfully",
		ReplayedMessage: toProtoAgentMessage(msg),
	}), nil
}

func normalizeDLQArgs(limit, offset int32) (int32, int32) {
	if limit < 1 || limit > 200 {
		limit = 50
	}
	if offset < 0 {
		offset = 0
	}
	return limit, offset
}

func toProtoDeadLetter(e *agent.DeadLetterEntry) *adminv1.DeadLetterEntry {
	if e == nil {
		return nil
	}
	out := &adminv1.DeadLetterEntry{
		Id:                e.ID,
		OriginalMessageId: e.OriginalMessageID,
		Reason:            e.Reason,
		FinalAttempt:      int32(e.FinalAttempt),
		MovedAt:           e.MovedAt.UTC().Format(time.RFC3339),
		CreatedAt:         e.CreatedAt.UTC().Format(time.RFC3339),
	}
	if e.ReplayedAt != nil {
		s := e.ReplayedAt.UTC().Format(time.RFC3339)
		out.ReplayedAt = &s
	}
	if e.ReplayResult != nil {
		out.ReplayResult = e.ReplayResult
	}
	if e.OriginalMessage != nil {
		out.OriginalMessage = toProtoAgentMessage(e.OriginalMessage)
	}
	return out
}

func toProtoAgentMessage(m *agent.AgentMessage) *adminv1.AgentMessage {
	if m == nil {
		return nil
	}
	out := &adminv1.AgentMessage{
		Id:               m.ID,
		SenderPod:        m.SenderPod,
		ReceiverPod:      m.ReceiverPod,
		MessageType:      m.MessageType,
		Status:           m.Status,
		DeliveryAttempts: int32(m.DeliveryAttempts),
		MaxRetries:       int32(m.MaxRetries),
		CreatedAt:        m.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt:        m.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if m.LastDeliveryAttempt != nil {
		s := m.LastDeliveryAttempt.UTC().Format(time.RFC3339)
		out.LastDeliveryAttempt = &s
	}
	if m.NextRetryAt != nil {
		s := m.NextRetryAt.UTC().Format(time.RFC3339)
		out.NextRetryAt = &s
	}
	if m.DeliveryError != nil {
		out.DeliveryError = m.DeliveryError
	}
	if m.DeliveredAt != nil {
		s := m.DeliveredAt.UTC().Format(time.RFC3339)
		out.DeliveredAt = &s
	}
	if m.ReadAt != nil {
		s := m.ReadAt.UTC().Format(time.RFC3339)
		out.ReadAt = &s
	}
	if m.ParentMessageID != nil {
		out.ParentMessageId = m.ParentMessageID
	}
	if m.CorrelationID != nil {
		out.CorrelationId = m.CorrelationID
	}
	return out
}
