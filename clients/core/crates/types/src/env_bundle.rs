use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EnvBundle is a named, owner-scoped set of environment variables an
/// AgentFile can reference via `USE_ENV_BUNDLE "name"`. The wire shape
/// mirrors the backend's `domain/envbundle.EnvBundle` exactly.
///
/// `kind` is a free-form string (no enum constraint) — code defines the
/// recognized values: `credential`, `runtime`, `shared`. UI layers decide
/// rendering per kind (e.g. credential → password inputs).
///
/// `data` holds plaintext values for non-secret kinds; for `credential`
/// kind the backend strips values from the wire (only `configured_fields`
/// is populated), matching the legacy credential-profile UX contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvBundle {
    pub id: i64,
    pub owner_scope: String,
    pub owner_id: i64,
    #[serde(default)]
    pub agent_slug: Option<String>,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub kind_primary: bool,
    #[serde(default)]
    pub is_active: bool,
    /// Names of every key set on this bundle. Always populated.
    #[serde(default)]
    pub configured_fields: Option<Vec<String>>,
    /// Plaintext values, present only for non-secret kinds. Secret bundles
    /// expose keys via `configured_fields` but never echo values back.
    #[serde(default)]
    pub configured_values: Option<HashMap<String, String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEnvBundleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_slug: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub kind_primary: bool,
    /// Plaintext KV map. Backend encrypts per-value for credential kind.
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEnvBundleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_primary: Option<bool>,
    /// Replacement KV map. Empty/absent means "no change".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// Backend wire format: `GET /api/v1/users/env-bundles[?kind=…&agent_slug=…]`
/// returns `{"items": [bundle, bundle, ...]}` — flat list, filtering happens
/// via query params, not by grouping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvBundleListResponse {
    #[serde(default)]
    pub items: Vec<EnvBundle>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_bundle_list_decodes_backend_shape() {
        let json = r#"{
            "items": [
                {
                    "id": 7,
                    "owner_scope": "user",
                    "owner_id": 1,
                    "agent_slug": "claude-code",
                    "name": "work",
                    "kind": "credential",
                    "kind_primary": true,
                    "is_active": true,
                    "configured_fields": ["ANTHROPIC_API_KEY"],
                    "created_at": "2026-05-18T00:00:00Z",
                    "updated_at": "2026-05-18T00:00:00Z"
                }
            ]
        }"#;
        let resp: EnvBundleListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].name, "work");
        assert!(resp.items[0].kind_primary);
        assert_eq!(
            resp.items[0].configured_fields.as_ref().unwrap()[0],
            "ANTHROPIC_API_KEY"
        );
    }

    #[test]
    fn env_bundle_list_tolerates_empty() {
        let resp: EnvBundleListResponse = serde_json::from_str(r#"{"items":[]}"#).unwrap();
        assert!(resp.items.is_empty());
    }

    #[test]
    fn runtime_kind_echoes_configured_values() {
        let json = r#"{
            "id": 1, "owner_scope": "user", "owner_id": 1,
            "name": "runtime-defaults", "kind": "runtime",
            "kind_primary": false, "is_active": true,
            "configured_fields": ["LOG_LEVEL", "MODEL"],
            "configured_values": {"LOG_LEVEL": "debug", "MODEL": "opus"},
            "created_at": "x", "updated_at": "x"
        }"#;
        let b: EnvBundle = serde_json::from_str(json).unwrap();
        let values = b.configured_values.unwrap();
        assert_eq!(values.get("LOG_LEVEL").unwrap(), "debug");
        assert_eq!(values.get("MODEL").unwrap(), "opus");
    }
}
