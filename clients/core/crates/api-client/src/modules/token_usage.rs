use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn get_token_usage_dashboard(
        &self,
        start_time: Option<&str>,
        end_time: Option<&str>,
        agent_slug: Option<&str>,
        user_id: Option<i64>,
        model: Option<&str>,
        granularity: Option<&str>,
    ) -> Result<TokenUsageDashboard, ApiError> {
        let mut path = self.org_path("/token-usage/dashboard");
        let mut params = Vec::new();
        if let Some(v) = start_time {
            params.push(format!("start_time={v}"));
        }
        if let Some(v) = end_time {
            params.push(format!("end_time={v}"));
        }
        if let Some(v) = agent_slug {
            params.push(format!("agent_slug={v}"));
        }
        if let Some(v) = user_id {
            params.push(format!("user_id={v}"));
        }
        if let Some(v) = model {
            params.push(format!("model={v}"));
        }
        if let Some(v) = granularity {
            params.push(format!("granularity={v}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }
}
