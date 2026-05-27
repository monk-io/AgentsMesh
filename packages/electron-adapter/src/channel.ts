import { invoke } from "./invoke";
import type { IChannelService } from "@agentsmesh/service-interface";
import { ChannelLocalState } from "./channel_state";
import { fromBinary } from "@bufbuild/protobuf";
import {
  ReplaceCachedChannelsRequestSchema,
  InsertChannelRequestSchema,
  PatchChannelMemberCountRequestSchema,
  ReplaceCachedChannelMessagesRequestSchema,
  PrependCachedChannelMessagesRequestSchema,
  InsertChannelMessageRequestSchema,
  ApplyIncomingChannelMessageRequestSchema,
  ApplyChannelMessageEditedEventRequestSchema,
  ReplaceChannelUnreadCountsRequestSchema,
} from "@agentsmesh/proto/channel_state/v1/mutations_pb";
import type {
  Channel as ProtoChannel,
  ChannelMessage as ProtoChannelMessage,
} from "@agentsmesh/proto/channel_state/v1/channel_state_pb";

// Proto -> JS-cache shape converters. The renderer reads from
// `_messagesCache` / `_channelsCache` via channels_json() / get_messages_json().
// Those readers parse the JSON and expect snake_case fields, with
// content/mentions as opaque *_json strings (matches WasmChannelMessage).
function channelToCache(c: ProtoChannel): Record<string, unknown> {
  return {
    id: Number(c.id),
    organization_id: Number(c.organizationId),
    name: c.name,
    description: c.description,
    document: c.document,
    repository_id: c.repositoryId !== undefined ? Number(c.repositoryId) : undefined,
    ticket_id: c.ticketId !== undefined ? Number(c.ticketId) : undefined,
    ticket_slug: c.ticketSlug || undefined,
    visibility: c.visibility,
    is_archived: c.isArchived,
    is_member: c.isMember,
    member_count: Number(c.memberCount),
    agent_count: Number(c.agentCount),
    created_at: c.createdAt,
    updated_at: c.updatedAt,
  };
}

function messageToCache(m: ProtoChannelMessage): Record<string, unknown> {
  return {
    id: Number(m.id),
    channel_id: Number(m.channelId),
    sender_pod: m.senderPod,
    sender_user_id: m.senderUserId !== undefined && m.senderUserId !== BigInt(0)
      ? Number(m.senderUserId) : undefined,
    sender_user: m.senderUser ? {
      id: Number(m.senderUser.id),
      username: m.senderUser.username,
      name: m.senderUser.name,
      avatar_url: m.senderUser.avatarUrl,
    } : undefined,
    sender_pod_info: m.senderPodInfo ? {
      pod_key: m.senderPodInfo.podKey,
      alias: m.senderPodInfo.alias,
      agent: m.senderPodInfo.agent ? { name: m.senderPodInfo.agent.name } : undefined,
    } : undefined,
    message_type: m.messageType,
    body: m.body,
    content_json: m.contentJson || undefined,
    mentions_json: m.mentionsJson || undefined,
    reply_to: m.replyTo !== undefined && m.replyTo !== BigInt(0)
      ? Number(m.replyTo) : undefined,
    edited_at: m.editedAt || undefined,
    is_deleted: m.isDeleted,
    created_at: m.createdAt,
  };
}

export class ElectronChannelService extends ChannelLocalState implements IChannelService {
  async fetch_channels(includeArchived?: boolean | null): Promise<string> {
    const result = await invoke<string>("channelFetchChannels", includeArchived);
    try {
      const parsed = JSON.parse(result) as { channels?: unknown[] };
      this._channelsCache = JSON.stringify(Array.isArray(parsed.channels) ? parsed.channels : parsed);
    } catch {
      this._channelsCache = "[]";
    }
    return result;
  }

  async fetch_channel(id: bigint): Promise<string> {
    const result = await invoke<string>("channelFetchChannel", Number(id));
    this.update_channel_local(id, result);
    return result;
  }

  async fetch_messages(channelId: bigint, limit?: number | null, beforeId?: bigint | null): Promise<string> {
    const result = await invoke<string>(
      "channelFetchMessages",
      Number(channelId),
      limit,
      beforeId ? Number(beforeId) : null,
    );
    const parsed = JSON.parse(result) as { messages?: unknown[]; has_more?: boolean };
    this._messagesCache.set(String(channelId), {
      messages: Array.isArray(parsed.messages) ? parsed.messages : [],
      has_more: parsed.has_more ?? false,
    });
    return result;
  }

  async fetch_unread_counts(): Promise<string> {
    const result = await invoke<string>("channelFetchUnreadCounts");
    this._unreadCountsCache = result;
    return result;
  }

  async create_channel(json: string): Promise<string> {
    const result = await invoke<string>("channelCreateChannel", json);
    this.add_channel_local(result);
    return result;
  }

  async update_channel(id: bigint, json: string): Promise<string> {
    const result = await invoke<string>("channelUpdateChannel", Number(id), json);
    this.update_channel_local(id, result);
    return result;
  }

  async archive_channel(id: bigint): Promise<void> {
    await invoke<void>("channelArchiveChannel", Number(id));
  }

  async unarchive_channel(id: bigint): Promise<void> {
    await invoke<void>("channelUnarchiveChannel", Number(id));
  }

  async send_message(channelId: bigint, json: string): Promise<string> {
    const result = await invoke<string>("channelSendMessage", Number(channelId), json);
    this.add_message(channelId, result);
    return result;
  }

  async edit_message(channelId: bigint, messageId: bigint, content: string): Promise<string> {
    const result = await invoke<string>(
      "channelEditMessage",
      Number(channelId),
      Number(messageId),
      content,
    );
    this.update_message_local(channelId, result);
    return result;
  }

  async delete_message(channelId: bigint, messageId: bigint): Promise<void> {
    await invoke<void>("channelDeleteMessage", Number(channelId), Number(messageId));
    this.remove_message_local(channelId, messageId);
  }

  async mark_read(channelId: bigint, messageId: bigint): Promise<void> {
    await invoke<void>("channelMarkRead", Number(channelId), Number(messageId));
    this.clear_channel_unread(channelId);
  }

  async mute_channel(channelId: bigint, muted: boolean): Promise<void> {
    await invoke<void>("channelMuteChannel", Number(channelId), muted);
  }

  async join_channel(channelId: bigint, podKey: string): Promise<string> {
    const result = await invoke<string>("channelJoinChannel", Number(channelId), podKey);
    await this.get_channel_pods(channelId).catch(() => undefined);
    return result;
  }

  async leave_channel(channelId: bigint, podKey: string): Promise<string> {
    const result = await invoke<string>("channelLeaveChannel", Number(channelId), podKey);
    await this.get_channel_pods(channelId).catch(() => undefined);
    return result;
  }

  async get_channel_pods(id: bigint): Promise<string> {
    const result = await invoke<string>("channelGetChannelPods", Number(id));
    try {
      const parsed = JSON.parse(result) as { pods?: unknown[] };
      this.set_channel_pods(id, JSON.stringify(Array.isArray(parsed.pods) ? parsed.pods : []));
    } catch {
      this.set_channel_pods(id, "[]");
    }
    return result;
  }

  // Proto-bytes mutators decode locally into the JS-side cache so synchronous
  // readers (channels_json / get_messages_json / unread_counts_json) see the
  // mutation immediately. NAPI forwarding is fire-and-forget — present so the
  // main-process Rust ChannelService stays in sync for future consumers, but
  // not awaited because IPC latency would defeat the sync-cache invariant
  // the renderer's _tick reactivity model assumes.
  replace_cached_channels(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(ReplaceCachedChannelsRequestSchema, reqBytes);
    this._channelsCache = JSON.stringify(req.channels.map(channelToCache));
    void invoke<void>("channelReplaceCachedChannels", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  insert_channel(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(InsertChannelRequestSchema, reqBytes);
    if (req.channel) {
      const c = channelToCache(req.channel);
      const list = JSON.parse(this._channelsCache) as { id: number }[];
      const idx = list.findIndex((x) => x.id === c.id);
      if (idx >= 0) list[idx] = { ...list[idx], ...c };
      else list.unshift(c as { id: number });
      this._channelsCache = JSON.stringify(list);
    }
    void invoke<void>("channelInsertChannel", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  patch_channel_member_count(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(PatchChannelMemberCountRequestSchema, reqBytes);
    const id = Number(req.channelId);
    const list = JSON.parse(this._channelsCache) as { id: number; member_count?: number }[];
    const idx = list.findIndex((x) => x.id === id);
    if (idx >= 0) {
      list[idx].member_count = Math.max(0, (list[idx].member_count ?? 0) + req.delta);
      this._channelsCache = JSON.stringify(list);
    }
    void invoke<void>("channelPatchChannelMemberCount", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  replace_cached_channel_messages(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(ReplaceCachedChannelMessagesRequestSchema, reqBytes);
    this._messagesCache.set(String(req.channelId), {
      messages: req.messages.map(messageToCache),
      has_more: req.hasMore,
    });
    void invoke<void>("channelReplaceCachedChannelMessages", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  prepend_cached_channel_messages(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(PrependCachedChannelMessagesRequestSchema, reqBytes);
    const key = String(req.channelId);
    const entry = this._messagesCache.get(key) ?? { messages: [], has_more: false };
    const older = req.messages.map(messageToCache);
    const existingIds = new Set((entry.messages as { id: number }[]).map((m) => m.id));
    const merged = [...older.filter((m) => !existingIds.has(m.id as number)), ...entry.messages];
    this._messagesCache.set(key, { messages: merged, has_more: req.hasMore });
    void invoke<void>("channelPrependCachedChannelMessages", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  insert_channel_message(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(InsertChannelMessageRequestSchema, reqBytes);
    if (req.message) {
      const key = String(req.channelId);
      const entry = this._messagesCache.get(key) ?? { messages: [], has_more: false };
      const msg = messageToCache(req.message);
      if (!entry.messages.some((m) => (m as { id: number }).id === msg.id)) {
        entry.messages.push(msg);
      }
      this._messagesCache.set(key, entry);
    }
    void invoke<void>("channelInsertChannelMessage", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  apply_incoming_channel_message(reqBytes: Uint8Array): Promise<boolean> {
    const req = fromBinary(ApplyIncomingChannelMessageRequestSchema, reqBytes);
    if (!req.message) return Promise.resolve(false);
    const key = String(req.channelId);
    const entry = this._messagesCache.get(key) ?? { messages: [], has_more: false };
    const msg = messageToCache(req.message);
    const dup = entry.messages.some((m) => (m as { id: number }).id === msg.id);
    if (!dup) {
      entry.messages.push(msg);
      this._messagesCache.set(key, entry);
    }
    void invoke<void>("channelApplyIncomingChannelMessage", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve(!dup);
  }

  apply_channel_message_edited_event(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(ApplyChannelMessageEditedEventRequestSchema, reqBytes);
    const key = String(req.channelId);
    const entry = this._messagesCache.get(key);
    if (entry) {
      const idx = entry.messages.findIndex((m) => (m as { id: number }).id === Number(req.messageId));
      if (idx >= 0) {
        const cur = entry.messages[idx] as Record<string, unknown>;
        entry.messages[idx] = {
          ...cur,
          body: req.body,
          content_json: req.content || undefined,
          edited_at: req.editedAt || undefined,
        };
        this._messagesCache.set(key, entry);
      }
    }
    void invoke<void>("channelApplyChannelMessageEditedEvent", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  replace_channel_unread_counts(reqBytes: Uint8Array): Promise<void> {
    const req = fromBinary(ReplaceChannelUnreadCountsRequestSchema, reqBytes);
    const out: Record<string, number> = {};
    for (const [k, v] of Object.entries(req.counts)) {
      out[String(k)] = Number(v);
    }
    this._unreadCountsCache = JSON.stringify(out);
    void invoke<void>("channelReplaceChannelUnreadCounts", Array.from(reqBytes)).catch(() => undefined);
    return Promise.resolve();
  }

  remove_message(channelId: bigint, messageId: bigint): void {
    void invoke<void>("channelRemoveMessage", Number(channelId), Number(messageId)).catch(() => undefined);
    super.remove_message_local(channelId, messageId);
  }
}
