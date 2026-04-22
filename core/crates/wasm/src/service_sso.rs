use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSSOService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmSSOService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn discover(&self, email: &str) -> Result<String, String> {
        let resp = self.client.sso_discover(email).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn ldap_auth(&self, domain: &str, json: &str) -> Result<String, String> {
        let req: LdapAuthRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.sso_ldap_auth(domain, &req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }
}
