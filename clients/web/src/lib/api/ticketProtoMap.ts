// TicketData ↔ proto.ticket.v1.Ticket mapping used by the ticket store
// when handing payloads to the wasm bridge encoded as proto-state bytes.
// Mirrors podProtoMap.ts in shape and intent.
//
// The ViewModel (TicketData) carries a handful of UI-only joins (reporter,
// assignees, labels, repository, parent_ticket) that aren't part of the
// proto.ticket.v1.Ticket schema. Those fields are dropped on the way down;
// the wasm cache only retains the SSOT-shape Ticket. Wherever the UI needs
// joined data it re-reads from the original API response payload, not from
// the wasm cache (the cache is a list of Tickets, not a list of joined
// objects).

import { create as protoCreate } from "@bufbuild/protobuf";
import {
  TicketSchema, LabelSchema, BoardColumnSchema,
  type Ticket as ProtoTicket, type Label as ProtoLabel,
  type BoardColumn as ProtoBoardColumn,
} from "@proto/ticket/v1/ticket_pb";
import type {
  TicketData, BoardColumn,
} from "@/lib/viewModels/ticket";

function asBigInt(v: number | undefined | null): bigint | undefined {
  return v === undefined || v === null ? undefined : BigInt(v);
}

export function ticketToProto(t: TicketData): ProtoTicket {
  return protoCreate(TicketSchema, {
    id: asBigInt(t.id) ?? BigInt(0),
    number: t.number,
    slug: t.slug,
    title: t.title,
    content: t.content,
    status: t.status,
    priority: t.priority,
    severity: t.severity,
    estimate: t.estimate,
    dueDate: t.due_date,
    startedAt: t.started_at,
    completedAt: t.completed_at,
    repositoryId: asBigInt(t.repository_id),
    createdAt: t.created_at ?? "",
    updatedAt: t.updated_at ?? "",
  });
}

export function ticketsToProto(tickets: TicketData[]): ProtoTicket[] {
  return tickets.map(ticketToProto);
}

// Read-back from wasm cache. Mirrors fromProtoTicket in ticketConnect.ts
// but kept here so the store module owns its inverse.
export function protoTicketToTicket(p: ProtoTicket): TicketData {
  return {
    id: Number(p.id),
    number: p.number,
    slug: p.slug,
    title: p.title,
    content: p.content,
    status: p.status as TicketData["status"],
    priority: p.priority as TicketData["priority"],
    severity: p.severity,
    estimate: p.estimate,
    due_date: p.dueDate,
    started_at: p.startedAt,
    completed_at: p.completedAt,
    repository_id: p.repositoryId !== undefined ? Number(p.repositoryId) : undefined,
    created_at: p.createdAt,
    updated_at: p.updatedAt,
  };
}

export function labelToProto(l: { id: number; name: string; color: string }): ProtoLabel {
  return protoCreate(LabelSchema, {
    id: BigInt(l.id),
    name: l.name,
    color: l.color,
  });
}

export function labelsToProto(labels: { id: number; name: string; color: string }[]): ProtoLabel[] {
  return labels.map(labelToProto);
}

export function boardColumnsToProto(cols: BoardColumn[]): ProtoBoardColumn[] {
  return cols.map((c) => protoCreate(BoardColumnSchema, {
    status: c.status,
    tickets: ticketsToProto(c.tickets),
    totalCount: BigInt(c.count),
  }));
}
