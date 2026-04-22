use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSODiscoverResponse {
    pub configs: Vec<crate::SSOConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOAuthUrlParams {
    pub domain: String,
    pub protocol: String,
    pub redirect: Option<String>,
}
