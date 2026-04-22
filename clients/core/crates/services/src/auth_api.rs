use std::sync::Arc;

use agentsmesh_api_client::ApiClient;

pub struct AuthApiService {
    client: Arc<ApiClient>,
}

impl AuthApiService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn register(&self, json: &str) -> Result<String, String> {
        let val: serde_json::Value = serde_json::from_str(json).map_err(crate::wire)?;
        let resp: serde_json::Value = self.client
            .public_post("/api/v1/auth/register", &val)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn verify_email(&self, token: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                &format!("/api/v1/auth/verify-email/{token}"),
                &serde_json::json!({}),
            )
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn resend_verification(&self, email: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                "/api/v1/auth/resend-verification",
                &serde_json::json!({ "email": email }),
            )
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn forgot_password(&self, email: &str) -> Result<String, String> {
        let resp: serde_json::Value = self.client
            .public_post(
                "/api/v1/auth/forgot-password",
                &serde_json::json!({ "email": email }),
            )
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn reset_password(&self, json: &str) -> Result<String, String> {
        let val: serde_json::Value = serde_json::from_str(json).map_err(crate::wire)?;
        let resp: serde_json::Value = self.client
            .public_post("/api/v1/auth/reset-password", &val)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
