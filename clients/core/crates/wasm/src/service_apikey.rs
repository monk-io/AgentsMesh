use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmApiKeyService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmApiKeyService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<String, String> {
        let resp = self.client.list_api_keys().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get(&self, id: i64) -> Result<String, String> {
        let resp = self.client.get_api_key(id).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        let req: CreateApiKeyRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.create_api_key(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn update(&self, id: i64, json: &str) -> Result<String, String> {
        let req: UpdateApiKeyRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.update_api_key(id, &req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.client.delete_api_key(id).await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }

    pub async fn revoke(&self, id: i64) -> Result<(), String> {
        self.client.revoke_api_key(id).await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }
}
