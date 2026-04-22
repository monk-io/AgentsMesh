use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPromoCodeService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmPromoCodeService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn validate(&self, json: &str) -> Result<String, String> {
        let req: ValidatePromoRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.validate_promo_code(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn redeem(&self, json: &str) -> Result<(), String> {
        let req: RedeemPromoRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        self.client.redeem_promo_code(&req).await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }

    pub async fn get_history(&self) -> Result<String, String> {
        let resp = self.client.get_promo_code_history().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }
}
