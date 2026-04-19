use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_user_repository_providers(
        &self,
    ) -> Result<RepositoryProviderListResponse, ApiError> {
        self.get("/api/v1/users/repository-providers").await
    }

    pub async fn create_user_repository_provider(
        &self,
        data: &CreateRepositoryProviderRequest,
    ) -> Result<RepositoryProvider, ApiError> {
        self.post_resource("/api/v1/users/repository-providers", data, "provider").await
    }

    pub async fn get_user_repository_provider(
        &self,
        id: i64,
    ) -> Result<RepositoryProvider, ApiError> {
        self.get_resource(&format!("/api/v1/users/repository-providers/{id}"), "provider").await
    }

    pub async fn update_user_repository_provider(
        &self,
        id: i64,
        data: &UpdateRepositoryProviderRequest,
    ) -> Result<RepositoryProvider, ApiError> {
        self.put_resource(&format!("/api/v1/users/repository-providers/{id}"), data, "provider").await
    }

    pub async fn delete_user_repository_provider(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/users/repository-providers/{id}"))
            .await
    }

    pub async fn set_default_repository_provider(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &format!("/api/v1/users/repository-providers/{id}/default"),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn test_repository_provider_connection(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &format!("/api/v1/users/repository-providers/{id}/test"),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn list_provider_repositories(
        &self,
        id: i64,
        page: Option<u32>,
        per_page: Option<u32>,
        search: Option<&str>,
    ) -> Result<ProviderRepositoryListResponse, ApiError> {
        let mut path = format!("/api/v1/users/repository-providers/{id}/repositories");
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={p}"));
        }
        if let Some(pp) = per_page {
            params.push(format!("per_page={pp}"));
        }
        if let Some(s) = search {
            params.push(format!("search={s}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }
}
