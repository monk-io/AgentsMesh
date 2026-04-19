use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmAuthApiService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmAuthApiService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn register(&self, json: &str) -> Result<String, String> {
        let val: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp: serde_json::Value = self.client
            .public_post("/api/v1/auth/register", &val)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn verify_email(&self, token: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                &format!("/api/v1/auth/verify-email/{token}"),
                &serde_json::json!({}),
            )
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn resend_verification(&self, email: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                "/api/v1/auth/resend-verification",
                &serde_json::json!({ "email": email }),
            )
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn forgot_password(&self, email: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                "/api/v1/auth/forgot-password",
                &serde_json::json!({ "email": email }),
            )
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn reset_password(&self, json: &str) -> Result<String, String> {
        let val: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp: serde_json::Value = self.client
            .public_post("/api/v1/auth/reset-password", &val)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
