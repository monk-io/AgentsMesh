use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_repositories(&self) -> Result<RepositoryListResponse, ApiError> {
        self.get(&self.org_path("/repositories")).await
    }

    pub async fn get_repository(&self, id: i64) -> Result<Repository, ApiError> {
        self.get_resource(&self.org_path(&format!("/repositories/{id}")), "repository").await
    }

    pub async fn create_repository(
        &self,
        data: &CreateRepositoryRequest,
    ) -> Result<Repository, ApiError> {
        self.post_resource(&self.org_path("/repositories"), data, "repository").await
    }

    pub async fn update_repository(
        &self,
        id: i64,
        data: &UpdateRepositoryRequest,
    ) -> Result<Repository, ApiError> {
        self.put_resource(&self.org_path(&format!("/repositories/{id}")), data, "repository").await
    }

    pub async fn delete_repository(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/repositories/{id}")))
            .await
    }

    pub async fn list_repository_branches(
        &self,
        id: i64,
    ) -> Result<BranchListResponse, ApiError> {
        self.get(&self.org_path(&format!("/repositories/{id}/branches")))
            .await
    }

    pub async fn sync_repository_branches(
        &self,
        id: i64,
        data: &SyncBranchesRequest,
    ) -> Result<BranchListResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/repositories/{id}/sync-branches")),
            data,
        )
        .await
    }

    pub async fn register_repository_webhook(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/repositories/{id}/webhook")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn delete_repository_webhook(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/repositories/{id}/webhook")))
            .await
    }

    pub async fn get_repository_webhook_status(
        &self,
        id: i64,
    ) -> Result<WebhookStatus, ApiError> {
        self.get_resource(&self.org_path(&format!("/repositories/{id}/webhook/status")), "webhook_status").await
    }

    pub async fn get_repository_webhook_secret(
        &self,
        id: i64,
    ) -> Result<WebhookSecret, ApiError> {
        self.get(&self.org_path(&format!("/repositories/{id}/webhook/secret")))
            .await
    }

    pub async fn list_repository_merge_requests(
        &self,
        id: i64,
        branch: Option<&str>,
        state: Option<&str>,
    ) -> Result<MergeRequestListResponse, ApiError> {
        let mut path = self.org_path(&format!("/repositories/{id}/merge-requests"));
        let mut params = Vec::new();
        if let Some(b) = branch {
            params.push(format!("branch={b}"));
        }
        if let Some(s) = state {
            params.push(format!("state={s}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }
}
