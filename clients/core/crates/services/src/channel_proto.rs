// Proto-bytes mutator surface for ChannelService. Each method decodes a
// prost-encoded request (issued by TS/Swift via wasm-bindgen / NAPI /
// UniFFI) and applies it to the in-memory cache via the underlying
// ChannelState.

use std::collections::HashMap;

use agentsmesh_state::channel_types::ChannelMessage;
use agentsmesh_types::proto_channel_state_v1::{
    ApplyChannelMessageEditedEventRequest, ApplyIncomingChannelMessageRequest,
    InsertChannelMessageRequest, InsertChannelRequest, PatchChannelMemberCountRequest,
    PrependCachedChannelMessagesRequest, RemoveChannelMemberRequest,
    ReplaceCachedChannelMessagesRequest, ReplaceCachedChannelsRequest,
    ReplaceChannelMembersRequest, ReplaceChannelPodsRequest,
    ReplaceChannelUnreadCountsRequest,
};
use prost::Message;

use super::ChannelService;

impl ChannelService {
    pub fn replace_cached_channels(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceCachedChannelsRequest::decode(req_bytes)
            .map_err(|e| format!("decode ReplaceCachedChannelsRequest: {e}"))?;
        self.state_write().set_channels(req.channels);
        Ok(())
    }

    pub fn insert_channel(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = InsertChannelRequest::decode(req_bytes)
            .map_err(|e| format!("decode InsertChannelRequest: {e}"))?;
        let channel = req.channel.ok_or_else(|| "missing channel".to_string())?;
        let id = channel.id;
        let mut state = self.state_write();
        if state.get_channel(id).is_some() {
            state.update_channel(id, channel);
        } else {
            state.add_channel(channel);
        }
        Ok(())
    }

    pub fn patch_channel_member_count(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = PatchChannelMemberCountRequest::decode(req_bytes)
            .map_err(|e| format!("decode PatchChannelMemberCountRequest: {e}"))?;
        let mut state = self.state_write();
        let Some(existing) = state.get_channel(req.channel_id).cloned() else {
            return Ok(());
        };
        let mut next = existing;
        let curr = next.member_count.unwrap_or(0);
        next.member_count = Some((curr + req.delta as i64).max(0));
        state.update_channel(req.channel_id, next);
        Ok(())
    }

    pub fn apply_incoming_channel_message(&self, req_bytes: &[u8]) -> Result<bool, String> {
        let req = ApplyIncomingChannelMessageRequest::decode(req_bytes)
            .map_err(|e| format!("decode ApplyIncomingChannelMessageRequest: {e}"))?;
        let msg = req.message.ok_or_else(|| "missing message".to_string())?;
        Ok(self.state_write().on_new_message(msg))
    }

    pub fn insert_channel_message(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = InsertChannelMessageRequest::decode(req_bytes)
            .map_err(|e| format!("decode InsertChannelMessageRequest: {e}"))?;
        let msg = req.message.ok_or_else(|| "missing message".to_string())?;
        self.state_write().add_message(req.channel_id, msg);
        Ok(())
    }

    pub fn apply_channel_message_edited_event(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ApplyChannelMessageEditedEventRequest::decode(req_bytes)
            .map_err(|e| format!("decode ApplyChannelMessageEditedEventRequest: {e}"))?;
        let mentions_json = if req.mentions.is_empty() {
            None
        } else {
            serde_json::to_string(&req.mentions).ok()
        };
        let patch = ChannelMessage {
            id: req.message_id,
            channel_id: req.channel_id,
            body: req.body,
            content_json: req.content,
            mentions_json,
            edited_at: Some(req.edited_at),
            ..ChannelMessage::default()
        };
        self.state_write().update_message(req.channel_id, patch);
        Ok(())
    }

    pub fn replace_cached_channel_messages(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceCachedChannelMessagesRequest::decode(req_bytes)
            .map_err(|e| format!("decode ReplaceCachedChannelMessagesRequest: {e}"))?;
        self.state_write().set_messages(req.channel_id, req.messages, req.has_more);
        Ok(())
    }

    pub fn prepend_cached_channel_messages(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = PrependCachedChannelMessagesRequest::decode(req_bytes)
            .map_err(|e| format!("decode PrependCachedChannelMessagesRequest: {e}"))?;
        self.state_write().prepend_messages(req.channel_id, req.messages, req.has_more);
        Ok(())
    }

    pub fn replace_channel_unread_counts(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceChannelUnreadCountsRequest::decode(req_bytes)
            .map_err(|e| format!("decode ReplaceChannelUnreadCountsRequest: {e}"))?;
        let counts: HashMap<i64, u32> = req.counts.into_iter().collect();
        self.state_write().set_unread_counts(counts);
        Ok(())
    }

    pub fn replace_channel_pods(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceChannelPodsRequest::decode(req_bytes)
            .map_err(|e| format!("decode ReplaceChannelPodsRequest: {e}"))?;
        self.state_write().set_channel_pods(req.channel_id, req.pods);
        Ok(())
    }

    pub fn replace_channel_members(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceChannelMembersRequest::decode(req_bytes)
            .map_err(|e| format!("decode ReplaceChannelMembersRequest: {e}"))?;
        self.state_write().set_channel_members(req.channel_id, req.members);
        Ok(())
    }

    pub fn remove_channel_member(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = RemoveChannelMemberRequest::decode(req_bytes)
            .map_err(|e| format!("decode RemoveChannelMemberRequest: {e}"))?;
        self.state_write().remove_channel_member(req.channel_id, req.user_id);
        Ok(())
    }
}
