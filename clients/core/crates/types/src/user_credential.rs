use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredential {
    pub id: i64,
    pub name: String,
    pub credential_type: Option<String>,
    pub repository_provider_id: Option<i64>,
    pub host_pattern: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGitCredentialRequest {
    pub name: String,
    pub credential_type: String,
    pub repository_provider_id: Option<i64>,
    pub pat: Option<String>,
    pub private_key: Option<String>,
    pub host_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGitCredentialRequest {
    pub name: Option<String>,
    pub pat: Option<String>,
    pub private_key: Option<String>,
    pub host_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDefaultGitCredentialRequest {
    pub credential_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredentialListResponse {
    #[serde(default)]
    pub credentials: Vec<GitCredential>,
    #[serde(default)]
    pub runner_local: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryProvider {
    pub id: i64,
    pub provider_type: String,
    pub name: String,
    pub base_url: Option<String>,
    #[serde(default)]
    pub has_client_id: Option<bool>,
    #[serde(default)]
    pub has_bot_token: Option<bool>,
    #[serde(default)]
    pub has_identity: Option<bool>,
    pub is_default: Option<bool>,
    #[serde(default)]
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryProviderRequest {
    pub provider_type: String,
    pub name: String,
    pub base_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub bot_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepositoryProviderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryProviderListResponse {
    pub providers: Vec<RepositoryProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRepository {
    pub id: Option<String>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: Option<String>,
    pub http_clone_url: Option<String>,
    pub ssh_clone_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRepositoryListResponse {
    pub repositories: Vec<ProviderRepository>,
    pub total: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_provider_preserves_backend_fields() {
        let json = r#"{
            "id": 42, "provider_type": "github", "name": "GitHub",
            "base_url": "https://github.com",
            "has_client_id": false, "has_bot_token": false, "has_identity": true,
            "is_default": true, "is_active": true,
            "created_at": "2026-05-06T00:00:00Z",
            "updated_at": "2026-05-06T00:00:00Z"
        }"#;
        let p: RepositoryProvider = serde_json::from_str(json).unwrap();
        assert_eq!(p.is_active, Some(true));
        assert_eq!(p.has_identity, Some(true));
    }

    #[test]
    fn update_repository_provider_request_skips_omitted_fields() {
        let req = UpdateRepositoryProviderRequest {
            name: Some("Renamed".to_string()),
            base_url: None, client_id: None, client_secret: None,
            bot_token: None, is_active: None,
        };
        let body = serde_json::to_value(&req).unwrap();
        assert_eq!(body["name"], serde_json::json!("Renamed"));
        assert!(body.get("is_active").is_none());
    }
}
