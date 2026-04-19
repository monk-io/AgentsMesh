use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_invitations(&self) -> Result<InvitationListResponse, ApiError> {
        self.get(&self.org_path("/invitations")).await
    }

    pub async fn create_invitation(
        &self,
        data: &CreateInvitationRequest,
    ) -> Result<Invitation, ApiError> {
        self.post_resource(&self.org_path("/invitations"), data, "invitation").await
    }

    pub async fn revoke_invitation(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/invitations/{id}")))
            .await
    }

    pub async fn resend_invitation(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/invitations/{id}/resend")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn get_invitation_by_token(&self, token: &str) -> Result<Invitation, ApiError> {
        self.public_get_resource(&format!("/api/v1/invitations/{token}"), "invitation").await
    }

    pub async fn accept_invitation(&self, token: &str) -> Result<EmptyResponse, ApiError> {
        self.public_post(
            &format!("/api/v1/invitations/{token}/accept"),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn list_pending_invitations(&self) -> Result<InvitationListResponse, ApiError> {
        self.public_get("/api/v1/invitations/pending").await
    }
}
