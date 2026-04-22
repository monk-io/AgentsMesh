use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

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
}
