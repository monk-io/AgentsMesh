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
  SenderUserSchema,
  SenderPodInfoSchema,
  SenderAgentInfoSchema,
  type Channel as ProtoChannel,
  type ChannelMessage as ProtoChannelMessage,
} from "@proto/channel_state/v1/channel_state_pb";
import type { Channel } from "@/stores/channelTypes";
import type { ChannelMessage } from "@/lib/api/facade/channel";
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
