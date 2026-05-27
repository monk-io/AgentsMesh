use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::ChannelService;
use agentsmesh_state::channel_state::ChannelState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmChannelService(pub(crate) ChannelService);

fn map_err(e: String) -> JsValue { JsValue::from_str(&e) }

#[wasm_bindgen]
impl WasmChannelService {
    pub(crate) fn new(client: Arc<ApiClient>, state: ChannelState) -> Self {
        Self(ChannelService::new(client, state))
    }

    pub fn channels_json(&self) -> String { self.0.channels_json() }

    pub fn current_channel_json(&self) -> JsValue {
        match self.0.current_channel_json() {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_channel_json(&self, id: i64) -> JsValue {
        match self.0.get_channel_json(id) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn filter_channels_json(&self, query: &str, include_archived: bool) -> String {
        self.0.filter_channels_json(query, include_archived)
    }

    pub fn get_messages_json(&self, channel_id: i64) -> JsValue {
        match self.0.get_messages_json(channel_id) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_unread_count(&self, channel_id: i64) -> u32 { self.0.get_unread_count(channel_id) }
    pub fn total_unread_count(&self) -> u32 { self.0.total_unread_count() }
    pub fn unread_counts_json(&self) -> String { self.0.unread_counts_json() }
    pub fn get_mention_count(&self, channel_id: i64) -> u32 { self.0.get_mention_count(channel_id) }
    pub fn total_mention_count(&self) -> u32 { self.0.total_mention_count() }
    pub fn mention_counts_json(&self) -> String { self.0.mention_counts_json() }

    pub fn sorted_channel_ids_json(&self, mode: &str, include_archived: bool) -> String {
        self.0.sorted_channel_ids_json(mode, include_archived)
    }

    pub fn get_last_message_json(&self, channel_id: i64) -> JsValue {
        match self.0.get_last_message_json(channel_id) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn set_current_channel(&self, id: Option<i64>) { self.0.set_current_channel(id); }

    pub fn select_channel(&self, id: Option<i64>) -> JsValue {
        match self.0.select_channel(id) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    // ---- Legacy JSON-bridge entry points (facade/channel.ts still uses these
    // until that layer migrates to proto). ----

    pub fn update_channel_local(&self, id: i64, json: &str) {
        self.0.update_channel_local(id, json);
    }

    pub fn set_channel_pods_local(&self, channel_id: i64, json: &str) {
        self.0.set_channel_pods_local(channel_id, json);
    }

    pub fn set_channel_members_local(&self, channel_id: i64, json: &str) {
        self.0.set_channel_members_local(channel_id, json);
    }

    pub fn remove_channel_member_local(&self, channel_id: i64, user_id: i64) {
        self.0.remove_channel_member_local(channel_id, user_id);
    }

    // ---- Proto-bytes mutators (new SSOT contract) ----

    pub fn replace_cached_channels(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.replace_cached_channels(req_bytes).map_err(map_err)
    }

    pub fn insert_channel(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.insert_channel(req_bytes).map_err(map_err)
    }

    pub fn patch_channel_member_count(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.patch_channel_member_count(req_bytes).map_err(map_err)
    }

    pub fn apply_incoming_channel_message(&self, req_bytes: &[u8]) -> Result<bool, JsValue> {
        self.0.apply_incoming_channel_message(req_bytes).map_err(map_err)
    }

    pub fn insert_channel_message(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.insert_channel_message(req_bytes).map_err(map_err)
    }

    pub fn apply_channel_message_edited_event(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.apply_channel_message_edited_event(req_bytes).map_err(map_err)
    }

    pub fn remove_message(&self, channel_id: i64, message_id: i64) {
        self.0.remove_message(channel_id, message_id);
    }

    pub fn replace_cached_channel_messages(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.replace_cached_channel_messages(req_bytes).map_err(map_err)
    }

    pub fn prepend_cached_channel_messages(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.prepend_cached_channel_messages(req_bytes).map_err(map_err)
    }

    pub fn replace_channel_unread_counts(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        self.0.replace_channel_unread_counts(req_bytes).map_err(map_err)
    }

    pub fn increment_unread(&self, channel_id: i64) { self.0.increment_unread(channel_id); }
    pub fn clear_channel_unread(&self, channel_id: i64) { self.0.clear_channel_unread(channel_id); }
    pub fn increment_mention(&self, channel_id: i64) { self.0.increment_mention(channel_id); }

    pub fn clear_channel_mentions(&self, channel_id: i64) {
        self.0.clear_channel_mentions(channel_id);
    }

    pub fn channel_pods_json(&self, id: i64) -> String {
        self.0.channel_pods_json(id)
    }

    pub fn channel_members_json(&self, id: i64) -> String {
        self.0.channel_members_json(id)
    }
}
