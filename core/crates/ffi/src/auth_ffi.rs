use agentsmesh_types::RegisterRequest;

use crate::core::AgentsMeshCore;
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    pub async fn login(&self, email: String, password: String) -> Result<String, CoreError> {
        let session = self.auth.login(&email, &password).await?;
        Ok(serde_json::to_string(&session)?)
    }

    pub async fn register(
        &self,
        name: String,
        email: String,
        username: String,
        password: String,
    ) -> Result<String, CoreError> {
        let req = RegisterRequest {
            name,
            email,
            username,
            password,
        };
        let session = self.auth.register(&req).await?;
        Ok(serde_json::to_string(&session)?)
    }

    pub async fn logout(&self) -> Result<(), CoreError> {
        self.auth.logout().await.map_err(CoreError::from)
    }

    pub async fn refresh_token(&self) -> Result<String, CoreError> {
        let tokens = self.auth.refresh_token().await?;
        Ok(serde_json::to_string(&tokens)?)
    }

    pub async fn verify_email(&self, token: String) -> Result<String, CoreError> {
        let session = self.auth.verify_email(&token).await?;
        Ok(serde_json::to_string(&session)?)
    }

    pub async fn forgot_password(&self, email: String) -> Result<(), CoreError> {
        self.auth.forgot_password(&email).await.map_err(CoreError::from)
    }

    pub async fn reset_password(
        &self,
        token: String,
        new_password: String,
    ) -> Result<(), CoreError> {
        self.auth
            .reset_password(&token, &new_password)
            .await
            .map_err(CoreError::from)
    }

    pub async fn fetch_organizations(&self) -> Result<String, CoreError> {
        let orgs = self.auth.fetch_organizations().await?;
        Ok(serde_json::to_string(&orgs)?)
    }

    pub fn switch_org(&self, slug: String) -> Result<(), CoreError> {
        self.auth.switch_org(&slug).map_err(CoreError::from)
    }
}
