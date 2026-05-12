use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTokenUsageService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmTokenUsageService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
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
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    // -------- Connect-RPC (binary wire) --------

    #[wasm_bindgen(js_name = getDashboardConnect)]
    pub async fn get_dashboard_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        let req = agentsmesh_types::proto_token_usage_v1::GetDashboardRequest::decode(request)
            .map_err(|e| format!("decode get_dashboard request: {e}"))?;
        let resp = self.client.get_token_usage_dashboard_connect(&req).await
            .map_err(agentsmesh_services::wire)?;
        Ok(resp.encode_to_vec())
    }
}
