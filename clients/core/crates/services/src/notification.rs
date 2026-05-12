use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::proto_notification_v1 as np;
use agentsmesh_types::*;
use prost::Message;

pub struct NotificationService {
    client: Arc<ApiClient>,
}

impl NotificationService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_preferences(&self) -> Result<String, String> {
        let resp = self.client
            .get_notification_preferences()
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn set_preference(&self, json: &str) -> Result<String, String> {
        let req: SetNotificationPreferenceRequest =
            serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .set_notification_preference(&req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Two lanes — request bytes in, response bytes out. The TS adapter
    // (`notificationConnect.ts`) is the only caller; the existing REST
    // methods above continue to populate any client-side cache during
    // the dual-track migration window.

    pub async fn list_preferences_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = np::ListPreferencesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_preferences request: {e}"))?;
        let resp = self.client.list_notification_preferences_connect(&req)
            .await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn set_preference_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = np::SetPreferenceRequest::decode(request_bytes)
            .map_err(|e| format!("decode set_preference request: {e}"))?;
        let resp = self.client.set_notification_preference_connect(&req)
            .await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}

