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
        let resp = self.client.sso_discover(email).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn ldap_auth(&self, domain: &str, json: &str) -> Result<String, String> {
        let req: LdapAuthRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.sso_ldap_auth(domain, &req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
