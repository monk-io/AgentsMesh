use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_provider_preserves_backend_fields() {
        let backend_json = r#"{
            "id": 42,
            "provider_type": "github",
            "name": "GitHub",
            "base_url": "https://github.com",
            "has_client_id": false,
            "has_bot_token": false,
            "has_identity": true,
            "is_default": true,
            "is_active": true,
            "created_at": "2026-05-06T00:00:00Z",
            "updated_at": "2026-05-06T00:00:00Z"
        }"#;

        let provider: RepositoryProvider = serde_json::from_str(backend_json).unwrap();
        assert_eq!(provider.is_active, Some(true));
        assert_eq!(provider.has_identity, Some(true));
        assert_eq!(provider.has_bot_token, Some(false));
        assert_eq!(provider.has_client_id, Some(false));

        let reserialized = serde_json::to_value(&provider).unwrap();
        assert_eq!(reserialized["is_active"], serde_json::json!(true));
        assert_eq!(reserialized["has_identity"], serde_json::json!(true));
        assert_eq!(reserialized["has_bot_token"], serde_json::json!(false));
        assert_eq!(reserialized["has_client_id"], serde_json::json!(false));
    }

    #[test]
    fn repository_provider_tolerates_missing_optional_fields() {
        let minimal_json = r#"{
            "id": 1,
            "provider_type": "gitlab",
            "name": "GitLab"
        }"#;

        let provider: RepositoryProvider = serde_json::from_str(minimal_json).unwrap();
        assert_eq!(provider.is_active, None);
        assert_eq!(provider.has_identity, None);
    }

    const PROVIDER_REPO_PAYLOAD: &str = r#"{
        "id": "987654",
        "name": "infra-tools",
        "slug": "infra-tools",
        "description": "Internal infrastructure helpers",
        "default_branch": "main",
        "visibility": "private",
        "http_clone_url": "https://gitlab.example.com/group/infra-tools.git",
        "ssh_clone_url": "git@gitlab.example.com:group/infra-tools.git",
        "web_url": "https://gitlab.example.com/group/infra-tools"
    }"#;

    #[test]
    fn provider_repository_decodes_backend_payload() {
        let r: ProviderRepository = serde_json::from_str(PROVIDER_REPO_PAYLOAD).unwrap();
        assert_eq!(r.name, "infra-tools");
        assert_eq!(r.description.as_deref(), Some("Internal infrastructure helpers"));
        assert_eq!(
            r.http_clone_url.as_deref(),
            Some("https://gitlab.example.com/group/infra-tools.git"),
        );
        assert_eq!(
            r.ssh_clone_url.as_deref(),
            Some("git@gitlab.example.com:group/infra-tools.git"),
        );
    }

    #[test]
    fn provider_repository_wasm_relay_preserves_import_fields() {
        let typed: ProviderRepository = serde_json::from_str(PROVIDER_REPO_PAYLOAD).unwrap();
        let relayed = serde_json::to_string(&typed).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&relayed).unwrap();
        for key in ["name", "description", "http_clone_url", "ssh_clone_url",
                    "default_branch", "visibility", "web_url", "slug"] {
            assert!(!parsed[key].is_null(), "field `{key}` dropped by relay");
        }
    }
}
