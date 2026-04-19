use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmUserApiService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmUserApiService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_me(&self) -> Result<String, String> {
        let resp = self.client.get_me().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_organizations(&self) -> Result<String, String> {
        let resp = self.client.get_organizations().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
