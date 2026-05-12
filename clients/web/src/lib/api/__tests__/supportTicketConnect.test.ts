// Unit tests for clients/web/src/lib/api/supportTicketConnect.ts proto →
// snake_case converters. Pins the field-by-field translation across the
// binary wire so the wire-drift bug class (PR #341 / #368 / 986a38ca6)
// is compile-time impossible for support_ticket.
//
// Note: this test file imports @proto/support_ticket/v1/support_ticket_pb
// which resolves via vitest.config.ts. Under `bazel test //clients/web:unit`
// the `proto/gen` tree is in .bazelignore so the import fails — runs
// under `pnpm test:run` (or once ts_proto_library lands; see runbook
// "TS proto codegen toolchain").

import { describe, it, expect } from "vitest";
import { create } from "@bufbuild/protobuf";
import {
  SupportTicketAttachmentSchema,
  SupportTicketMessageSchema,
  SupportTicketSchema,
  SupportTicketUserSchema,
} from "@proto/support_ticket/v1/support_ticket_pb";
import {
  fromProtoAttachment,
  fromProtoMessage,
  fromProtoTicket,
} from "../supportTicketConnect";

describe("fromProtoTicket", () => {
  it("converts the full SupportTicket proto to snake_case web shape", () => {
    const proto = create(SupportTicketSchema, {
      id: BigInt(42),
      userId: BigInt(7),
      title: "Login failure",
      category: "bug",
      status: "open",
      priority: "high",
      createdAt: "2026-05-12T13:16:10Z",
      updatedAt: "2026-05-12T13:16:10Z",
      resolvedAt: "2026-05-13T00:00:00Z",
    });
    const data = fromProtoTicket(proto);
    expect(data.id).toBe(42);
    expect(data.user_id).toBe(7);
    expect(data.title).toBe("Login failure");
    expect(data.category).toBe("bug");
    expect(data.status).toBe("open");
    expect(data.priority).toBe("high");
    expect(data.created_at).toBe("2026-05-12T13:16:10Z");
    expect(data.updated_at).toBe("2026-05-12T13:16:10Z");
    expect(data.resolved_at).toBe("2026-05-13T00:00:00Z");
  });

  it("handles unresolved tickets (no resolved_at)", () => {
    const proto = create(SupportTicketSchema, {
      id: BigInt(1),
      userId: BigInt(1),
      title: "Q",
      category: "other",
      status: "open",
      priority: "medium",
      createdAt: "2026-05-12T00:00:00Z",
      updatedAt: "2026-05-12T00:00:00Z",
    });
    const data = fromProtoTicket(proto);
    expect(data.resolved_at).toBeUndefined();
  });

  it("narrows BigInt to number for id/user_id", () => {
    const proto = create(SupportTicketSchema, {
      id: BigInt(2_000_000),
      userId: BigInt(1_000_000),
      title: "x",
      category: "bug",
      status: "open",
      priority: "low",
      createdAt: "2026-05-12T00:00:00Z",
      updatedAt: "2026-05-12T00:00:00Z",
    });
    const data = fromProtoTicket(proto);
    expect(typeof data.id).toBe("number");
    expect(typeof data.user_id).toBe("number");
    expect(data.id).toBe(2_000_000);
    expect(data.user_id).toBe(1_000_000);
  });
});

describe("fromProtoAttachment", () => {
  it("converts attachment proto preserving size as number", () => {
    const proto = create(SupportTicketAttachmentSchema, {
      id: BigInt(11),
      ticketId: BigInt(42),
      messageId: BigInt(99),
      uploaderId: BigInt(7),
      originalName: "screenshot.png",
      mimeType: "image/png",
      size: BigInt(12_345),
      createdAt: "2026-05-12T13:16:10Z",
    });
    const data = fromProtoAttachment(proto);
    expect(data).toEqual({
      id: 11,
      ticket_id: 42,
      message_id: 99,
      uploader_id: 7,
      original_name: "screenshot.png",
      mime_type: "image/png",
      size: 12_345,
      created_at: "2026-05-12T13:16:10Z",
    });
  });

  it("treats ticket-level attachment (no message_id) as undefined", () => {
    const proto = create(SupportTicketAttachmentSchema, {
      id: BigInt(11),
      ticketId: BigInt(42),
      uploaderId: BigInt(7),
      originalName: "doc.pdf",
      mimeType: "application/pdf",
      size: BigInt(2048),
      createdAt: "2026-05-12T13:16:10Z",
    });
    const data = fromProtoAttachment(proto);
    expect(data.message_id).toBeUndefined();
  });
});

describe("fromProtoMessage", () => {
  it("converts message with user + attachments", () => {
    const userProto = create(SupportTicketUserSchema, {
      id: BigInt(7),
      name: "Alice",
      email: "alice@example.com",
      avatarUrl: "https://example.com/a.png",
    });
    const attProto = create(SupportTicketAttachmentSchema, {
      id: BigInt(11),
      ticketId: BigInt(42),
      messageId: BigInt(99),
      uploaderId: BigInt(7),
      originalName: "s.png",
      mimeType: "image/png",
      size: BigInt(1024),
      createdAt: "2026-05-12T13:16:10Z",
    });
    const proto = create(SupportTicketMessageSchema, {
      id: BigInt(99),
      ticketId: BigInt(42),
      userId: BigInt(7),
      content: "Steps to reproduce: ...",
      isAdminReply: false,
      createdAt: "2026-05-12T13:16:10Z",
      user: userProto,
      attachments: [attProto],
    });
    const data = fromProtoMessage(proto);
    expect(data.id).toBe(99);
    expect(data.ticket_id).toBe(42);
    expect(data.user_id).toBe(7);
    expect(data.content).toBe("Steps to reproduce: ...");
    expect(data.is_admin_reply).toBe(false);
    expect(data.user).toEqual({
      id: 7,
      name: "Alice",
      email: "alice@example.com",
      avatar_url: "https://example.com/a.png",
    });
    expect(data.attachments).toHaveLength(1);
    expect(data.attachments![0].original_name).toBe("s.png");
  });

  it("admin reply has is_admin_reply true", () => {
    const proto = create(SupportTicketMessageSchema, {
      id: BigInt(100),
      ticketId: BigInt(42),
      userId: BigInt(2),
      content: "We're investigating",
      isAdminReply: true,
      createdAt: "2026-05-12T13:16:10Z",
    });
    const data = fromProtoMessage(proto);
    expect(data.is_admin_reply).toBe(true);
    expect(data.user).toBeUndefined();
    expect(data.attachments).toEqual([]);
  });

  it("user.name falls back to empty string when not set", () => {
    const userProto = create(SupportTicketUserSchema, {
      id: BigInt(7),
      email: "alice@example.com",
    });
    const proto = create(SupportTicketMessageSchema, {
      id: BigInt(99),
      ticketId: BigInt(42),
      userId: BigInt(7),
      content: "hi",
      isAdminReply: false,
      createdAt: "2026-05-12T13:16:10Z",
      user: userProto,
    });
    const data = fromProtoMessage(proto);
    expect(data.user?.name).toBe("");
    expect(data.user?.email).toBe("alice@example.com");
  });
});
