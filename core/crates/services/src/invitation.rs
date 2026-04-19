use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct InvitationService {
    client: Arc<ApiClient>,
}

impl InvitationService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<String, String> {
        let resp = self.client.list_invitations().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        let req: CreateInvitationRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client.create_invitation(&req).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn revoke(&self, id: i64) -> Result<(), String> {
        self.client.revoke_invitation(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn resend(&self, id: i64) -> Result<(), String> {
        self.client.resend_invitation(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_by_token(&self, token: &str) -> Result<String, String> {
        let resp = self.client.get_invitation_by_token(token).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn accept(&self, token: &str) -> Result<(), String> {
        self.client.accept_invitation(token).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_pending(&self) -> Result<String, String> {
        let resp = self.client.list_pending_invitations().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
