// Connect-RPC adapter for proto.support_ticket.v1.SupportTicketService.
//
// Encodes requests via @bufbuild/protobuf .toBinary(), passes the Uint8Array
// to the wasm bridge (binary in / binary out per conventions §2.5), decodes
// responses via .fromBinary(). No JSON intermediate.
//
// Returns snake_case web shapes (SupportTicket, SupportTicketDetail,
// SupportTicketListResponse) so the existing call sites in
// SupportTicketsContent / support detail page / message-list don't have
// to flip off camelCase + BigInt — same dual-track pattern as
// invitationConnect.ts during the migration window.
//
// REST shape translation:
// - Proto list envelope is {items, total, limit, offset} (conventions §8);
//   the renderer reads {data, total, page, page_size, total_pages} for
//   pagination UI. This adapter remaps so existing code keeps working.
// - Page/page_size translate to offset/limit: offset = (page-1) * page_size.

import {
  GetAttachmentUrlRequestSchema,
  GetAttachmentUrlResponseSchema,
  GetSupportTicketRequestSchema,
  ListSupportTicketsRequestSchema,
  ListSupportTicketsResponseSchema,
  SupportTicketDetailSchema,
  type SupportTicket as ProtoSupportTicket,
  type SupportTicketAttachment as ProtoSupportTicketAttachment,
  type SupportTicketMessage as ProtoSupportTicketMessage,
} from "@proto/support_ticket/v1/support_ticket_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getSupportTicketService } from "@/lib/wasm-core";
import type {
  SupportTicket,
  SupportTicketAttachment,
  SupportTicketDetail,
  SupportTicketListParams,
  SupportTicketListResponse,
  SupportTicketMessage,
} from "@/lib/api/supportTicketTypes";

// ============== Wire conversion (proto -> snake_case web shape) ==============

export function fromProtoTicket(p: ProtoSupportTicket): SupportTicket {
  return {
    id: Number(p.id),
    user_id: Number(p.userId),
    title: p.title,
    category: p.category,
    status: p.status,
    priority: p.priority,
    created_at: p.createdAt,
    updated_at: p.updatedAt,
    resolved_at: p.resolvedAt,
  };
}

export function fromProtoAttachment(p: ProtoSupportTicketAttachment): SupportTicketAttachment {
  return {
    id: Number(p.id),
    ticket_id: Number(p.ticketId),
    message_id: p.messageId !== undefined ? Number(p.messageId) : undefined,
    uploader_id: Number(p.uploaderId),
    original_name: p.originalName,
    mime_type: p.mimeType,
    size: Number(p.size),
    created_at: p.createdAt,
  };
}

export function fromProtoMessage(p: ProtoSupportTicketMessage): SupportTicketMessage {
  return {
    id: Number(p.id),
    ticket_id: Number(p.ticketId),
    user_id: Number(p.userId),
    content: p.content,
    is_admin_reply: p.isAdminReply,
    created_at: p.createdAt,
    user: p.user
      ? {
          id: Number(p.user.id),
          name: p.user.name ?? "",
          email: p.user.email,
          avatar_url: p.user.avatarUrl,
        }
      : undefined,
    attachments: p.attachments.map(fromProtoAttachment),
  };
}

// ============== RPCs ==============

export async function listSupportTickets(
  params?: SupportTicketListParams,
): Promise<SupportTicketListResponse> {
  const page = params?.page ?? 1;
  const pageSize = params?.page_size ?? 20;
  const offset = (page - 1) * pageSize;
  const req = create(ListSupportTicketsRequestSchema, {
    status: params?.status ?? "",
    offset,
    limit: pageSize,
  });
  const bytes = toBinary(ListSupportTicketsRequestSchema, req);
  const respBytes = await getSupportTicketService().listSupportTicketsConnect(bytes);
  const resp = fromBinary(ListSupportTicketsResponseSchema, new Uint8Array(respBytes));

  const total = Number(resp.total);
  const totalPages = pageSize > 0 ? Math.ceil(total / pageSize) : 1;
  return {
    data: resp.items.map(fromProtoTicket),
    total,
    page,
    page_size: pageSize,
    total_pages: totalPages,
  };
}

export async function getSupportTicketDetail(id: number): Promise<SupportTicketDetail> {
  const req = create(GetSupportTicketRequestSchema, { id: BigInt(id) });
  const bytes = toBinary(GetSupportTicketRequestSchema, req);
  const respBytes = await getSupportTicketService().getSupportTicketConnect(bytes);
  const resp = fromBinary(SupportTicketDetailSchema, new Uint8Array(respBytes));
  if (!resp.ticket) {
    throw new Error("support ticket detail response missing ticket");
  }
  return {
    ticket: fromProtoTicket(resp.ticket),
    messages: resp.messages.map(fromProtoMessage),
  };
}

export async function getSupportTicketAttachmentUrl(
  attachmentId: number,
): Promise<{ url: string }> {
  const req = create(GetAttachmentUrlRequestSchema, { attachmentId: BigInt(attachmentId) });
  const bytes = toBinary(GetAttachmentUrlRequestSchema, req);
  const respBytes = await getSupportTicketService().getAttachmentUrlConnect(bytes);
  const resp = fromBinary(GetAttachmentUrlResponseSchema, new Uint8Array(respBytes));
  return { url: resp.url };
}
