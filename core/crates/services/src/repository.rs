use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct RepositoryService {
    client: Arc<ApiClient>,
}

impl RepositoryService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<String, String> {
        let resp = self.client.list_repositories().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get(&self, id: i64) -> Result<String, String> {
        let resp = self.client.get_repository(id).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        let req: CreateRepositoryRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client.create_repository(&req).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: i64, json: &str) -> Result<String, String> {
        let req: UpdateRepositoryRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client.update_repository(id, &req).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.client.delete_repository(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_branches(&self, id: i64) -> Result<String, String> {
        let resp = self.client.list_repository_branches(id).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn sync_branches(&self, id: i64, json: &str) -> Result<String, String> {
        let req: SyncBranchesRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client
            .sync_repository_branches(id, &req)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn register_webhook(&self, id: i64) -> Result<(), String> {
        self.client.register_repository_webhook(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete_webhook(&self, id: i64) -> Result<(), String> {
        self.client.delete_repository_webhook(id).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_webhook_status(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_repository_webhook_status(id)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_webhook_secret(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_repository_webhook_secret(id)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn list_merge_requests(
        &self, id: i64, branch: Option<String>, state: Option<String>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_repository_merge_requests(id, branch.as_deref(), state.as_deref())
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn mark_webhook_configured(&self, id: i64) -> Result<(), String> {
        let path = self.client.org_path(&format!("/repositories/{id}/webhook/configured"));
        self.client
            .post::<agentsmesh_types::EmptyResponse>(&path, &serde_json::json!({}))
            .await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
