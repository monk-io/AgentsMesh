import { describe, it, expect, vi, beforeEach } from "vitest";
import { fromBinary } from "@bufbuild/protobuf";
import {
  ApplyIncomingChannelMessageRequestSchema,
  ApplyChannelMessageEditedEventRequestSchema,
} from "@proto/channel_state/v1/mutations_pb";
import { handleChannelEvent, handleInfraEvent } from "../realtimeEventHandlers";
import { useChannelMessageStore } from "@/stores/channel";
import { readMessages } from "@/stores/channelMessageStore";
import { useAuthStore } from "@/stores/auth";
import { getAuthManager } from "@/lib/wasm-core";
import { useChannelStore } from "@/stores/channel";
import type { RealtimeEvent } from "@/lib/realtime";

const mockUpdateRunnerStatus = vi.fn();
const mockUpdateTicketStatus = vi.fn();
const mockRemoveTicket = vi.fn();
const mockFetchTickets = vi.fn();
const mockFetchTicket = vi.fn();
const mockFetchPod = vi.fn();

vi.mock("@/stores/pod", () => ({
  usePodStore: { getState: () => ({ pods: [{ id: 1, pod_key: "pk-1" }], fetchPod: mockFetchPod }) },
}));
vi.mock("@/lib/wasm-core", async () => {
  const actual = await vi.importActual<typeof import("@/lib/wasm-core")>("@/lib/wasm-core");
  type Bucket = Map<number, Record<string, unknown>[]>;
  const buckets = (globalThis as unknown as { __channelBuckets?: Bucket }).__channelBuckets ?? new Map();
  const unread = (globalThis as unknown as { __channelUnread?: Record<number, number> }).__channelUnread ?? {};
  const authBox = (globalThis as unknown as { __authBox?: { user: unknown; current_org: unknown; organizations: unknown[] } }).__authBox
    ?? { user: null, current_org: null, organizations: [] };
  (globalThis as unknown as { __channelBuckets: Bucket }).__channelBuckets = buckets;
  (globalThis as unknown as { __channelUnread: Record<number, number> }).__channelUnread = unread;
  (globalThis as unknown as { __authBox: typeof authBox }).__authBox = authBox;
  return {
    ...actual,
    getAuthManager: () => ({
      get_current_user_json: () => (authBox.user ? JSON.stringify(authBox.user) : null),
      get_current_org_json: () => (authBox.current_org ? JSON.stringify(authBox.current_org) : null),
      get_organizations_json: () => JSON.stringify(authBox.organizations),
      apply_session: (sessionJson: string) => {
        try { authBox.user = (JSON.parse(sessionJson) as { user?: unknown }).user ?? null; } catch { /* noop */ }
      },
      set_organizations: (json: string) => {
        try {
          const orgs = JSON.parse(json);
          authBox.organizations = Array.isArray(orgs) ? orgs : [];
          if (authBox.current_org == null && authBox.organizations.length > 0) {
            authBox.current_org = authBox.organizations[0];
          }
        } catch { /* noop */ }
      },
      set_current_org: (json: string) => {
        if (json === "") { authBox.current_org = null; return; }
        try { authBox.current_org = JSON.parse(json); } catch { /* noop */ }
      },
      clear_session: () => { authBox.user = null; authBox.current_org = null; authBox.organizations = []; },
      is_authenticated: () => authBox.user !== null,
      logout: () => Promise.resolve(),
      switch_org: () => {},
    }),
    getPodService: () => ({
      pods_json: () => JSON.stringify([{ id: 1, pod_key: "pk-1" }]),
    }),
    getPodState: () => ({
      pods_json: () => JSON.stringify([{ id: 1, pod_key: "pk-1" }]),
    }),
    getTicketService: () => ({
      current_ticket_json: () => null,
    }),
    getTicketState: () => ({
      current_ticket_json: () => null,
    }),
    getChannelService: () => ({
      apply_incoming_channel_message: (bytes: Uint8Array) => {
        const req = fromBinary(ApplyIncomingChannelMessageRequestSchema, bytes);
        if (!req.message) return false;
        const channelId = Number(req.channelId);
        const msg = {
          id: Number(req.message.id), channel_id: channelId,
          body: req.message.body,
          sender_pod: req.message.senderPod,
          sender_user_id: req.message.senderUserId !== undefined ? Number(req.message.senderUserId) : undefined,
          message_type: req.message.messageType,
          content_json: req.message.contentJson,
          mentions_json: req.message.mentionsJson,
          reply_to: req.message.replyTo !== undefined ? Number(req.message.replyTo) : undefined,
          created_at: req.message.createdAt,
          sender_user: req.message.senderUser ? {
            id: Number(req.message.senderUser.id),
            username: req.message.senderUser.username,
            name: req.message.senderUser.name,
          } : undefined,
          sender_pod_info: req.message.senderPodInfo ? {
            pod_key: req.message.senderPodInfo.podKey,
            alias: req.message.senderPodInfo.alias,
            ...(req.message.senderPodInfo.agent ? { agent: { name: req.message.senderPodInfo.agent.name } } : {}),
          } : undefined,
        };
        const list = buckets.get(channelId) ?? [];
        if (!list.some((m: Record<string, unknown>) => (m as { id: number }).id === msg.id)) list.push(msg);
        buckets.set(channelId, list);
        return true;
      },
      apply_channel_message_edited_event: (bytes: Uint8Array) => {
        const req = fromBinary(ApplyChannelMessageEditedEventRequestSchema, bytes);
        const cid = Number(req.channelId);
        const list = buckets.get(cid) ?? [];
        const idx = list.findIndex((m: Record<string, unknown>) => (m as { id: number }).id === Number(req.messageId));
        if (idx >= 0) list[idx] = { ...list[idx], body: req.body, edited_at: req.editedAt };
        buckets.set(cid, list);
      },
      remove_message: (channelId: bigint, messageId: bigint) => {
        const cid = Number(channelId);
        const mid = Number(messageId);
        buckets.set(cid, (buckets.get(cid) ?? []).filter((m: Record<string, unknown>) => (m as { id: number }).id !== mid));
      },
      get_messages_json: (channelId: bigint) =>
        JSON.stringify({ messages: buckets.get(Number(channelId)) ?? [], has_more: false }),
      increment_unread: (channelId: bigint) => {
        unread[Number(channelId)] = (unread[Number(channelId)] ?? 0) + 1;
      },
      clear_channel_unread: (channelId: bigint) => {
        delete unread[Number(channelId)];
      },
      unread_counts_json: () => JSON.stringify(unread),
    }),
    parseWasmAny: (val: unknown) => (val ? (typeof val === "string" ? JSON.parse(val) : val) : null),
  };
});
vi.mock("@/stores/runner", () => ({
  useRunnerStore: { getState: () => ({ updateRunnerStatus: mockUpdateRunnerStatus }) },
}));
vi.mock("@/stores/ticket", () => ({
  useTicketStore: {
    getState: () => ({
      updateTicketStatusFromEvent: mockUpdateTicketStatus,
      removeTicketFromEvent: mockRemoveTicket,
      fetchTickets: mockFetchTickets,
      fetchTicket: mockFetchTicket,
      currentTicket: null,
    }),
  },
}));

describe("handleChannelEvent", () => {
  // Accessor for the shared WASM-mock unread map seeded in the vi.mock block.
  const wasmUnread = () =>
    (globalThis as unknown as { __channelUnread: Record<number, number> }).__channelUnread;

  beforeEach(() => {
    // Reset shared WASM-mock buckets between tests.
    const g = globalThis as unknown as {
      __channelBuckets?: Map<number, Record<string, unknown>[]>;
      __channelUnread?: Record<number, number>;
    };
    g.__channelBuckets?.clear();
    if (g.__channelUnread) for (const k of Object.keys(g.__channelUnread)) delete g.__channelUnread[Number(k)];
    useChannelMessageStore.setState({ cache: {}, _unreadTick: 0 });
    getAuthManager().apply_session(JSON.stringify({ token: "t", refresh_token: "r", user: { id: 1, email: "u@e.com", username: "u" } }));
    useChannelStore.setState({ selectedChannelId: null } as never);
  });

  describe("channel:message", () => {
    it("adds message to store with body and content", () => {
      const content = { kind: "text", blocks: [{ type: "paragraph", elements: [{ type: "text", text: "hello" }] }] };
      const mentions = { users: [3] };
      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 10, channel_id: 1,
          sender_user_id: 2, sender_name: "alice",
          message_type: "text",
          body: "hello",
          content_json: JSON.stringify(content),
          mentions_json: JSON.stringify(mentions),
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity",
        organization_id: 1,
        entity_type: "channel",
        entity_id: "1",
        timestamp: Date.now(),
      };

      handleChannelEvent(event);

      const view = readMessages(1);
      expect(view.messages).toHaveLength(1);
      expect(view.messages[0].body).toBe("hello");
      expect(view.messages[0].content).toEqual(content);
      expect(view.messages[0].mentions).toEqual(mentions);
    });

    it("includes sender_pod_info when present", () => {
      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 11, channel_id: 1,
          sender_pod: "pk-bot",
          sender_pod_info: { pod_key: "pk-bot", alias: "MyBot", agent: { name: "Claude" } },
          sender_name: "",
          message_type: "text",
          body: "agent message",
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      const msg = readMessages(1).messages[0];
      expect(msg.sender_pod_info).toEqual({ pod_key: "pk-bot", alias: "MyBot", agent: { name: "Claude" } });
    });

    it("constructs sender_user from sender_name", () => {
      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 12, channel_id: 1,
          sender_user_id: 5, sender_name: "bob",
          message_type: "text", body: "hi",
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      const msg = readMessages(1).messages[0];
      expect(msg.sender_user).toEqual({ id: 5, username: "bob", name: "bob" });
    });

    it("increments unread when message is from another user and not viewing", () => {
      getAuthManager().apply_session(JSON.stringify({ token: "t", refresh_token: "r", user: { id: 1, email: "u@e.com", username: "u" } }));
      useChannelStore.setState({ selectedChannelId: 999 } as never);

      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 13, channel_id: 1, sender_user_id: 2,
          sender_name: "",
          message_type: "text", body: "hi",
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      expect(wasmUnread()[1]).toBe(1);
    });

    it("does NOT increment unread for own message", () => {
      getAuthManager().apply_session(JSON.stringify({ token: "t", refresh_token: "r", user: { id: 1, email: "u@e.com", username: "u" } }));

      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 14, channel_id: 1, sender_user_id: 1,
          sender_name: "",
          message_type: "text", body: "self",
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      expect(wasmUnread()[1]).toBeUndefined();
    });

    it("does NOT increment unread when viewing that channel", () => {
      useChannelStore.setState({ selectedChannelId: 1 } as never);

      const event: RealtimeEvent = {
        type: "channel:message",
        data: {
          id: 15, channel_id: 1, sender_user_id: 2,
          sender_name: "",
          message_type: "text", body: "hi",
          created_at: "2024-01-01T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      expect(wasmUnread()[1]).toBeUndefined();
    });
  });

  describe("channel:message_edited", () => {
    it("updates message body and content in store", () => {
      // Seed via addMessage so both WASM mock bucket and store cache are populated.
      useChannelMessageStore.getState().addMessage(1, {
        id: 20, channel_id: 1, body: "old", message_type: "text", created_at: "2024-01-01T00:00:00Z",
      } as never);

      const event: RealtimeEvent = {
        type: "channel:message_edited",
        data: {
          id: 20, channel_id: 1,
          body: "edited",
          content_json: JSON.stringify({ kind: "text", blocks: [{ type: "paragraph", elements: [{ type: "text", text: "edited" }] }] }),
          mentions_json: JSON.stringify({ users: [3] }),
          edited_at: "2024-01-02T00:00:00Z",
        },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      const msg = readMessages(1).messages[0];
      expect(msg.body).toBe("edited");
      expect((msg as { edited_at: string }).edited_at).toBe("2024-01-02T00:00:00Z");
    });
  });

  describe("channel:message_deleted", () => {
    it("removes message from store", () => {
      useChannelMessageStore.getState().addMessage(1, {
        id: 30, channel_id: 1, body: "keep", message_type: "text", created_at: "2024-01-01T00:00:00Z",
      } as never);
      useChannelMessageStore.getState().addMessage(1, {
        id: 31, channel_id: 1, body: "delete", message_type: "text", created_at: "2024-01-01T00:00:00Z",
      } as never);

      const event: RealtimeEvent = {
        type: "channel:message_deleted",
        data: { id: 31, channel_id: 1 },
        category: "entity", organization_id: 1, entity_type: "channel", entity_id: "1", timestamp: Date.now(),
      };

      handleChannelEvent(event);

      const msgs = readMessages(1).messages;
      expect(msgs).toHaveLength(1);
      expect(msgs[0].id).toBe(30);
    });
  });
});

describe("handleInfraEvent", () => {
  const baseEvent = { category: "entity" as const, organization_id: 1, entity_type: "runner", entity_id: "1", timestamp: Date.now() };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("runner:online updates runner status", () => {
    handleInfraEvent({ type: "runner:online", data: { runner_id: 1, node_id: "n1", status: "online" }, ...baseEvent });
    expect(mockUpdateRunnerStatus).toHaveBeenCalledWith(1, "online");
  });

  it("runner:offline updates runner status", () => {
    handleInfraEvent({ type: "runner:offline", data: { runner_id: 2, node_id: "n2", status: "offline" }, ...baseEvent });
    expect(mockUpdateRunnerStatus).toHaveBeenCalledWith(2, "offline");
  });

  it("ticket:status_changed updates ticket status", () => {
    handleInfraEvent({
      type: "ticket:status_changed",
      data: { slug: "DEV-1", status: "in_progress", previous_status: "backlog" },
      ...baseEvent, entity_type: "ticket",
    });
    expect(mockUpdateTicketStatus).toHaveBeenCalledWith("DEV-1", "in_progress", "backlog");
    expect(mockFetchTickets).toHaveBeenCalled();
  });

  it("ticket:deleted removes ticket", () => {
    handleInfraEvent({
      type: "ticket:deleted",
      data: { slug: "DEV-2", status: "" },
      ...baseEvent, entity_type: "ticket",
    });
    expect(mockRemoveTicket).toHaveBeenCalledWith("DEV-2");
  });

  it("ticket:created triggers refetch", () => {
    handleInfraEvent({
      type: "ticket:created",
      data: { slug: "DEV-3", status: "backlog" },
      ...baseEvent, entity_type: "ticket",
    });
    expect(mockFetchTickets).toHaveBeenCalled();
  });

  it("mr:created with ticket_slug fetches ticket", () => {
    handleInfraEvent({
      type: "mr:created",
      data: { mr_id: 1, mr_iid: 1, mr_url: "", source_branch: "feat", state: "open", ticket_slug: "DEV-1", repository_id: 1 },
      ...baseEvent,
    });
    expect(mockFetchTicket).toHaveBeenCalledWith("DEV-1");
  });

  it("mr:created with pod_id fetches pod", () => {
    handleInfraEvent({
      type: "mr:created",
      data: { mr_id: 1, mr_iid: 1, mr_url: "", source_branch: "feat", state: "open", pod_id: 1, repository_id: 1 },
      ...baseEvent,
    });
    expect(mockFetchPod).toHaveBeenCalledWith("pk-1");
  });

  it("pipeline:updated with ticket_slug fetches ticket", () => {
    handleInfraEvent({
      type: "pipeline:updated",
      data: { pipeline_id: 1, pipeline_status: "success", ticket_slug: "DEV-1", repository_id: 1 },
      ...baseEvent,
    });
    expect(mockFetchTicket).toHaveBeenCalledWith("DEV-1");
  });

  it("pipeline:updated with pod_id fetches pod", () => {
    handleInfraEvent({
      type: "pipeline:updated",
      data: { pipeline_id: 1, pipeline_status: "success", pod_id: 1, repository_id: 1 },
      ...baseEvent,
    });
    expect(mockFetchPod).toHaveBeenCalledWith("pk-1");
  });
});
