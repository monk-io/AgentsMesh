use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::PromoCodeService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPromoCodeService {
    inner: PromoCodeService,
}

#[wasm_bindgen]
impl WasmPromoCodeService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { inner: PromoCodeService::new(client) }
    }

    // -------- Legacy REST JSON methods (preserved during dual-track) --------

    pub async fn validate(&self, json: &str) -> Result<String, String> {
        self.inner.validate(json).await
    }

    pub async fn redeem(&self, json: &str) -> Result<(), String> {
        self.inner.redeem(json).await
    }

    pub async fn get_history(&self) -> Result<String, String> {
        self.inner.get_history().await
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes the request via @bufbuild/protobuf .toBinary(), passes the
    // Uint8Array in, receives a Uint8Array back, decodes via .fromBinary().
    // No JSON intermediate; conventions §2.5 forbids it on the client.
    //
    // js_name is camelCase; the `Connect` suffix marks the migration lane so
    // the legacy JSON methods can coexist until the UI fully cuts over.

    #[wasm_bindgen(js_name = validatePromoCodeConnect)]
    pub async fn validate_promo_code_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.validate_promo_code_connect(request).await
    }

    #[wasm_bindgen(js_name = redeemPromoCodeConnect)]
    pub async fn redeem_promo_code_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.redeem_promo_code_connect(request).await
    }

    #[wasm_bindgen(js_name = getRedemptionHistoryConnect)]
    pub async fn get_redemption_history_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.get_redemption_history_connect(request).await
    }
}
