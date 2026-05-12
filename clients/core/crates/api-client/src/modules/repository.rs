use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_repository_v1 as repo_proto;

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/repository/. Procedure paths derive from
// `proto.repository.v1.RepositoryService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn list_repositories_connect(
        &self,
        req: &repo_proto::ListRepositoriesRequest,
    ) -> Result<repo_proto::ListRepositoriesResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/ListRepositories",
            req,
        )
        .await
    }

    pub async fn get_repository_connect(
        &self,
        req: &repo_proto::GetRepositoryRequest,
    ) -> Result<repo_proto::Repository, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/GetRepository",
            req,
        )
        .await
    }

    pub async fn create_repository_connect(
        &self,
        req: &repo_proto::CreateRepositoryRequest,
    ) -> Result<repo_proto::Repository, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/CreateRepository",
            req,
        )
        .await
    }

    pub async fn update_repository_connect(
        &self,
        req: &repo_proto::UpdateRepositoryRequest,
    ) -> Result<repo_proto::Repository, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/UpdateRepository",
            req,
        )
        .await
    }

    pub async fn delete_repository_connect(
        &self,
        req: &repo_proto::DeleteRepositoryRequest,
    ) -> Result<repo_proto::DeleteRepositoryResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/DeleteRepository",
            req,
        )
        .await
    }

    pub async fn list_repository_branches_connect(
        &self,
        req: &repo_proto::ListRepositoryBranchesRequest,
    ) -> Result<repo_proto::ListRepositoryBranchesResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/ListRepositoryBranches",
            req,
        )
        .await
    }

    pub async fn sync_repository_branches_connect(
        &self,
        req: &repo_proto::SyncRepositoryBranchesRequest,
    ) -> Result<repo_proto::ListRepositoryBranchesResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/SyncRepositoryBranches",
            req,
        )
        .await
    }

    pub async fn list_repository_merge_requests_connect(
        &self,
        req: &repo_proto::ListRepositoryMergeRequestsRequest,
    ) -> Result<repo_proto::ListRepositoryMergeRequestsResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/ListRepositoryMergeRequests",
            req,
        )
        .await
    }

    pub async fn register_repository_webhook_connect(
        &self,
        req: &repo_proto::RegisterRepositoryWebhookRequest,
    ) -> Result<repo_proto::RegisterRepositoryWebhookResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/RegisterRepositoryWebhook",
            req,
        )
        .await
    }

    pub async fn delete_repository_webhook_connect(
        &self,
        req: &repo_proto::DeleteRepositoryWebhookRequest,
    ) -> Result<repo_proto::DeleteRepositoryWebhookResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/DeleteRepositoryWebhook",
            req,
        )
        .await
    }

    pub async fn get_repository_webhook_status_connect(
        &self,
        req: &repo_proto::GetRepositoryWebhookStatusRequest,
    ) -> Result<repo_proto::WebhookStatus, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/GetRepositoryWebhookStatus",
            req,
        )
        .await
    }

    pub async fn get_repository_webhook_secret_connect(
        &self,
        req: &repo_proto::GetRepositoryWebhookSecretRequest,
    ) -> Result<repo_proto::WebhookSecret, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/GetRepositoryWebhookSecret",
            req,
        )
        .await
    }

    pub async fn mark_repository_webhook_configured_connect(
        &self,
        req: &repo_proto::MarkRepositoryWebhookConfiguredRequest,
    ) -> Result<repo_proto::MarkRepositoryWebhookConfiguredResponse, ApiError> {
        connect_call(
            self,
            "/proto.repository.v1.RepositoryService/MarkRepositoryWebhookConfigured",
            req,
        )
        .await
    }
}

// =============================================================================
// Legacy REST methods — preserved for dual-track migration.
// =============================================================================

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
