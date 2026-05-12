use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use agentsmesh_types::proto_promocode_v1 as pc_proto;
use prost::Message;

pub struct PromoCodeService {
    client: Arc<ApiClient>,
}

impl PromoCodeService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    // -------- Legacy REST (JSON wire) — preserved during dual-track --------

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

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes via @bufbuild/protobuf .toBinary() → wasm bridge → here →
    // ApiClient.*_connect (binary in / binary out, conventions §2.5). No
    // JSON path on the client.
    //
    // Each `*_connect` method takes prost-encoded bytes and returns
    // prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
    // surface. The TS adapter populates org_slug + code before encoding.

    pub async fn validate_promo_code_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = pc_proto::ValidatePromoCodeRequest::decode(request_bytes)
            .map_err(|e| format!("decode validate_promo_code request: {e}"))?;
        let resp = self.client.validate_promo_code_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn redeem_promo_code_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = pc_proto::RedeemPromoCodeRequest::decode(request_bytes)
            .map_err(|e| format!("decode redeem_promo_code request: {e}"))?;
        let resp = self.client.redeem_promo_code_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_redemption_history_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = pc_proto::GetRedemptionHistoryRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_redemption_history request: {e}"))?;
        let resp = self.client.get_redemption_history_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
