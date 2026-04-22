use crate::core::AgentsMeshCore;
use crate::dto::{
    edit_message_req, invite_channel_members_req, join_channel_pod_req, mute_channel_req,
    send_message_req, ChannelDto, ChannelListResponseDto, ChannelMemberListResponseDto,
    ChannelMessageDto, ChannelMessageListResponseDto, ChannelUnreadResponseDto,
    CreateChannelRequestDto, PodListResponseDto, UpdateChannelRequestDto,
};
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    pub async fn list_channels(
        &self,
        include_archived: Option<bool>,
    ) -> Result<ChannelListResponseDto, CoreError> {
        let resp = self.api.list_channels(include_archived).await?;
        Ok(resp.into())
    }

    pub async fn get_channel(&self, id: i64) -> Result<ChannelDto, CoreError> {
        let ch = self.api.get_channel(id).await?;
        Ok(ch.into())
    }

    pub async fn create_channel(
        &self,
        req: CreateChannelRequestDto,
    ) -> Result<ChannelDto, CoreError> {
        let ch = self.api.create_channel(&req.into()).await?;
        Ok(ch.into())
    }

    pub async fn update_channel(
        &self,
        id: i64,
        req: UpdateChannelRequestDto,
    ) -> Result<ChannelDto, CoreError> {
        let ch = self.api.update_channel(id, &req.into()).await?;
        Ok(ch.into())
    }

    pub async fn archive_channel(&self, id: i64) -> Result<(), CoreError> {
        self.api.archive_channel(id).await?;
        Ok(())
    }

    pub async fn unarchive_channel(&self, id: i64) -> Result<(), CoreError> {
        self.api.unarchive_channel(id).await?;
        Ok(())
    }

    pub async fn get_channel_messages(
        &self,
        id: i64,
        limit: Option<u32>,
        before_id: Option<i64>,
    ) -> Result<ChannelMessageListResponseDto, CoreError> {
        let resp = self.api.get_channel_messages(id, limit, before_id).await?;
        Ok(resp.into())
    }

    /// Send a channel message. `content_json` is the AST string (validated
    /// by the server; frontend builds it via the agentfile/block editor).
    pub async fn send_channel_message(
        &self,
        id: i64,
        content_json: String,
        pod_key: Option<String>,
        reply_to: Option<i64>,
    ) -> Result<ChannelMessageDto, CoreError> {
        let req = send_message_req(content_json, pod_key, reply_to)?;
        let msg = self.api.send_channel_message(id, &req).await?;
        Ok(msg.into())
    }

    pub async fn edit_channel_message(
        &self,
        channel_id: i64,
        message_id: i64,
        content_json: String,
    ) -> Result<ChannelMessageDto, CoreError> {
        let req = edit_message_req(content_json)?;
        let msg = self
            .api
            .edit_channel_message(channel_id, message_id, &req)
            .await?;
        Ok(msg.into())
    }

    pub async fn delete_channel_message(
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> Result<(), CoreError> {
        self.api
            .delete_channel_message(channel_id, message_id)
            .await?;
        Ok(())
    }

    pub async fn mark_channel_read(
        &self,
        id: i64,
        message_id: i64,
    ) -> Result<(), CoreError> {
        self.api.mark_channel_read(id, message_id).await?;
        Ok(())
    }

    pub async fn get_channel_unread_counts(&self) -> Result<ChannelUnreadResponseDto, CoreError> {
        let resp = self.api.get_channel_unread_counts().await?;
        Ok(resp.into())
    }

    pub async fn mute_channel(&self, id: i64, muted: bool) -> Result<(), CoreError> {
        self.api.mute_channel(id, &mute_channel_req(muted)).await?;
        Ok(())
    }

    pub async fn get_channel_pods(&self, id: i64) -> Result<PodListResponseDto, CoreError> {
        let resp = self.api.get_channel_pods(id).await?;
        Ok(resp.into())
    }

    pub async fn join_channel_pod(&self, id: i64, pod_key: String) -> Result<(), CoreError> {
        self.api
            .join_channel_pod(id, &join_channel_pod_req(pod_key))
            .await?;
        Ok(())
    }

    pub async fn leave_channel_pod(
        &self,
        id: i64,
        pod_key: String,
    ) -> Result<(), CoreError> {
        self.api.leave_channel_pod(id, &pod_key).await?;
        Ok(())
    }

    pub async fn list_channel_members(
        &self,
        id: i64,
    ) -> Result<ChannelMemberListResponseDto, CoreError> {
        let resp = self.api.list_channel_members(id).await?;
        Ok(resp.into())
    }

    pub async fn invite_channel_members(
        &self,
        id: i64,
        user_ids: Vec<i64>,
    ) -> Result<(), CoreError> {
        self.api
            .invite_channel_members(id, &invite_channel_members_req(user_ids))
            .await?;
        Ok(())
    }

    pub async fn remove_channel_member(
        &self,
        id: i64,
        user_id: i64,
    ) -> Result<(), CoreError> {
        self.api.remove_channel_member(id, user_id).await?;
        Ok(())
    }
}
