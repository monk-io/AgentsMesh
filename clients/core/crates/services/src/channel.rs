use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::channel_state::ChannelState;
use prost::Message as _;

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

    /// Crate-local write lock used by the proto mutator surface
    /// (channel_proto.rs).
    pub(crate) fn state_write(&self) -> RwLockWriteGuard<'_, ChannelState> {
        self.state.write().unwrap()
    }

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
        let counts = self.state.read().unwrap().get_all_unread_counts();
        serde_json::to_string(&counts).unwrap_or_default()
    }

    pub fn get_mention_count(&self, channel_id: i64) -> u32 {
        self.state.read().unwrap().get_mention_count(channel_id)
    }

    pub fn total_mention_count(&self) -> u32 {
        self.state.read().unwrap().total_mention_count()
    }

    pub fn mention_counts_json(&self) -> String {
        let counts = self.state.read().unwrap().get_all_mention_counts();
        serde_json::to_string(&counts).unwrap_or_default()
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

    pub fn set_current_channel(&self, id: Option<i64>) {
        self.state.write().unwrap().set_current_channel(id);
    }

    pub fn select_channel(&self, id: Option<i64>) -> Option<String> {
        self.state.write().unwrap().select_channel(id)
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn remove_message(&self, channel_id: i64, message_id: i64) {
        self.state.write().unwrap().remove_message(channel_id, message_id);
    }

    pub fn increment_unread(&self, channel_id: i64) {
        self.state.write().unwrap().increment_unread(channel_id);
    }

    pub fn clear_channel_unread(&self, channel_id: i64) {
        self.state.write().unwrap().clear_channel_unread(channel_id);
    }

    pub fn increment_mention(&self, channel_id: i64) {
        self.state.write().unwrap().increment_mention(channel_id);
    }

    pub fn clear_channel_mentions(&self, channel_id: i64) {
        self.state.write().unwrap().clear_channel_mentions(channel_id);
    }

    pub fn channel_pods_json(&self, id: i64) -> String {
        let pods = self.state.read().unwrap().get_channel_pods(id);
        serde_json::to_string(&pods).unwrap_or_else(|_| "[]".into())
    }

    pub fn channel_members_json(&self, id: i64) -> String {
        let members = self.state.read().unwrap().get_channel_members(id);
        serde_json::to_string(&members).unwrap_or_else(|_| "[]".into())
    }
}

// =============================================================================
// Connect-RPC bridge methods. Binary in (prost-encoded), binary out — same wire
// the wasm/node-bridge layers speak. Each method decodes the request, calls the
// `*_connect` method on the shared ApiClient, and re-encodes the response.
// =============================================================================

use agentsmesh_types::proto_channel_v1 as channel_proto;

use crate::wire;

macro_rules! connect_bridge {
    ($name:ident, $req:ident, $client_call:ident) => {
        pub async fn $name(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
            let req = channel_proto::$req::decode(request_bytes)
                .map_err(|e| format!("decode {}: {e}", stringify!($req)))?;
            let resp = self.client().$client_call(&req).await.map_err(wire)?;
            Ok(resp.encode_to_vec())
        }
    };
}

impl ChannelService {
    connect_bridge!(list_channels_connect, ListChannelsRequest, list_channels_connect);
    connect_bridge!(get_channel_connect, GetChannelRequest, get_channel_connect);
    connect_bridge!(create_channel_connect, CreateChannelRequest, create_channel_connect);
    connect_bridge!(update_channel_connect, UpdateChannelRequest, update_channel_connect);
    connect_bridge!(archive_channel_connect, ArchiveChannelRequest, archive_channel_connect);
    connect_bridge!(unarchive_channel_connect, UnarchiveChannelRequest, unarchive_channel_connect);
    connect_bridge!(get_channel_document_connect, GetChannelDocumentRequest, get_channel_document_connect);
    connect_bridge!(update_channel_document_connect, UpdateChannelDocumentRequest, update_channel_document_connect);
    connect_bridge!(list_channel_messages_connect, ListChannelMessagesRequest, list_channel_messages_connect);
    connect_bridge!(search_channel_messages_connect, SearchChannelMessagesRequest, search_channel_messages_connect);
    connect_bridge!(send_channel_message_connect, SendChannelMessageRequest, send_channel_message_connect);
    connect_bridge!(edit_channel_message_connect, EditChannelMessageRequest, edit_channel_message_connect);
    connect_bridge!(delete_channel_message_connect, DeleteChannelMessageRequest, delete_channel_message_connect);
    connect_bridge!(mark_channel_read_connect, MarkChannelReadRequest, mark_channel_read_connect);
    connect_bridge!(get_channel_unread_counts_connect, GetChannelUnreadCountsRequest, get_channel_unread_counts_connect);
    connect_bridge!(mute_channel_connect, MuteChannelRequest, mute_channel_connect);
    connect_bridge!(list_channel_members_connect, ListChannelMembersRequest, list_channel_members_connect);
    connect_bridge!(join_channel_connect, JoinChannelRequest, join_channel_connect);
    connect_bridge!(leave_channel_connect, LeaveChannelRequest, leave_channel_connect);
    connect_bridge!(invite_channel_members_connect, InviteChannelMembersRequest, invite_channel_members_connect);
    connect_bridge!(remove_channel_member_connect, RemoveChannelMemberRequest, remove_channel_member_connect);
    connect_bridge!(list_channel_pods_connect, ListChannelPodsRequest, list_channel_pods_connect);
    connect_bridge!(join_channel_pod_connect, JoinChannelPodRequest, join_channel_pod_connect);
    connect_bridge!(leave_channel_pod_connect, LeaveChannelPodRequest, leave_channel_pod_connect);
}
