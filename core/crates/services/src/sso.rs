use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct SSOService {
    client: Arc<ApiClient>,
}

impl SSOService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn discover(&self, email: &str) -> Result<String, String> {
        let resp = self.client.sso_discover(email).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn ldap_auth(&self, domain: &str, json: &str) -> Result<String, String> {
        let req: LdapAuthRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client.sso_ldap_auth(domain, &req).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
