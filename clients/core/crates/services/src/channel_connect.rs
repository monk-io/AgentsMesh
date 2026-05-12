// Connect-RPC bridge methods for ChannelService. Binary in, binary out
// (conventions §2.5). Each method:
//   1. Decodes the prost-encoded request bytes from the wasm bridge.
//   2. Forwards to the api-client `*_connect` method (which speaks
//      application/proto to the Connect handler).
//   3. Encodes the response back to prost bytes for the bridge to return.
//
// State management (channels_json, set_channels, etc.) stays in channel.rs
// because the wasm bridge re-uses it for the dual-track REST path. Once the
// REST lane is retired, the Connect responses can be projected back into the
// same state via a post-decode hook.

use agentsmesh_types::proto_channel_v1 as channel_proto;
use prost::Message;

use crate::channel::ChannelService;
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
