use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

/// Frontend-facing facade for the env-bundle REST API. Everything goes
/// through JSON-string boundaries so the wasm wrapper has nothing to do
/// except pass strings through to the renderer.
pub struct EnvBundleService {
    client: Arc<ApiClient>,
}

impl EnvBundleService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(&self, kind: Option<&str>, agent_slug: Option<&str>) -> Result<String, String> {
        let resp = self
            .client
            .list_user_env_bundles(kind, agent_slug)
            .await
            .map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get(&self, id: i64) -> Result<String, String> {
        let resp = self.client.get_user_env_bundle(id).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        let req: CreateEnvBundleRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.create_user_env_bundle(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update(&self, id: i64, json: &str) -> Result<String, String> {
        let req: UpdateEnvBundleRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.update_user_env_bundle(id, &req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.client.delete_user_env_bundle(id).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn set_primary(&self, id: i64) -> Result<String, String> {
        let resp = self.client.set_primary_env_bundle(id).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
