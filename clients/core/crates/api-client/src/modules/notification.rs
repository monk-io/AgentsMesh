use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn get_notification_preferences(
        &self,
    ) -> Result<NotificationPreferenceListResponse, ApiError> {
        self.get(&self.org_path("/notifications/preferences")).await
    }

    pub async fn set_notification_preference(
        &self,
        data: &SetNotificationPreferenceRequest,
    ) -> Result<NotificationPreference, ApiError> {
        self.put(&self.org_path("/notifications/preferences"), data)
            .await
    }
}
