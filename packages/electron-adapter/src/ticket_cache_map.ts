import { create } from "@bufbuild/protobuf";
import {
  TicketSchema,
  type Ticket as ProtoTicket,
  type Label as ProtoLabel,
  type BoardColumn as ProtoBoardColumn,
} from "@agentsmesh/proto/ticket/v1/ticket_pb";

// snake_case projection of proto.ticket.v1 types. Mirrors protoTicketToTicket /
// ticketToProto in clients/web/src/lib/api/ticketProtoMap.ts but inlined to
// keep electron-adapter independent of the web/src tree (same rationale as
// podToCache in pod.ts). The cache holds only the SSOT-shape Ticket; the UI
// re-reads joined fields (reporter/assignees/labels) from the API payload.

export interface CachedTicket {
  id: number; number: number; slug: string; title: string;
  content?: string; status: string; priority: string; severity?: string;
  estimate?: number; due_date?: string; started_at?: string; completed_at?: string;
  repository_id?: number; created_at?: string; updated_at?: string;
}

export interface CachedBoardColumn {
  status: string; tickets: CachedTicket[]; total_count: number;
}

export function ticketToCache(p: ProtoTicket): CachedTicket {
  return {
    id: Number(p.id), number: p.number, slug: p.slug, title: p.title,
    content: p.content, status: p.status, priority: p.priority, severity: p.severity,
    estimate: p.estimate, due_date: p.dueDate, started_at: p.startedAt,
    completed_at: p.completedAt,
    repository_id: p.repositoryId !== undefined ? Number(p.repositoryId) : undefined,
    created_at: p.createdAt, updated_at: p.updatedAt,
  };
}

export function boardColumnToCache(c: ProtoBoardColumn): CachedBoardColumn {
  return { status: c.status, tickets: c.tickets.map(ticketToCache), total_count: Number(c.totalCount) };
}

export function labelToCache(p: ProtoLabel): Record<string, unknown> {
  return { id: Number(p.id), name: p.name, color: p.color };
}

// Inverse of ticketToCache: filter_tickets must return a FilterTicketsResponse
// of proto Tickets (the store decodes it back via protoTicketToTicket).
export function cacheTicketToProto(t: CachedTicket): ProtoTicket {
  return create(TicketSchema, {
    id: BigInt(t.id), number: t.number, slug: t.slug, title: t.title,
    content: t.content, status: t.status, priority: t.priority, severity: t.severity,
    estimate: t.estimate, dueDate: t.due_date, startedAt: t.started_at, completedAt: t.completed_at,
    repositoryId: t.repository_id !== undefined ? BigInt(t.repository_id) : undefined,
    createdAt: t.created_at ?? "", updatedAt: t.updated_at ?? "",
  });
}
