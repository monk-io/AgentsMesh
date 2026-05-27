use std::collections::HashMap;

use agentsmesh_state::channel_state::{ChannelSortMode, ChannelState};
use agentsmesh_state::channel_types::ChannelMessage;
use agentsmesh_types::proto_channel_state_v1::{
    ApplyChannelMessageEditedEventRequest, ApplyIncomingChannelMessageRequest,
    InsertChannelMessageRequest, InsertChannelRequest, PatchChannelMemberCountRequest,
    PrependCachedChannelMessagesRequest, ReplaceCachedChannelMessagesRequest,
    ReplaceCachedChannelsRequest, ReplaceChannelUnreadCountsRequest,
};
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmChannelState {
    inner: ChannelState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

#[wasm_bindgen]
impl WasmChannelState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: ChannelState::with_storage(crate::new_memory_backend()) }
    }

    pub fn set_current_user_id(&mut self, user_id: Option<i64>) {
        self.inner.set_current_user_id(user_id);
    }

    pub fn channels_json(&self) -> String {
        serde_json::to_string(self.inner.get_channels()).unwrap_or_default()
    }

    pub fn current_channel_json(&self) -> JsValue {
        match self.inner.get_current_channel() {
            Some(c) => JsValue::from_str(
                &serde_json::to_string(c).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn set_current_channel(&mut self, id: Option<i64>) {
        self.inner.set_current_channel(id);
    }

    pub fn get_channel_json(&self, id: i64) -> JsValue {
        match self.inner.get_channel(id) {
            Some(c) => JsValue::from_str(
                &serde_json::to_string(c).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_channels(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedChannelsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_channels(req.channels);
        Ok(())
    }

    pub fn insert_channel(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertChannelRequest::decode(req_bytes).map_err(decode_err)?;
        let channel = req.channel.ok_or_else(|| JsValue::from_str("missing channel"))?;
        let id = channel.id;
        if self.inner.get_channel(id).is_some() {
            self.inner.update_channel(id, channel);
        } else {
            self.inner.add_channel(channel);
        }
        Ok(())
    }

    pub fn remove_channel(&mut self, id: i64) {
        self.inner.remove_channel(id);
    }

    pub fn patch_channel_member_count(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchChannelMemberCountRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(existing) = self.inner.get_channel(req.channel_id).cloned() {
            let mut next = existing;
            let curr = next.member_count.unwrap_or(0);
            let new = (curr + req.delta as i64).max(0);
            next.member_count = Some(new);
            self.inner.update_channel(req.channel_id, next);
        }
        Ok(())
    }

    pub fn filter_channels_json(&self, query: &str, include_archived: bool) -> String {
        let filtered = self.inner.filter_channels(query, include_archived);
        serde_json::to_string(&filtered).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn select_channel(&mut self, id: Option<i64>) -> JsValue {
        match self.inner.select_channel(id) {
            Some(c) => JsValue::from_str(
                &serde_json::to_string(c).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn sorted_channel_ids_json(&self, mode: &str, include_archived: bool) -> String {
        let sort_mode = match mode {
            "unread_first" => ChannelSortMode::UnreadFirst,
            "name" => ChannelSortMode::Name,
            _ => ChannelSortMode::LastMessage,
        };
        let ids = self.inner.sorted_channel_ids(sort_mode, include_archived);
        serde_json::to_string(&ids).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn get_last_message_json(&self, channel_id: i64) -> JsValue {
        match self.inner.get_last_message(channel_id) {
            Some(preview) => JsValue::from_str(
                &serde_json::to_string(preview).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn apply_incoming_channel_message(&mut self, req_bytes: &[u8]) -> Result<bool, JsValue> {
        let req = ApplyIncomingChannelMessageRequest::decode(req_bytes).map_err(decode_err)?;
        let msg = req.message.ok_or_else(|| JsValue::from_str("missing message"))?;
        Ok(self.inner.on_new_message(msg))
    }

    pub fn insert_channel_message(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertChannelMessageRequest::decode(req_bytes).map_err(decode_err)?;
        let msg = req.message.ok_or_else(|| JsValue::from_str("missing message"))?;
        self.inner.add_message(req.channel_id, msg);
        Ok(())
    }

    pub fn apply_channel_message_edited_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyChannelMessageEditedEventRequest::decode(req_bytes).map_err(decode_err)?;
        let mentions_json = if req.mentions.is_empty() {
            None
        } else {
            serde_json::to_string(&req.mentions).ok()
        };
        let mut patch = ChannelMessage {
            id: req.message_id,
            channel_id: req.channel_id,
            body: req.body,
            content_json: req.content,
            mentions_json,
            edited_at: Some(req.edited_at),
            ..ChannelMessage::default()
        };
        // ChannelMessage default has `body: ""`, but the inner.update_message
        // guards on `!message.body.is_empty()` — explicit reset above is fine.
        // Clear the default channel_id=0 if the caller really meant ID 0 — but
        // realtime always supplies a positive channel_id, so the override above
        // is correct.
        let _ = &mut patch;
        self.inner.update_message(req.channel_id, patch);
        Ok(())
    }

    pub fn remove_message(&mut self, channel_id: i64, message_id: i64) {
        self.inner.remove_message(channel_id, message_id);
    }

    pub fn replace_cached_channel_messages(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedChannelMessagesRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_messages(req.channel_id, req.messages, req.has_more);
        Ok(())
    }

    pub fn prepend_cached_channel_messages(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PrependCachedChannelMessagesRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.prepend_messages(req.channel_id, req.messages, req.has_more);
        Ok(())
    }

    pub fn get_messages_json(&self, channel_id: i64) -> JsValue {
        match self.inner.get_messages(channel_id) {
            Some(cache) => {
                let val = serde_json::json!({
                    "messages": cache.messages,
                    "has_more": cache.has_more,
                });
                JsValue::from_str(
                    &serde_json::to_string(&val).unwrap_or_default(),
                )
            }
            None => JsValue::NULL,
        }
    }

    pub fn replace_channel_unread_counts(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceChannelUnreadCountsRequest::decode(req_bytes).map_err(decode_err)?;
        let counts: HashMap<i64, u32> = req.counts.into_iter().collect();
        self.inner.set_unread_counts(counts);
        Ok(())
    }

    pub fn increment_unread(&mut self, channel_id: i64) {
        self.inner.increment_unread(channel_id);
    }

    pub fn clear_channel_unread(&mut self, channel_id: i64) {
        self.inner.clear_channel_unread(channel_id);
    }

    pub fn get_unread_count(&self, channel_id: i64) -> u32 {
        self.inner.get_unread_count(channel_id)
    }

    pub fn total_unread_count(&self) -> u32 {
        self.inner.total_unread_count()
    }

    pub fn unread_counts_json(&self) -> String {
        let counts = self.inner.get_all_unread_counts();
        serde_json::to_string(&counts).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn increment_mention(&mut self, channel_id: i64) {
        self.inner.increment_mention(channel_id);
    }

    pub fn clear_channel_mentions(&mut self, channel_id: i64) {
        self.inner.clear_channel_mentions(channel_id);
    }

    pub fn get_mention_count(&self, channel_id: i64) -> u32 {
        self.inner.get_mention_count(channel_id)
    }

    pub fn total_mention_count(&self) -> u32 {
        self.inner.total_mention_count()
    }

    pub fn mention_counts_json(&self) -> String {
        let counts = self.inner.get_all_mention_counts();
        serde_json::to_string(&counts).unwrap_or_else(|_| "{}".to_string())
    }
}
