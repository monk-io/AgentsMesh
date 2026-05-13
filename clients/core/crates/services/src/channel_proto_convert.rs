// proto.channel.v1 ↔ legacy serde type conversions for ChannelService.
//
// services::ChannelService keeps the legacy JSON-shaped state contract
// (HashMap<i64, Channel>, Vec<ChannelMessage>, etc.) so wasm + node-bridge
// callers stay untouched. The network lane runs on Connect-RPC, so we
// map proto records onto the legacy shapes at the boundary.
//
// `content` / `mentions` ride as opaque JSON strings on the proto wire
// (`content_json` / `mentions_json`) — see conventions §2.5.

use std::collections::HashMap;

use agentsmesh_types::proto_channel_v1 as channel_proto;
use agentsmesh_types::{
    Channel, ChannelListResponse, ChannelMember, ChannelMemberListResponse,
    ChannelMessage, ChannelMessageListResponse, EditChannelMessageRequest,
    SendChannelMessageRequest, SenderAgentInfo, SenderPodInfo, User,
};
use serde_json::Value;

pub(crate) fn channel_from_proto(c: channel_proto::Channel) -> Channel {
    Channel {
        id: c.id,
        name: c.name,
        description: c.description,
        is_archived: c.is_archived,
        visibility: Some(c.visibility),
        is_member: c.is_member,
        member_count: Some(c.member_count),
        organization_id: Some(c.organization_id),
        document: c.document,
        repository_id: c.repository_id,
        ticket_id: c.ticket_id,
        ticket_slug: c.ticket_slug,
        created_by_pod: c.created_by_pod,
        created_by_user_id: c.created_by_user_id,
        created_at: Some(c.created_at),
        updated_at: Some(c.updated_at),
    }
}

pub(crate) fn message_from_proto(m: channel_proto::ChannelMessage) -> ChannelMessage {
    ChannelMessage {
        id: m.id,
        channel_id: m.channel_id,
        body: m.body,
        content: m.content_json.as_deref().and_then(parse_value_opt),
        mentions: m.mentions_json.as_deref().and_then(parse_value_opt),
        reply_to: m.reply_to,
        sender_user: m.sender_user.map(|u| User {
            id: u.id,
            email: String::new(),
            username: u.username,
            name: u.name,
            avatar_url: u.avatar_url,
            is_email_verified: None,
        }),
        sender_user_id: m.sender_user_id,
        sender_pod: m.sender_pod.clone(),
        sender_pod_info: m.sender_pod_info.map(|p| SenderPodInfo {
            pod_key: p.pod_key,
            alias: p.alias,
            agent: p.agent.map(|a| SenderAgentInfo { name: a.name }),
        }),
        message_type: Some(m.message_type),
        pod_key: m.sender_pod,
        metadata: None,
        edited_at: m.edited_at,
        is_deleted: Some(m.is_deleted),
        created_at: Some(m.created_at),
    }
}

pub(crate) fn member_from_proto(m: channel_proto::ChannelMember) -> ChannelMember {
    ChannelMember {
        channel_id: m.channel_id,
        user_id: m.user_id,
        role: m.role,
        is_muted: m.is_muted,
        joined_at: m.joined_at,
    }
}

pub(crate) fn channel_list_from_proto(resp: channel_proto::ListChannelsResponse) -> ChannelListResponse {
    ChannelListResponse {
        channels: resp.items.into_iter().map(channel_from_proto).collect(),
        total: Some(resp.total),
    }
}

pub(crate) fn message_list_from_proto(
    resp: channel_proto::ListChannelMessagesResponse,
) -> ChannelMessageListResponse {
    ChannelMessageListResponse {
        messages: resp.items.into_iter().map(message_from_proto).collect(),
        has_more: resp.has_more,
    }
}

pub(crate) fn member_list_from_proto(
    resp: channel_proto::ListChannelMembersResponse,
) -> ChannelMemberListResponse {
    ChannelMemberListResponse {
        members: resp.items.into_iter().map(member_from_proto).collect(),
        total: resp.total,
    }
}

pub(crate) fn pod_list_from_proto(
    resp: channel_proto::ListChannelPodsResponse,
) -> serde_json::Value {
    serde_json::json!({
        "pods": resp.items.iter().map(|p| serde_json::json!({
            "id": p.id,
            "pod_key": p.pod_key,
            "alias": p.alias,
            "status": p.status,
            "agent_status": p.agent_status,
        })).collect::<Vec<_>>(),
        "total": resp.total,
    })
}

fn parse_value_opt(s: &str) -> Option<Value> {
    if s.is_empty() { return None; }
    serde_json::from_str(s).ok()
}

pub(crate) fn send_request_to_proto(
    org_slug: String, channel_id: i64, env: SendChannelMessageRequest,
) -> channel_proto::SendChannelMessageRequest {
    channel_proto::SendChannelMessageRequest {
        org_slug,
        channel_id,
        source: env.source,
        mentions: mentions_to_proto(env.mentions),
        content_json: env.content.as_ref().map(|v| v.to_string()),
        attachment_key: env.attachment_key,
        pod_key: env.pod_key,
        reply_to: env.reply_to,
    }
}

pub(crate) fn edit_request_to_proto(
    org_slug: String, channel_id: i64, message_id: i64, env: EditChannelMessageRequest,
) -> channel_proto::EditChannelMessageRequest {
    channel_proto::EditChannelMessageRequest {
        org_slug,
        channel_id,
        message_id,
        source: env.source,
        mentions: mentions_to_proto(env.mentions),
        content_json: env.content.as_ref().map(|v| v.to_string()),
        attachment_key: None,
    }
}

fn mentions_to_proto(
    raw: Option<serde_json::Value>,
) -> HashMap<String, channel_proto::MentionRef> {
    let Some(value) = raw else { return HashMap::new(); };
    let Some(map) = value.as_object() else { return HashMap::new(); };
    map.iter()
        .filter_map(|(k, v)| {
            let obj = v.as_object()?;
            let entity_type = obj.get("entity_type").and_then(|x| x.as_str())?.to_string();
            let entity_key = obj.get("entity_key").and_then(|x| x.as_str())?.to_string();
            Some((k.clone(), channel_proto::MentionRef { entity_type, entity_key }))
        })
        .collect()
}
