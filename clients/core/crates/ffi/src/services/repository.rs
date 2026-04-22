use agentsmesh_types::SyncBranchesRequest;

use crate::core::AgentsMeshCore;
use crate::dto::{
    BranchDto, CreateRepositoryRequestDto, MergeRequestListResponseDto, RepositoryDto,
    RepositoryListResponseDto, UpdateRepositoryRequestDto, WebhookSecretDto, WebhookStatusDto,
};
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    pub async fn list_repositories(&self) -> Result<RepositoryListResponseDto, CoreError> {
        let resp = self.api.list_repositories().await?;
        Ok(resp.into())
    }

    pub async fn get_repository(&self, id: i64) -> Result<RepositoryDto, CoreError> {
        let repo = self.api.get_repository(id).await?;
        Ok(repo.into())
    }

    pub async fn create_repository(
        &self,
        req: CreateRepositoryRequestDto,
    ) -> Result<RepositoryDto, CoreError> {
        let repo = self.api.create_repository(&req.into()).await?;
        Ok(repo.into())
    }

    pub async fn update_repository(
        &self,
        id: i64,
        req: UpdateRepositoryRequestDto,
    ) -> Result<RepositoryDto, CoreError> {
        let repo = self.api.update_repository(id, &req.into()).await?;
        Ok(repo.into())
    }

    pub async fn delete_repository(&self, id: i64) -> Result<(), CoreError> {
        self.api.delete_repository(id).await?;
        Ok(())
    }

    pub async fn list_repository_branches(&self, id: i64) -> Result<Vec<BranchDto>, CoreError> {
        let resp = self.api.list_repository_branches(id).await?;
        Ok(resp.branches.into_iter().map(BranchDto::from).collect())
    }

    pub async fn sync_repository_branches(
        &self,
        id: i64,
        access_token: Option<String>,
    ) -> Result<Vec<BranchDto>, CoreError> {
        let req = SyncBranchesRequest { access_token };
        let resp = self.api.sync_repository_branches(id, &req).await?;
        Ok(resp.branches.into_iter().map(BranchDto::from).collect())
    }

    pub async fn register_repository_webhook(&self, id: i64) -> Result<(), CoreError> {
        self.api.register_repository_webhook(id).await?;
        Ok(())
    }

    pub async fn delete_repository_webhook(&self, id: i64) -> Result<(), CoreError> {
        self.api.delete_repository_webhook(id).await?;
        Ok(())
    }

    pub async fn get_repository_webhook_status(
        &self,
        id: i64,
    ) -> Result<WebhookStatusDto, CoreError> {
        let status = self.api.get_repository_webhook_status(id).await?;
        Ok(status.into())
    }

    pub async fn get_repository_webhook_secret(
        &self,
        id: i64,
    ) -> Result<WebhookSecretDto, CoreError> {
        let secret = self.api.get_repository_webhook_secret(id).await?;
        Ok(secret.into())
    }

    pub async fn list_repository_merge_requests(
        &self,
        id: i64,
        branch: Option<String>,
        state: Option<String>,
    ) -> Result<MergeRequestListResponseDto, CoreError> {
        let resp = self
            .api
            .list_repository_merge_requests(id, branch.as_deref(), state.as_deref())
            .await?;
        Ok(resp.into())
    }
}
