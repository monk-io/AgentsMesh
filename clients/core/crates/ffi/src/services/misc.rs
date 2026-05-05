use crate::core::AgentsMeshCore;
use crate::dto::{
    create_invitation_req, create_resource_grant_req, InvitationDto, InvitationListResponseDto,
    PresignRequestDto, PresignResponseDto, ResourceGrantListResponseDto, ResourceGrantResponseDto,
};
use crate::error::CoreError;

#[uniffi::export(async_runtime = "tokio")]
impl AgentsMeshCore {
    // ── Invitations ───────────────────────────────────────

    pub async fn list_invitations(&self) -> Result<InvitationListResponseDto, CoreError> {
        let resp = self.api.list_invitations().await?;
        Ok(resp.into())
    }

    pub async fn create_invitation(
        &self,
        email: String,
        role: String,
    ) -> Result<InvitationDto, CoreError> {
        let inv = self
            .api
            .create_invitation(&create_invitation_req(email, role))
            .await?;
        Ok(inv.into())
    }

    pub async fn revoke_invitation(&self, id: i64) -> Result<(), CoreError> {
        self.api.revoke_invitation(id).await?;
        Ok(())
    }

    pub async fn resend_invitation(&self, id: i64) -> Result<(), CoreError> {
        self.api.resend_invitation(id).await?;
        Ok(())
    }

    pub async fn get_invitation_by_token(&self, token: String) -> Result<InvitationDto, CoreError> {
        let inv = self.api.get_invitation_by_token(&token).await?;
        Ok(inv.into())
    }

    pub async fn accept_invitation(&self, token: String) -> Result<(), CoreError> {
        self.api.accept_invitation(&token).await?;
        Ok(())
    }

    pub async fn list_pending_invitations(
        &self,
    ) -> Result<InvitationListResponseDto, CoreError> {
        let resp = self.api.list_pending_invitations().await?;
        Ok(resp.into())
    }

    // ── File presign ──────────────────────────────────────

    pub async fn presign_file_upload(
        &self,
        req: PresignRequestDto,
    ) -> Result<PresignResponseDto, CoreError> {
        let resp = self.api.presign_file_upload(&req.into()).await?;
        Ok(resp.into())
    }

    // ── Resource Grants ───────────────────────────────────

    pub async fn list_resource_grants(
        &self,
        resource_type: String,
        resource_id: String,
    ) -> Result<ResourceGrantListResponseDto, CoreError> {
        let resp = self
            .api
            .list_resource_grants(&resource_type, &resource_id)
            .await?;
        Ok(resp.into())
    }

    pub async fn grant_resource_access(
        &self,
        resource_type: String,
        resource_id: String,
        user_id: i64,
    ) -> Result<ResourceGrantResponseDto, CoreError> {
        let resp = self
            .api
            .grant_resource_access(&resource_type, &resource_id, &create_resource_grant_req(user_id))
            .await?;
        Ok(resp.into())
    }

    pub async fn revoke_resource_grant(
        &self,
        resource_type: String,
        resource_id: String,
        grant_id: i64,
    ) -> Result<(), CoreError> {
        self.api
            .revoke_resource_grant(&resource_type, &resource_id, grant_id)
            .await?;
        Ok(())
    }
}
