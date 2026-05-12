// Hand-maintained `prost::Message` mirrors of
// `proto/pod/v1/agentpod_settings.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` enforces it (watch list §8).
// No `Serialize` / `Deserialize` derives — binary wire only (conventions §2.5).
//
// User-scoped — no org_slug field. Tenant comes from the auth interceptor.

use std::collections::HashMap;

#[derive(Clone, PartialEq, prost::Message)]
pub struct AgentPodSettings {
    #[prost(string, optional, tag = "1")]
    pub default_agent_slug: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub default_model: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub default_perm_mode: Option<String>,
    #[prost(int32, optional, tag = "4")]
    pub terminal_font_size: Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub terminal_theme: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AIProvider {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub provider_type: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(bool, tag = "4")]
    pub is_default: bool,
    #[prost(bool, tag = "5")]
    pub is_enabled: bool,
    #[prost(string, tag = "6")]
    pub created_at: String,
    #[prost(string, tag = "7")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetSettingsRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateSettingsRequest {
    #[prost(string, optional, tag = "1")]
    pub default_agent_slug: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub default_model: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub default_perm_mode: Option<String>,
    #[prost(int32, optional, tag = "4")]
    pub terminal_font_size: Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub terminal_theme: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListProvidersRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListProvidersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<AIProvider>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateProviderRequest {
    #[prost(string, tag = "1")]
    pub provider_type: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(map = "string, string", tag = "3")]
    pub credentials: HashMap<String, String>,
    #[prost(bool, tag = "4")]
    pub is_default: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(map = "string, string", tag = "3")]
    pub credentials: HashMap<String, String>,
    #[prost(bool, optional, tag = "4")]
    pub is_default: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub is_enabled: Option<bool>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteProviderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultProviderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn ai_provider_round_trip() {
        let original = AIProvider {
            id: 7,
            provider_type: "claude".into(),
            name: "My Claude".into(),
            is_default: true,
            is_enabled: true,
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = AIProvider::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn settings_optionals_distinguishable() {
        // Update with one field set, others absent → server should only
        // touch the present fields.
        let req = UpdateSettingsRequest {
            default_agent_slug: Some("claude-code".into()),
            default_model: None,
            default_perm_mode: None,
            terminal_font_size: None,
            terminal_theme: None,
        };
        let bytes = req.encode_to_vec();
        let decoded = UpdateSettingsRequest::decode(&*bytes).unwrap();
        assert_eq!(decoded.default_agent_slug.as_deref(), Some("claude-code"));
        assert!(decoded.default_model.is_none());
    }

    #[test]
    fn create_provider_with_credentials_map() {
        let mut creds = HashMap::new();
        creds.insert("api_key".into(), "secret-xyz".into());
        creds.insert("base_url".into(), "https://api.example.com".into());
        let original = CreateProviderRequest {
            provider_type: "openai".into(),
            name: "My OpenAI".into(),
            credentials: creds.clone(),
            is_default: false,
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateProviderRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.credentials.get("api_key").unwrap(), "secret-xyz");
    }

    #[test]
    fn list_providers_response_envelope() {
        let original = ListProvidersResponse {
            items: vec![AIProvider {
                id: 1,
                provider_type: "claude".into(),
                name: "P1".into(),
                is_default: true,
                is_enabled: true,
                created_at: "2026-05-01T00:00:00Z".into(),
                updated_at: "2026-05-01T00:00:00Z".into(),
            }],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListProvidersResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn delete_provider_response_round_trip() {
        let original = DeleteProviderResponse { message: "Provider deleted".into() };
        let bytes = original.encode_to_vec();
        let decoded = DeleteProviderResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
