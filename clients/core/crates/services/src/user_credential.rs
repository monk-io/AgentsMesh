use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct UserCredentialService {
    client: Arc<ApiClient>,
}

impl UserCredentialService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list_git_credentials(&self) -> Result<String, String> {
        let resp = self.client.list_user_git_credentials().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_git_credential(&self, json: &str) -> Result<String, String> {
        let req: CreateGitCredentialRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.create_user_git_credential(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_git_credential(&self, id: i64) -> Result<String, String> {
        let resp = self.client.get_user_git_credential(id).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_git_credential(&self, id: i64, json: &str) -> Result<String, String> {
        let req: UpdateGitCredentialRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .update_user_git_credential(id, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete_git_credential(&self, id: i64) -> Result<(), String> {
        self.client.delete_user_git_credential(id).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn get_default_git_credential(&self) -> Result<String, String> {
        let resp = self.client.get_default_git_credential().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn set_default_git_credential(&self, json: &str) -> Result<(), String> {
        let req: SetDefaultGitCredentialRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        self.client.set_default_git_credential(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn clear_default_git_credential(&self) -> Result<(), String> {
        self.client.clear_default_git_credential().await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_agent_credentials(&self) -> Result<String, String> {
        let resp = self.client
            .list_user_agent_credentials()
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn list_agent_credentials_for_agent(
        &self, agent_slug: &str,
    ) -> Result<String, String> {
        let resp = self.client
            .list_user_agent_credentials_for_agent(agent_slug)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_agent_credential(
        &self, agent_slug: &str, json: &str,
    ) -> Result<String, String> {
        let req: CreateAgentCredentialProfileRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .create_user_agent_credential(agent_slug, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_agent_credential(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_user_agent_credential(id)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_agent_credential(
        &self, id: i64, json: &str,
    ) -> Result<String, String> {
        let req: UpdateAgentCredentialProfileRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .update_user_agent_credential(id, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete_agent_credential(&self, id: i64) -> Result<(), String> {
        self.client.delete_user_agent_credential(id).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn set_default_agent_credential(&self, id: i64) -> Result<(), String> {
        self.client.set_default_agent_credential(id).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_repo_providers(&self) -> Result<String, String> {
        let resp = self.client
            .list_user_repository_providers()
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_repo_provider(&self, json: &str) -> Result<String, String> {
        let req: CreateRepositoryProviderRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .create_user_repository_provider(&req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_repo_provider(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_user_repository_provider(id)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_repo_provider(&self, id: i64, json: &str) -> Result<String, String> {
        let req: UpdateRepositoryProviderRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .update_user_repository_provider(id, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete_repo_provider(&self, id: i64) -> Result<(), String> {
        self.client
            .delete_user_repository_provider(id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn set_default_repo_provider(&self, id: i64) -> Result<(), String> {
        self.client
            .set_default_repository_provider(id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn test_repo_provider(&self, id: i64) -> Result<(), String> {
        self.client
            .test_repository_provider_connection(id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_provider_repositories(
        &self,
        id: i64,
        page: Option<u32>,
        per_page: Option<u32>,
        search: Option<String>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_provider_repositories(id, page, per_page, search.as_deref())
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
