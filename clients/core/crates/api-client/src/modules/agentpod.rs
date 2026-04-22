use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn get_agentpod_settings(&self) -> Result<AgentPodSettings, ApiError> {
        self.get_resource("/api/v1/users/me/agentpod/settings", "settings").await
    }

    pub async fn update_agentpod_settings(
        &self,
        data: &AgentPodSettings,
    ) -> Result<AgentPodSettings, ApiError> {
        self.put("/api/v1/users/me/agentpod/settings", data).await
    }

    pub async fn list_agentpod_providers(&self) -> Result<AIProviderListResponse, ApiError> {
        self.get("/api/v1/users/me/agentpod/providers").await
    }

    pub async fn create_agentpod_provider(
        &self,
        data: &CreateAIProviderRequest,
    ) -> Result<AIProvider, ApiError> {
        self.post("/api/v1/users/me/agentpod/providers", data)
            .await
    }

    pub async fn update_agentpod_provider(
        &self,
        id: i64,
        data: &UpdateAIProviderRequest,
    ) -> Result<AIProvider, ApiError> {
        self.put(
            &format!("/api/v1/users/me/agentpod/providers/{id}"),
            data,
        )
        .await
    }

    pub async fn delete_agentpod_provider(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/users/me/agentpod/providers/{id}"))
            .await
    }

    pub async fn set_default_agentpod_provider(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &format!("/api/v1/users/me/agentpod/providers/{id}/default"),
            &serde_json::json!({}),
        )
        .await
    }
}
