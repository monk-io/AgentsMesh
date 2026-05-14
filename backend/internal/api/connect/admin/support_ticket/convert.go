package supportticketadminconnect

import (
	"time"

	supportticketdomain "github.com/anthropics/agentsmesh/backend/internal/domain/supportticket"
	supportticketv1 "github.com/anthropics/agentsmesh/proto/gen/go/support_ticket/v1"
)

// toProtoAdminTicket mirrors the GORM-backed domain model into the wire shape,
// including AssignedAdminID which the user-facing variant intentionally omits.
func toProtoAdminTicket(t *supportticketdomain.SupportTicket) *supportticketv1.AdminSupportTicket {
	if t == nil {
		return nil
	}
	out := &supportticketv1.AdminSupportTicket{
		Id:        t.ID,
		UserId:    t.UserID,
		Title:     t.Title,
		Category:  t.Category,
		Status:    t.Status,
		Priority:  t.Priority,
		CreatedAt: t.CreatedAt.UTC().Format(time.RFC3339),
		UpdatedAt: t.UpdatedAt.UTC().Format(time.RFC3339),
	}
	if t.ResolvedAt != nil {
		v := t.ResolvedAt.UTC().Format(time.RFC3339)
		out.ResolvedAt = &v
	}
	if t.AssignedAdminID != nil {
		v := *t.AssignedAdminID
		out.AssignedAdminId = &v
	}
	return out
}

// toProtoAdminMessage converts a domain message + eager-loaded associations.
// Attachment.StorageKey is hidden — it's internal.
func toProtoAdminMessage(m *supportticketdomain.SupportTicketMessage) *supportticketv1.AdminSupportTicketMessage {
	if m == nil {
		return nil
	}
	out := &supportticketv1.AdminSupportTicketMessage{
		Id:           m.ID,
		TicketId:     m.TicketID,
		UserId:       m.UserID,
		Content:      m.Content,
		IsAdminReply: m.IsAdminReply,
		CreatedAt:    m.CreatedAt.UTC().Format(time.RFC3339),
		Attachments:  make([]*supportticketv1.AdminSupportTicketAttachment, 0, len(m.Attachments)),
	}
	if m.User != nil {
		out.User = &supportticketv1.AdminSupportTicketUser{
			Id:    m.User.ID,
			Email: m.User.Email,
		}
		if m.User.Name != nil {
			out.User.Name = m.User.Name
		}
		if m.User.AvatarURL != nil {
			out.User.AvatarUrl = m.User.AvatarURL
		}
	}
	for i := range m.Attachments {
		out.Attachments = append(out.Attachments, toProtoAdminAttachment(&m.Attachments[i]))
	}
	return out
}

func toProtoAdminAttachment(a *supportticketdomain.SupportTicketAttachment) *supportticketv1.AdminSupportTicketAttachment {
	if a == nil {
		return nil
	}
	out := &supportticketv1.AdminSupportTicketAttachment{
		Id:           a.ID,
		TicketId:     a.TicketID,
		UploaderId:   a.UploaderID,
		OriginalName: a.OriginalName,
		MimeType:     a.MimeType,
		Size:         a.Size,
		CreatedAt:    a.CreatedAt.UTC().Format(time.RFC3339),
	}
	if a.MessageID != nil {
		out.MessageId = a.MessageID
	}
	return out
}
