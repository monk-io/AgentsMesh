use std::sync::Arc;

use agentsmesh_api_client::ApiClient;

pub struct TokenUsageService {
    client: Arc<ApiClient>,
}

impl TokenUsageService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_dashboard(
        &self,
        start_time: Option<String>,
        end_time: Option<String>,
        agent_slug: Option<String>,
        user_id: Option<i64>,
        model: Option<String>,
        granularity: Option<String>,
    ) -> Result<String, String> {
        let resp = self.client
            .get_token_usage_dashboard(
                start_time.as_deref(),
                end_time.as_deref(),
                agent_slug.as_deref(),
                user_id,
                model.as_deref(),
                granularity.as_deref(),
            )
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
