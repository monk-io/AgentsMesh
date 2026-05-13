use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::channel_state::ChannelState;
use agentsmesh_types::proto_channel_v1 as channel_proto;
use agentsmesh_types::{
    Channel, ChannelMessage, EditChannelMessageRequest, SendChannelMessageRequest,
};

use crate::channel_proto_convert::{
    channel_from_proto, channel_list_from_proto, edit_request_to_proto,
    member_list_from_proto, message_from_proto, message_list_from_proto,
    pod_list_from_proto, send_request_to_proto,
};

pub struct ChannelService {
    client: Arc<ApiClient>,
    state: RwLock<ChannelState>,
}

impl ChannelService {
    pub fn new(client: Arc<ApiClient>, state: ChannelState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    /// Crate-local accessor used by channel_connect.rs to forward to the
    /// underlying api-client `*_connect` methods.
    pub(crate) fn client(&self) -> &ApiClient { &self.client }

    fn org_slug(&self) -> String { self.client.current_org_slug() }

    pub fn channels_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_channels()).unwrap_or_default()
    }

    pub fn current_channel_json(&self) -> Option<String> {
        self.state.read().unwrap().get_current_channel()
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn get_channel_json(&self, id: i64) -> Option<String> {
        self.state.read().unwrap().get_channel(id)
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn filter_channels_json(&self, query: &str, include_archived: bool) -> String {
        let binding = self.state.read().unwrap();
        let filtered = binding.filter_channels(query, include_archived);
        serde_json::to_string(&filtered).unwrap_or_default()
    }

    pub fn get_messages_json(&self, channel_id: i64) -> Option<String> {
        self.state.read().unwrap().get_messages(channel_id).map(|cache| {
            let result = serde_json::json!({
                "messages": cache.messages,
                "has_more": cache.has_more,
            });
            serde_json::to_string(&result).unwrap_or_default()
        })
    }

    pub fn get_unread_count(&self, channel_id: i64) -> u32 {
        self.state.read().unwrap().get_unread_count(channel_id)
    }

    pub fn total_unread_count(&self) -> u32 {
        self.state.read().unwrap().total_unread_count()
    }

    pub fn unread_counts_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_all_unread_counts()).unwrap_or_default()
    }

    pub fn get_mention_count(&self, channel_id: i64) -> u32 {
        self.state.read().unwrap().get_mention_count(channel_id)
    }

    pub fn total_mention_count(&self) -> u32 {
        self.state.read().unwrap().total_mention_count()
    }

    pub fn mention_counts_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_all_mention_counts()).unwrap_or_default()
    }

    pub fn sorted_channel_ids_json(&self, mode: &str, include_archived: bool) -> String {
        use agentsmesh_state::channel_state::ChannelSortMode;
        let sort_mode = match mode {
            "unread_first" => ChannelSortMode::UnreadFirst,
            "name" => ChannelSortMode::Name,
            _ => ChannelSortMode::LastMessage,
        };
        let ids = self.state.read().unwrap().sorted_channel_ids(sort_mode, include_archived);
        serde_json::to_string(&ids).unwrap_or_default()
    }

    pub fn get_last_message_json(&self, channel_id: i64) -> Option<String> {
        self.state.read().unwrap().get_last_message(channel_id)
            .map(|p| serde_json::to_string(p).unwrap_or_default())
    }

    pub fn set_channels(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<Channel>>(json) {
            self.state.write().unwrap().set_channels(v);
        }
    }

    pub fn set_current_channel(&self, id: Option<i64>) {
        self.state.write().unwrap().set_current_channel(id);
    }

    pub fn select_channel(&self, id: Option<i64>) -> Option<String> {
        self.state.write().unwrap().select_channel(id)
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn add_channel_local(&self, json: &str) {
        if let Ok(c) = serde_json::from_str::<Channel>(json) {
            self.state.write().unwrap().add_channel(c);
        }
    }

    pub fn update_channel_local(&self, id: i64, json: &str) {
        if let Ok(c) = serde_json::from_str::<Channel>(json) {
            self.state.write().unwrap().update_channel(id, c);
        }
    }

    pub fn remove_channel_local(&self, id: i64) {
        self.state.write().unwrap().remove_channel(id);
    }

    pub fn set_current_user(&self, user_json: &str) {
        if let Ok(u) = serde_json::from_str(user_json) {
            self.state.write().unwrap().set_current_user(Some(u));
        }
    }

    pub fn set_current_user_id(&self, user_id: Option<i64>) {
        self.state.write().unwrap().set_current_user_id(user_id);
    }

    pub fn set_messages(&self, channel_id: i64, json: &str, has_more: bool) {
        if let Ok(msgs) = serde_json::from_str::<Vec<ChannelMessage>>(json) {
            self.state.write().unwrap().set_messages(channel_id, msgs, has_more);
        }
    }

    pub fn prepend_messages(&self, channel_id: i64, json: &str, has_more: bool) {
        if let Ok(msgs) = serde_json::from_str::<Vec<ChannelMessage>>(json) {
            self.state.write().unwrap().prepend_messages(channel_id, msgs, has_more);
        }
    }

    pub fn add_message(&self, channel_id: i64, json: &str) {
        if let Ok(msg) = serde_json::from_str::<ChannelMessage>(json) {
            self.state.write().unwrap().add_message(channel_id, msg);
        }
    }

    pub fn on_new_message(&self, json: &str) -> bool {
        match serde_json::from_str::<ChannelMessage>(json) {
            Ok(msg) => self.state.write().unwrap().on_new_message(msg),
            Err(_) => false,
        }
    }

    pub fn update_message_local(&self, channel_id: i64, json: &str) {
        if let Ok(msg) = serde_json::from_str::<ChannelMessage>(json) {
            self.state.write().unwrap().update_message(channel_id, msg);
        }
    }

    pub fn remove_message_local(&self, channel_id: i64, message_id: i64) {
        self.state.write().unwrap().remove_message(channel_id, message_id);
    }

    pub fn set_unread_counts(&self, json: &str) {
        if let Ok(counts) = serde_json::from_str(json) {
            self.state.write().unwrap().set_unread_counts(counts);
        }
    }

    pub fn increment_unread(&self, channel_id: i64) {
        self.state.write().unwrap().increment_unread(channel_id);
    }

    pub fn clear_channel_unread(&self, channel_id: i64) {
        self.state.write().unwrap().clear_channel_unread(channel_id);
    }

    pub fn set_mention_counts(&self, json: &str) {
        if let Ok(counts) = serde_json::from_str(json) {
            self.state.write().unwrap().set_mention_counts(counts);
        }
    }

    pub fn increment_mention(&self, channel_id: i64) {
        self.state.write().unwrap().increment_mention(channel_id);
    }

    pub fn clear_channel_mentions(&self, channel_id: i64) {
        self.state.write().unwrap().clear_channel_mentions(channel_id);
    }

    pub fn set_last_message(&self, channel_id: i64, json: &str) {
        if let Ok(p) = serde_json::from_str(json) {
            self.state.write().unwrap().set_last_message(channel_id, p);
        }
    }

    pub async fn fetch_channels(&self, include_archived: Option<bool>) -> Result<String, String> {
        let req = channel_proto::ListChannelsRequest {
            org_slug: self.org_slug(),
            include_archived,
            ..Default::default()
        };
        let resp = self.client.list_channels_connect(&req).await.map_err(crate::wire)?;
        let list = channel_list_from_proto(resp);
        self.state.write().unwrap().set_channels(list.channels.clone());
        serde_json::to_string(&list).map_err(crate::wire)
    }

    pub async fn fetch_channel(&self, id: i64) -> Result<String, String> {
        let req = channel_proto::GetChannelRequest { org_slug: self.org_slug(), id };
        let resp = self.client.get_channel_connect(&req).await.map_err(crate::wire)?;
        let ch = channel_from_proto(resp);
        self.state.write().unwrap().update_channel(id, ch.clone());
        serde_json::to_string(&ch).map_err(crate::wire)
    }

    pub async fn create_channel(&self, request_json: &str) -> Result<String, String> {
        let raw: serde_json::Value = serde_json::from_str(request_json).map_err(crate::wire)?;
        let req = channel_proto::CreateChannelRequest {
            org_slug: self.org_slug(),
            name: raw.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            description: raw.get("description").and_then(|v| v.as_str()).map(String::from),
            document: raw.get("document").and_then(|v| v.as_str()).map(String::from),
            repository_id: raw.get("repository_id").and_then(|v| v.as_i64()),
            ticket_slug: raw.get("ticket_slug").and_then(|v| v.as_str()).map(String::from),
            visibility: raw.get("visibility").and_then(|v| v.as_str()).map(String::from),
            member_ids: Vec::new(),
        };
        let resp = self.client.create_channel_connect(&req).await.map_err(crate::wire)?;
        let ch = channel_from_proto(resp);
        self.state.write().unwrap().add_channel(ch.clone());
        serde_json::to_string(&ch).map_err(crate::wire)
    }

    pub async fn update_channel(&self, id: i64, request_json: &str) -> Result<String, String> {
        let raw: serde_json::Value = serde_json::from_str(request_json).map_err(crate::wire)?;
        let req = channel_proto::UpdateChannelRequest {
            org_slug: self.org_slug(),
            id,
            name: raw.get("name").and_then(|v| v.as_str()).map(String::from),
            description: raw.get("description").and_then(|v| v.as_str()).map(String::from),
            document: raw.get("document").and_then(|v| v.as_str()).map(String::from),
        };
        let resp = self.client.update_channel_connect(&req).await.map_err(crate::wire)?;
        let ch = channel_from_proto(resp);
        self.state.write().unwrap().update_channel(id, ch.clone());
        serde_json::to_string(&ch).map_err(crate::wire)
    }

    pub async fn archive_channel(&self, id: i64) -> Result<(), String> {
        let req = channel_proto::ArchiveChannelRequest { org_slug: self.org_slug(), id };
        self.client.archive_channel_connect(&req).await.map_err(crate::wire)?;
        if let Some(ch) = self.state.read().unwrap().get_channel(id).cloned() {
            let mut updated = ch;
            updated.is_archived = true;
            self.state.write().unwrap().update_channel(id, updated);
        }
        Ok(())
    }

    pub async fn unarchive_channel(&self, id: i64) -> Result<(), String> {
        let req = channel_proto::UnarchiveChannelRequest { org_slug: self.org_slug(), id };
        self.client.unarchive_channel_connect(&req).await.map_err(crate::wire)?;
        if let Some(ch) = self.state.read().unwrap().get_channel(id).cloned() {
            let mut updated = ch;
            updated.is_archived = false;
            self.state.write().unwrap().update_channel(id, updated);
        }
        Ok(())
    }

    pub async fn join_channel(&self, channel_id: i64, pod_key: &str) -> Result<String, String> {
        let req = channel_proto::JoinChannelPodRequest {
            org_slug: self.org_slug(), id: channel_id, pod_key: pod_key.to_string(),
        };
        self.client.join_channel_pod_connect(&req).await.map_err(crate::wire)?;
        self.fetch_channel(channel_id).await
    }

    pub async fn leave_channel(&self, channel_id: i64, pod_key: &str) -> Result<String, String> {
        let req = channel_proto::LeaveChannelPodRequest {
            org_slug: self.org_slug(), id: channel_id, pod_key: pod_key.to_string(),
        };
        self.client.leave_channel_pod_connect(&req).await.map_err(crate::wire)?;
        self.fetch_channel(channel_id).await
    }

    pub async fn fetch_messages(
        &self, channel_id: i64, limit: Option<u32>, before_id: Option<i64>,
    ) -> Result<String, String> {
        let req = channel_proto::ListChannelMessagesRequest {
            org_slug: self.org_slug(),
            channel_id,
            before_id,
            limit: limit.map(|v| v as i32),
        };
        let resp = self.client.list_channel_messages_connect(&req).await.map_err(crate::wire)?;
        let list = message_list_from_proto(resp);
        if before_id.is_some() {
            self.state.write().unwrap().prepend_messages(channel_id, list.messages.clone(), list.has_more);
        } else {
            self.state.write().unwrap().set_messages(channel_id, list.messages.clone(), list.has_more);
        }
        serde_json::to_string(&list).map_err(crate::wire)
    }

    pub async fn send_message(&self, channel_id: i64, request_json: &str) -> Result<String, String> {
        let value: serde_json::Value = serde_json::from_str(request_json).map_err(crate::wire)?;
        let envelope = if request_has_new_shape(&value) {
            serde_json::from_value::<SendChannelMessageRequest>(value).map_err(crate::wire)?
        } else {
            SendChannelMessageRequest { content: Some(value), ..Default::default() }
        };
        let req = send_request_to_proto(self.org_slug(), channel_id, envelope);
        let resp = self.client.send_channel_message_connect(&req).await.map_err(crate::wire)?;
        let msg = message_from_proto(resp);
        self.state.write().unwrap().on_new_message(msg.clone());
        serde_json::to_string(&msg).map_err(crate::wire)
    }

    /// Edit a message. `request_json` is the JSON of either:
    ///   - the new `EditChannelMessageRequest` (`{source}`, `{content}`, or
    ///     `{source, mentions}`), or
    ///   - a bare `MessageContent` AST (legacy callers) which is rewrapped
    ///     into `{content: <ast>}`. Shape is detected structurally.
    pub async fn edit_message(
        &self, channel_id: i64, message_id: i64, request_json: &str,
    ) -> Result<String, String> {
        let value: serde_json::Value = serde_json::from_str(request_json)
            .map_err(|e| format!("invalid edit request JSON: {e}"))?;
        let envelope = if request_has_new_shape(&value) {
            serde_json::from_value::<EditChannelMessageRequest>(value)
                .map_err(|e| format!("invalid edit request JSON: {e}"))?
        } else {
            EditChannelMessageRequest { content: Some(value), ..Default::default() }
        };
        let req = edit_request_to_proto(self.org_slug(), channel_id, message_id, envelope);
        let resp = self.client.edit_channel_message_connect(&req).await.map_err(crate::wire)?;
        let msg = message_from_proto(resp);
        self.state.write().unwrap().update_message(channel_id, msg.clone());
        serde_json::to_string(&msg).map_err(crate::wire)
    }

    pub async fn delete_message(&self, channel_id: i64, message_id: i64) -> Result<(), String> {
        let req = channel_proto::DeleteChannelMessageRequest {
            org_slug: self.org_slug(), channel_id, message_id,
        };
        self.client.delete_channel_message_connect(&req).await.map_err(crate::wire)?;
        self.state.write().unwrap().remove_message(channel_id, message_id);
        Ok(())
    }

    pub async fn fetch_unread_counts(&self) -> Result<String, String> {
        let req = channel_proto::GetChannelUnreadCountsRequest { org_slug: self.org_slug() };
        let resp = self.client.get_channel_unread_counts_connect(&req).await.map_err(crate::wire)?;
        let counts: HashMap<i64, u32> = resp.unread
            .into_iter()
            .filter_map(|(k, v)| k.parse::<i64>().ok().map(|id| (id, v as u32)))
            .collect();
        self.state.write().unwrap().set_unread_counts(counts);
        serde_json::to_string(self.state.read().unwrap().get_all_unread_counts()).map_err(crate::wire)
    }

    pub async fn mark_read(&self, channel_id: i64, message_id: i64) -> Result<(), String> {
        let req = channel_proto::MarkChannelReadRequest {
            org_slug: self.org_slug(), channel_id, message_id,
        };
        self.client.mark_channel_read_connect(&req).await.map_err(crate::wire)?;
        self.state.write().unwrap().clear_channel_unread(channel_id);
        Ok(())
    }

    pub async fn mute_channel(&self, channel_id: i64, muted: bool) -> Result<(), String> {
        let req = channel_proto::MuteChannelRequest {
            org_slug: self.org_slug(), id: channel_id, muted,
        };
        self.client.mute_channel_connect(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn get_channel_pods(&self, id: i64) -> Result<String, String> {
        let req = channel_proto::ListChannelPodsRequest { org_slug: self.org_slug(), id };
        let resp = self.client.list_channel_pods_connect(&req).await.map_err(crate::wire)?;
        let pods: Vec<agentsmesh_types::Pod> = resp.items.iter().map(|p| {
            agentsmesh_types::Pod {
                id: Some(p.id),
                key: p.pod_key.clone(),
                alias: p.alias.clone(),
                agent_status: Some(p.agent_status.clone()),
                status: serde_json::from_value(serde_json::Value::String(p.status.clone()))
                    .unwrap_or_default(),
                ..Default::default()
            }
        }).collect();
        self.state.write().unwrap().set_channel_pods(id, pods);
        serde_json::to_string(&pod_list_from_proto(resp)).map_err(crate::wire)
    }

    pub fn channel_pods_json(&self, id: i64) -> String {
        let pods = self.state.read().unwrap().get_channel_pods(id);
        serde_json::to_string(&pods).unwrap_or_else(|_| "[]".into())
    }

    pub async fn fetch_channel_members(&self, id: i64) -> Result<String, String> {
        let req = channel_proto::ListChannelMembersRequest {
            org_slug: self.org_slug(), id, ..Default::default()
        };
        let resp = self.client.list_channel_members_connect(&req).await.map_err(crate::wire)?;
        let list = member_list_from_proto(resp);
        self.state.write().unwrap().set_channel_members(id, list.members.clone());
        serde_json::to_string(&list).map_err(crate::wire)
    }

    pub async fn invite_channel_members(&self, id: i64, user_ids_json: &str) -> Result<(), String> {
        let user_ids: Vec<i64> = serde_json::from_str(user_ids_json).map_err(crate::wire)?;
        let req = channel_proto::InviteChannelMembersRequest {
            org_slug: self.org_slug(), id, user_ids,
        };
        self.client.invite_channel_members_connect(&req).await.map_err(crate::wire)?;
        // Server ack-only; refresh the cache to reflect the new membership.
        let refresh_req = channel_proto::ListChannelMembersRequest {
            org_slug: self.org_slug(), id, ..Default::default()
        };
        let fresh = self.client.list_channel_members_connect(&refresh_req).await.map_err(crate::wire)?;
        self.state.write().unwrap().set_channel_members(id, member_list_from_proto(fresh).members);
        Ok(())
    }

    pub async fn remove_channel_member(&self, id: i64, user_id: i64) -> Result<(), String> {
        let req = channel_proto::RemoveChannelMemberRequest {
            org_slug: self.org_slug(), id, user_id,
        };
        self.client.remove_channel_member_connect(&req).await.map_err(crate::wire)?;
        self.state.write().unwrap().remove_channel_member(id, user_id);
        Ok(())
    }

    pub fn channel_members_json(&self, id: i64) -> String {
        let members = self.state.read().unwrap().get_channel_members(id);
        serde_json::to_string(&members).unwrap_or_else(|_| "[]".into())
    }

    pub async fn search_channel_messages(
        &self, id: i64, q: &str, limit: Option<u32>,
    ) -> Result<String, String> {
        let req = channel_proto::SearchChannelMessagesRequest {
            org_slug: self.org_slug(),
            channel_id: id,
            query: q.to_string(),
            limit: limit.map(|v| v as i32),
        };
        let resp = self.client.search_channel_messages_connect(&req).await.map_err(crate::wire)?;
        let messages: Vec<ChannelMessage> = resp.items.into_iter().map(message_from_proto).collect();
        serde_json::to_string(&serde_json::json!({ "messages": messages })).map_err(crate::wire)
    }
}

fn request_has_new_shape(v: &serde_json::Value) -> bool {
    let Some(obj) = v.as_object() else { return false };
    obj.contains_key("source")
        || obj.contains_key("content")
        || obj.contains_key("mentions")
        || obj.contains_key("attachment_key")
        || obj.contains_key("pod_key")
        || obj.contains_key("reply_to")
}
