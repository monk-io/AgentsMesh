use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct PromoCodeService {
    client: Arc<ApiClient>,
}

impl PromoCodeService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn validate(&self, json: &str) -> Result<String, String> {
        let req: ValidatePromoRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.validate_promo_code(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn redeem(&self, json: &str) -> Result<(), String> {
        let req: RedeemPromoRequest = serde_json::from_str(json).map_err(crate::wire)?;
        self.client.redeem_promo_code(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn get_history(&self) -> Result<String, String> {
        let resp = self.client.get_promo_code_history().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
