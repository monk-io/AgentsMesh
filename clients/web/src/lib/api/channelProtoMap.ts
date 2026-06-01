// Channel (web snake_case shape) ↔ proto.channel_state.v1.* (camelCase).
// Mirror of podProtoMap.ts — denormalises renderer types into the proto
// state mutation contract so the wasm bridge can decode well-typed
// messages via prost.
//
// Used by channelStore.ts / channelMessageStore.ts to encode mutation
// requests as binary bytes.

import { create as protoCreate } from "@bufbuild/protobuf";
import {
  ChannelSchema,
  ChannelMessageSchema,
  ChannelMemberSchema,
  SenderUserSchema,
  SenderPodInfoSchema,
  SenderAgentInfoSchema,
  type Channel as ProtoChannel,
  type ChannelMessage as ProtoChannelMessage,
  type ChannelMember as ProtoChannelMember,
} from "@proto/channel_state/v1/channel_state_pb";
import { PodSchema, type Pod as ProtoPod } from "@proto/pod/v1/pod_pb";
import type { Channel } from "@/stores/channelTypes";
import type {
  ChannelMessage,
  ChannelData,
} from "@/lib/api/facade/channel";
import type {
  ChannelMemberData,
  ChannelPodSummary,
} from "@/lib/api/connect/channelMembersConnect";
import { toWasmProjection } from "@/stores/channelMessageWasmProjection";

function asBigInt(v: number | undefined | null): bigint | undefined {
  return v === undefined || v === null ? undefined : BigInt(v);
}

export function channelToProtoChannel(c: Channel): ProtoChannel {
  return protoCreate(ChannelSchema, {
    id: asBigInt(c.id) ?? BigInt(0),
    organizationId: asBigInt(c.organization_id),
    name: c.name,
    description: c.description,
    document: c.document,
    ticketSlug: c.ticket?.slug,
    ticketId: asBigInt(c.ticket?.id),
    repositoryId: asBigInt(c.repository?.id),
    visibility: c.visibility,
    isArchived: c.is_archived,
    isMember: c.is_member ?? false,
    memberCount: asBigInt(c.member_count),
    agentCount: asBigInt(c.agent_count),
    createdAt: c.created_at,
    updatedAt: c.updated_at,
  });
}

// channelApi facade response shape (returned by Connect updateChannel etc).
// Distinct from store Channel — exposes repository_id / ticket_id as scalars
// instead of nested objects, so the mapping is flatter.
export function channelDataToProtoChannel(c: ChannelData): ProtoChannel {
  return protoCreate(ChannelSchema, {
    id: asBigInt(c.id) ?? BigInt(0),
    organizationId: asBigInt(c.organization_id),
    name: c.name,
    description: c.description,
    document: c.document,
    repositoryId: asBigInt(c.repository_id),
    ticketId: asBigInt(c.ticket_id),
    ticketSlug: c.ticket_slug,
    createdByPod: c.created_by_pod,
    createdByUserId: asBigInt(c.created_by_user_id),
    visibility: c.visibility,
    isArchived: c.is_archived,
    isMember: c.is_member,
    memberCount: asBigInt(c.member_count),
    agentCount: asBigInt(c.agent_count),
    createdAt: c.created_at,
    updatedAt: c.updated_at,
  });
}

// Wire-level Pod with only the 5 summary fields populated — the channel
// cache stores Vec<proto.pod.v1.Pod> but the renderer only reads the
// summary projection (id/pod_key/alias/status/agent_status). Other Pod
// fields default to their proto3 zero values.
export function channelPodSummaryToProtoPod(p: ChannelPodSummary): ProtoPod {
  return protoCreate(PodSchema, {
    id: asBigInt(p.id) ?? BigInt(0),
    podKey: p.pod_key,
    alias: p.alias,
    status: p.status,
    agentStatus: p.agent_status,
  });
}

export function channelMemberDataToProto(m: ChannelMemberData): ProtoChannelMember {
  return protoCreate(ChannelMemberSchema, {
    channelId: asBigInt(m.channel_id) ?? BigInt(0),
    userId: asBigInt(m.user_id) ?? BigInt(0),
    role: m.role,
    isMuted: m.is_muted,
    joinedAt: m.joined_at,
  });
}

export function channelMessageToProto(m: ChannelMessage): ProtoChannelMessage {
  // Use the existing wasm projection to fold rich `content` / `mentions`
  // ASTs into the *_json string fields the proto carries.
  const projected = toWasmProjection(m);
  return protoCreate(ChannelMessageSchema, {
    id: asBigInt(projected.id) ?? BigInt(0),
    channelId: asBigInt(projected.channel_id) ?? BigInt(0),
    senderPod: projected.sender_pod,
    senderPodInfo: projected.sender_pod_info ? protoCreate(SenderPodInfoSchema, {
      podKey: projected.sender_pod_info.pod_key,
      alias: projected.sender_pod_info.alias,
      agent: projected.sender_pod_info.agent ? protoCreate(SenderAgentInfoSchema, {
        name: projected.sender_pod_info.agent.name,
      }) : undefined,
    }) : undefined,
    senderUserId: asBigInt(projected.sender_user_id),
    senderUser: projected.sender_user ? protoCreate(SenderUserSchema, {
      id: asBigInt(projected.sender_user.id) ?? BigInt(0),
      username: projected.sender_user.username,
      name: projected.sender_user.name,
      avatarUrl: projected.sender_user.avatar_url,
      email: "",
    }) : undefined,
    body: projected.body,
    contentJson: projected.content_json,
    mentionsJson: projected.mentions_json,
    replyTo: asBigInt(projected.reply_to),
    messageType: projected.message_type,
    createdAt: projected.created_at,
    editedAt: projected.edited_at,
    isDeleted: projected.is_deleted,
  });
}
