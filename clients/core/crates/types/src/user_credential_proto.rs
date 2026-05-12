// Hand-maintained `prost::Message` mirrors of
// `proto/user_credential/v1/user_credential.proto`. Tag numbers match the
// .proto byte-for-byte; `tools/validate_prost_tags` runs at build time to
// catch drift (watch list §8). NO `Serialize`/`Deserialize` derives on
// these structs — binary wire only (conventions §2.5, §3).
//
// PR #329 lineage: RepositoryProvider preserves every backend field —
// `has_client_id`, `has_bot_token`, `has_identity`, `is_active`. The legacy
// serde RepositoryProvider in user_credential.rs drifted on these flags;
// the proto SSOT pins them.
//
// Three Connect services share this module because they live in the same
// proto package (proto.user_credential.v1). Splitting into three files
// would duplicate the `mod` plumbing without semantic benefit.

// ============================================================================
// UserGitCredentialService
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct GitCredential {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub credential_type: String,
    #[prost(int64, optional, tag = "4")]
    pub repository_provider_id: Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub provider_name: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub public_key: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub fingerprint: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub host_pattern: Option<String>,
    #[prost(bool, tag = "9")]
    pub is_default: bool,
    #[prost(string, tag = "10")]
    pub created_at: String,
    #[prost(string, tag = "11")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListGitCredentialsRequest {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListGitCredentialsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<GitCredential>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
    #[prost(bool, tag = "5")]
    pub runner_local_is_default: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetGitCredentialRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateGitCredentialRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub credential_type: String,
    #[prost(int64, optional, tag = "3")]
    pub repository_provider_id: Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub pat: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub private_key: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub host_pattern: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateGitCredentialRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub pat: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub private_key: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub host_pattern: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteGitCredentialRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteGitCredentialResponse {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetDefaultGitCredentialRequest {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetDefaultGitCredentialResponse {
    #[prost(message, optional, tag = "1")]
    pub credential: Option<GitCredential>,
    #[prost(bool, tag = "2")]
    pub is_runner_local: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultGitCredentialRequest {
    #[prost(int64, optional, tag = "1")]
    pub credential_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultGitCredentialResponse {
    #[prost(bool, tag = "1")]
    pub is_runner_local: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ClearDefaultGitCredentialRequest {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ClearDefaultGitCredentialResponse {
}

// ============================================================================
// UserAgentCredentialService
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct AgentCredentialProfile {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub agent_slug: String,
    #[prost(string, tag = "4")]
    pub name: String,
    #[prost(string, optional, tag = "5")]
    pub description: Option<String>,
    #[prost(bool, tag = "6")]
    pub is_runner_host: bool,
    #[prost(bool, tag = "7")]
    pub is_default: bool,
    #[prost(bool, tag = "8")]
    pub is_active: bool,
    #[prost(string, repeated, tag = "9")]
    pub configured_fields: Vec<String>,
    #[prost(map = "string, string", tag = "10")]
    pub configured_values: ::std::collections::HashMap<String, String>,
    #[prost(string, optional, tag = "11")]
    pub agent_name: Option<String>,
    #[prost(string, tag = "12")]
    pub created_at: String,
    #[prost(string, tag = "13")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CredentialProfilesByAgent {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
    #[prost(string, tag = "2")]
    pub agent_name: String,
    #[prost(message, repeated, tag = "3")]
    pub profiles: Vec<AgentCredentialProfile>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAgentCredentialProfilesRequest {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAgentCredentialProfilesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<CredentialProfilesByAgent>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAgentCredentialProfilesForAgentRequest {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RunnerHostInfo {
    #[prost(bool, tag = "1")]
    pub available: bool,
    #[prost(string, tag = "2")]
    pub description: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAgentCredentialProfilesForAgentResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<AgentCredentialProfile>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
    #[prost(message, optional, tag = "5")]
    pub runner_host: Option<RunnerHostInfo>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAgentCredentialProfileRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateAgentCredentialProfileRequest {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(bool, tag = "4")]
    pub is_runner_host: bool,
    #[prost(map = "string, string", tag = "5")]
    pub credentials: ::std::collections::HashMap<String, String>,
    #[prost(bool, tag = "6")]
    pub is_default: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateAgentCredentialProfileRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(bool, optional, tag = "4")]
    pub is_runner_host: Option<bool>,
    #[prost(map = "string, string", tag = "5")]
    pub credentials: ::std::collections::HashMap<String, String>,
    #[prost(bool, optional, tag = "6")]
    pub is_default: Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_active: Option<bool>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteAgentCredentialProfileRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteAgentCredentialProfileResponse {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultAgentCredentialProfileRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

// ============================================================================
// UserRepositoryProviderService
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct RepositoryProvider {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub provider_type: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(string, tag = "4")]
    pub base_url: String,
    #[prost(bool, tag = "5")]
    pub has_client_id: bool,
    #[prost(bool, tag = "6")]
    pub has_bot_token: bool,
    #[prost(bool, tag = "7")]
    pub has_identity: bool,
    #[prost(bool, tag = "8")]
    pub is_default: bool,
    #[prost(bool, tag = "9")]
    pub is_active: bool,
    #[prost(string, tag = "10")]
    pub created_at: String,
    #[prost(string, tag = "11")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryProvidersRequest {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryProvidersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<RepositoryProvider>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRepositoryProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateRepositoryProviderRequest {
    #[prost(string, tag = "1")]
    pub provider_type: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub base_url: String,
    #[prost(string, optional, tag = "4")]
    pub client_id: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub client_secret: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub bot_token: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateRepositoryProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub base_url: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub client_id: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub client_secret: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub bot_token: Option<String>,
    #[prost(bool, optional, tag = "7")]
    pub is_active: Option<bool>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryProviderResponse {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultRepositoryProviderRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetDefaultRepositoryProviderResponse {
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TestRepositoryProviderConnectionRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TestRepositoryProviderConnectionResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ProviderRepository {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub slug: String,
    #[prost(string, tag = "4")]
    pub description: String,
    #[prost(string, tag = "5")]
    pub default_branch: String,
    #[prost(string, tag = "6")]
    pub visibility: String,
    #[prost(string, tag = "7")]
    pub http_clone_url: String,
    #[prost(string, tag = "8")]
    pub ssh_clone_url: String,
    #[prost(string, tag = "9")]
    pub web_url: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListProviderRepositoriesRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int32, optional, tag = "2")]
    pub page: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub per_page: Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub search: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListProviderRepositoriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<ProviderRepository>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_git_credential() -> GitCredential {
        GitCredential {
            id: 42,
            name: "github-pat".into(),
            credential_type: "pat".into(),
            repository_provider_id: Some(7),
            provider_name: Some("GitHub".into()),
            public_key: None,
            fingerprint: None,
            host_pattern: Some("github.com".into()),
            is_default: true,
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-10T00:00:00Z".into(),
        }
    }

    #[test]
    fn git_credential_round_trip_preserves_every_field() {
        let original = sample_git_credential();
        let bytes = original.encode_to_vec();
        let decoded = GitCredential::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn list_git_credentials_response_envelope_round_trip() {
        let original = ListGitCredentialsResponse {
            items: vec![sample_git_credential()],
            total: 1,
            limit: 50,
            offset: 0,
            runner_local_is_default: false,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListGitCredentialsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn repository_provider_preserves_all_flag_fields() {
        // PR #329: legacy Rust DTO drifted on has_identity, has_bot_token,
        // is_active. Proto SSOT pins them.
        let original = RepositoryProvider {
            id: 1,
            provider_type: "github".into(),
            name: "GitHub".into(),
            base_url: "https://github.com".into(),
            has_client_id: false,
            has_bot_token: false,
            has_identity: true,
            is_default: true,
            is_active: true,
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-01T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = RepositoryProvider::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.has_identity);
        assert!(decoded.is_active);
        assert!(!decoded.has_bot_token);
    }

    #[test]
    fn agent_credential_profile_round_trip_with_map() {
        let mut values = ::std::collections::HashMap::new();
        values.insert("base_url".into(), "https://api.example.com".into());
        let original = AgentCredentialProfile {
            id: 1,
            user_id: 42,
            agent_slug: "claude".into(),
            name: "Work".into(),
            description: Some("Work credentials".into()),
            is_runner_host: false,
            is_default: true,
            is_active: true,
            configured_fields: vec!["api_key".into(), "base_url".into()],
            configured_values: values,
            agent_name: Some("Claude".into()),
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-01T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = AgentCredentialProfile::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn set_default_credential_id_absent_vs_present_distinguishable() {
        // Conventions §5: absent credential_id means "set runner_local as
        // default", explicit value means "set this credential". Proto3
        // optional preserves the distinction.
        let runner_local = SetDefaultGitCredentialRequest { credential_id: None };
        let explicit = SetDefaultGitCredentialRequest { credential_id: Some(5) };
        assert_ne!(runner_local.encode_to_vec(), explicit.encode_to_vec());
    }

    #[test]
    fn create_git_credential_response_is_entity() {
        // Conventions §9: CreateGitCredential returns the entity directly.
        // No {credential: ...} wrapper. Pinned by encoding identity.
        let entity = sample_git_credential();
        let bytes = entity.encode_to_vec();
        let decoded = GitCredential::decode(&*bytes).unwrap();
        assert_eq!(entity, decoded);
    }

    #[test]
    fn provider_repository_preserves_every_field() {
        let original = ProviderRepository {
            id: "987654".into(),
            name: "infra-tools".into(),
            slug: "infra-tools".into(),
            description: "Internal infrastructure helpers".into(),
            default_branch: "main".into(),
            visibility: "private".into(),
            http_clone_url: "https://gitlab.example.com/group/infra-tools.git".into(),
            ssh_clone_url: "git@gitlab.example.com:group/infra-tools.git".into(),
            web_url: "https://gitlab.example.com/group/infra-tools".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = ProviderRepository::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn delete_request_response_round_trip() {
        let req = DeleteGitCredentialRequest { id: 99 };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, DeleteGitCredentialRequest::decode(&*req_bytes).unwrap());

        let resp = DeleteGitCredentialResponse {};
        let resp_bytes = resp.encode_to_vec();
        assert!(resp_bytes.is_empty());
        assert_eq!(resp, DeleteGitCredentialResponse::decode(&*resp_bytes).unwrap());
    }
}
