use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::NotificationService;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmNotificationService {
    client: Arc<ApiClient>,
    svc: NotificationService,
}

#[wasm_bindgen]
impl WasmNotificationService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        let svc = NotificationService::new(client.clone());
        Self { client, svc }
    }

    pub async fn get_preferences(&self) -> Result<String, String> {
        let resp = self.client
            .get_notification_preferences()
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn set_preference(&self, json: &str) -> Result<String, String> {
        let req: SetNotificationPreferenceRequest =
            serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client
            .set_notification_preference(&req)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    // -------- Connect-RPC (binary wire) --------

    #[wasm_bindgen(js_name = listPreferencesConnect)]
    pub async fn list_preferences_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.svc.list_preferences_connect(request).await
    }

    #[wasm_bindgen(js_name = setPreferenceConnect)]
    pub async fn set_preference_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.svc.set_preference_connect(request).await
    }
}

