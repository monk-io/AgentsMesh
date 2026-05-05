use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    pub source: Option<String>,
    pub entity_id: Option<String>,
    pub is_muted: Option<bool>,
    pub channels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetNotificationPreferenceRequest {
    pub source: String,
    pub entity_id: Option<String>,
    pub is_muted: Option<bool>,
    pub channels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferenceListResponse {
    pub preferences: Vec<NotificationPreference>,
}
