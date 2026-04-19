use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmNotificationService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmNotificationService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_preferences(&self) -> Result<String, String> {
        let resp = self.client
            .get_notification_preferences()
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn set_preference(&self, json: &str) -> Result<String, String> {
        let req: SetNotificationPreferenceRequest =
            serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client
            .set_notification_preference(&req)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
