use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_channel_v1 as channel_proto;

impl ApiClient {
    pub async fn list_channels(
        &self,
        include_archived: Option<bool>,
    ) -> Result<ChannelListResponse, ApiError> {
        let mut path = self.org_path("/channels");
        if let Some(archived) = include_archived {
            path = format!("{path}?include_archived={archived}");
        }
        self.get(&path).await
    }

    pub async fn get_channel(&self, id: i64) -> Result<Channel, ApiError> {
        self.get_resource(&self.org_path(&format!("/channels/{id}")), "channel").await
    }

    pub async fn create_channel(
        &self,
        data: &CreateChannelRequest,
    ) -> Result<Channel, ApiError> {
        self.post_resource(&self.org_path("/channels"), data, "channel").await
    }

    pub async fn update_channel(
        &self,
        id: i64,
        data: &UpdateChannelRequest,
    ) -> Result<Channel, ApiError> {
        self.put_resource(&self.org_path(&format!("/channels/{id}")), data, "channel").await
    }

    pub async fn archive_channel(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/channels/{id}/archive")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn unarchive_channel(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/channels/{id}/unarchive")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn get_channel_messages(
        &self,
        id: i64,
        limit: Option<u32>,
        before_id: Option<i64>,
    ) -> Result<ChannelMessageListResponse, ApiError> {
        let mut path = self.org_path(&format!("/channels/{id}/messages"));
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(b) = before_id {
            params.push(format!("before_id={b}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn send_channel_message(
        &self,
        id: i64,
        data: &SendChannelMessageRequest,
    ) -> Result<ChannelMessage, ApiError> {
        self.post_resource(&self.org_path(&format!("/channels/{id}/messages")), data, "message").await
    }

    pub async fn get_channel_pods(&self, id: i64) -> Result<PodListResponse, ApiError> {
        self.get(&self.org_path(&format!("/channels/{id}/pods")))
            .await
    }

    pub async fn join_channel_pod(
        &self,
        id: i64,
        data: &JoinChannelPodRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path(&format!("/channels/{id}/pods")), data)
            .await
    }

    pub async fn leave_channel_pod(
        &self,
        id: i64,
        pod_key: &str,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/channels/{id}/pods/{pod_key}")))
            .await
    }

    pub async fn mark_channel_read(
        &self,
        id: i64,
        message_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/channels/{id}/read")),
            &serde_json::json!({ "message_id": message_id }),
        )
        .await
    }

    pub async fn get_channel_unread_counts(&self) -> Result<ChannelUnreadResponse, ApiError> {
        self.get(&self.org_path("/channels/unread")).await
    }

    pub async fn edit_channel_message(
        &self,
        channel_id: i64,
        message_id: i64,
        data: &EditChannelMessageRequest,
    ) -> Result<ChannelMessage, ApiError> {
        self.put_resource(
            &self.org_path(&format!("/channels/{channel_id}/messages/{message_id}")),
            data, "message",
        ).await
    }

    pub async fn delete_channel_message(
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!(
            "/channels/{channel_id}/messages/{message_id}"
        )))
        .await
    }

    pub async fn mute_channel(
        &self,
        id: i64,
        data: &MuteChannelRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path(&format!("/channels/{id}/mute")), data)
            .await
    }

    pub async fn list_channel_members(
        &self,
        id: i64,
    ) -> Result<ChannelMemberListResponse, ApiError> {
        self.get(&self.org_path(&format!("/channels/{id}/members")))
            .await
    }

    pub async fn invite_channel_members(
        &self,
        id: i64,
        data: &InviteChannelMembersRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path(&format!("/channels/{id}/members")), data)
            .await
    }

    pub async fn remove_channel_member(
        &self,
        id: i64,
        user_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/channels/{id}/members/{user_id}")))
            .await
    }

    pub async fn search_channel_messages(
        &self,
        id: i64,
        q: &str,
        limit: Option<u32>,
    ) -> Result<serde_json::Value, ApiError> {
        let lim = limit.unwrap_or(20);
        let path = self.org_path(&format!(
            "/channels/{id}/messages/search?q={}&limit={lim}",
            urlencoding::encode(q)
        ));
        self.get(&path).await
    }
}

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/channel/. Procedure paths derive from
// `proto.channel.v1.ChannelService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn list_channels_connect(
        &self, req: &channel_proto::ListChannelsRequest,
    ) -> Result<channel_proto::ListChannelsResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/ListChannels", req).await
    }

    pub async fn get_channel_connect(
        &self, req: &channel_proto::GetChannelRequest,
    ) -> Result<channel_proto::Channel, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/GetChannel", req).await
    }

    pub async fn create_channel_connect(
        &self, req: &channel_proto::CreateChannelRequest,
    ) -> Result<channel_proto::Channel, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/CreateChannel", req).await
    }

    pub async fn update_channel_connect(
        &self, req: &channel_proto::UpdateChannelRequest,
    ) -> Result<channel_proto::Channel, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/UpdateChannel", req).await
    }

    pub async fn archive_channel_connect(
        &self, req: &channel_proto::ArchiveChannelRequest,
    ) -> Result<channel_proto::ArchiveChannelResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/ArchiveChannel", req).await
    }

    pub async fn unarchive_channel_connect(
        &self, req: &channel_proto::UnarchiveChannelRequest,
    ) -> Result<channel_proto::UnarchiveChannelResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/UnarchiveChannel", req).await
    }

    pub async fn get_channel_document_connect(
        &self, req: &channel_proto::GetChannelDocumentRequest,
    ) -> Result<channel_proto::GetChannelDocumentResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/GetChannelDocument", req).await
    }

    pub async fn update_channel_document_connect(
        &self, req: &channel_proto::UpdateChannelDocumentRequest,
    ) -> Result<channel_proto::UpdateChannelDocumentResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/UpdateChannelDocument", req).await
    }

    pub async fn list_channel_messages_connect(
        &self, req: &channel_proto::ListChannelMessagesRequest,
    ) -> Result<channel_proto::ListChannelMessagesResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/ListChannelMessages", req).await
    }

    pub async fn search_channel_messages_connect(
        &self, req: &channel_proto::SearchChannelMessagesRequest,
    ) -> Result<channel_proto::SearchChannelMessagesResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/SearchChannelMessages", req).await
    }

    pub async fn send_channel_message_connect(
        &self, req: &channel_proto::SendChannelMessageRequest,
    ) -> Result<channel_proto::ChannelMessage, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/SendChannelMessage", req).await
    }

    pub async fn edit_channel_message_connect(
        &self, req: &channel_proto::EditChannelMessageRequest,
    ) -> Result<channel_proto::ChannelMessage, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/EditChannelMessage", req).await
    }

    pub async fn delete_channel_message_connect(
        &self, req: &channel_proto::DeleteChannelMessageRequest,
    ) -> Result<channel_proto::DeleteChannelMessageResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/DeleteChannelMessage", req).await
    }

    pub async fn mark_channel_read_connect(
        &self, req: &channel_proto::MarkChannelReadRequest,
    ) -> Result<channel_proto::MarkChannelReadResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/MarkChannelRead", req).await
    }

    pub async fn get_channel_unread_counts_connect(
        &self, req: &channel_proto::GetChannelUnreadCountsRequest,
    ) -> Result<channel_proto::GetChannelUnreadCountsResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/GetChannelUnreadCounts", req).await
    }

    pub async fn mute_channel_connect(
        &self, req: &channel_proto::MuteChannelRequest,
    ) -> Result<channel_proto::MuteChannelResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/MuteChannel", req).await
    }

    pub async fn list_channel_members_connect(
        &self, req: &channel_proto::ListChannelMembersRequest,
    ) -> Result<channel_proto::ListChannelMembersResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/ListChannelMembers", req).await
    }

    pub async fn join_channel_connect(
        &self, req: &channel_proto::JoinChannelRequest,
    ) -> Result<channel_proto::JoinChannelResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/JoinChannel", req).await
    }

    pub async fn leave_channel_connect(
        &self, req: &channel_proto::LeaveChannelRequest,
    ) -> Result<channel_proto::LeaveChannelResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/LeaveChannel", req).await
    }

    pub async fn invite_channel_members_connect(
        &self, req: &channel_proto::InviteChannelMembersRequest,
    ) -> Result<channel_proto::InviteChannelMembersResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/InviteChannelMembers", req).await
    }

    pub async fn remove_channel_member_connect(
        &self, req: &channel_proto::RemoveChannelMemberRequest,
    ) -> Result<channel_proto::RemoveChannelMemberResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/RemoveChannelMember", req).await
    }

    pub async fn list_channel_pods_connect(
        &self, req: &channel_proto::ListChannelPodsRequest,
    ) -> Result<channel_proto::ListChannelPodsResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/ListChannelPods", req).await
    }

    pub async fn join_channel_pod_connect(
        &self, req: &channel_proto::JoinChannelPodRequest,
    ) -> Result<channel_proto::JoinChannelPodResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/JoinChannelPod", req).await
    }

    pub async fn leave_channel_pod_connect(
        &self, req: &channel_proto::LeaveChannelPodRequest,
    ) -> Result<channel_proto::LeaveChannelPodResponse, ApiError> {
        connect_call(self, "/proto.channel.v1.ChannelService/LeaveChannelPod", req).await
    }
}
