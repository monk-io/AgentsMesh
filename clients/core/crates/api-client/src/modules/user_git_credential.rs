use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_user_git_credentials(
        &self,
    ) -> Result<GitCredentialListResponse, ApiError> {
        self.get("/api/v1/users/git-credentials").await
    }

    pub async fn create_user_git_credential(
        &self,
        data: &CreateGitCredentialRequest,
    ) -> Result<GitCredential, ApiError> {
        self.post_resource("/api/v1/users/git-credentials", data, "credential").await
    }

    pub async fn get_user_git_credential(
        &self,
        id: i64,
    ) -> Result<GitCredential, ApiError> {
        self.get_resource(&format!("/api/v1/users/git-credentials/{id}"), "credential").await
    }

    pub async fn update_user_git_credential(
        &self,
        id: i64,
        data: &UpdateGitCredentialRequest,
    ) -> Result<GitCredential, ApiError> {
        self.put_resource(&format!("/api/v1/users/git-credentials/{id}"), data, "credential").await
    }

    pub async fn delete_user_git_credential(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/users/git-credentials/{id}"))
            .await
    }

    pub async fn get_default_git_credential(&self) -> Result<GitCredential, ApiError> {
        self.get_resource("/api/v1/users/git-credentials/default", "credential").await
    }

    pub async fn set_default_git_credential(
        &self,
        data: &SetDefaultGitCredentialRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post("/api/v1/users/git-credentials/default", data)
            .await
    }

    pub async fn clear_default_git_credential(&self) -> Result<EmptyResponse, ApiError> {
        self.delete("/api/v1/users/git-credentials/default").await
    }
}
