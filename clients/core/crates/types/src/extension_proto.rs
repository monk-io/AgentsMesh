// Hand-maintained `prost::Message` mirrors of
// `proto/extension/v1/skill_registry.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` runs at build time to catch
// drift (watch list §8). NO `Serialize`/`Deserialize` derives on these
// structs — binary wire only (conventions §2.5, §3).
//
// Single point of risk per watch list #8: a swap between `tag = "N"` and the
// .proto field number is undetectable at compile time. Mitigations:
//   1. `tools/validate_prost_tags` parses both sides and asserts equality.
//   2. The round-trip test at the bottom of this file encodes every field
//      with a distinguishing value and decodes — a transposed tag pair
//      surfaces as field-value swap in the assertion.

#[derive(Clone, PartialEq, prost::Message)]
pub struct SkillRegistry {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, optional, tag = "2")]
    pub organization_id: Option<i64>,
    #[prost(string, tag = "3")]
    pub repository_url: String,
    #[prost(string, tag = "4")]
    pub branch: String,
    #[prost(string, tag = "5")]
    pub source_type: String,
    #[prost(string, optional, tag = "6")]
    pub detected_type: Option<String>,
    #[prost(string, repeated, tag = "7")]
    pub compatible_agents: Vec<String>,
    #[prost(string, tag = "8")]
    pub auth_type: String,
    #[prost(string, optional, tag = "9")]
    pub last_synced_at: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub last_commit_sha: Option<String>,
    #[prost(string, tag = "11")]
    pub sync_status: String,
    #[prost(string, optional, tag = "12")]
    pub sync_error: Option<String>,
    #[prost(int32, tag = "13")]
    pub skill_count: i32,
    #[prost(bool, tag = "14")]
    pub is_active: bool,
    #[prost(string, tag = "15")]
    pub created_at: String,
    #[prost(string, tag = "16")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SkillRegistryOverride {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(int64, tag = "3")]
    pub registry_id: i64,
    #[prost(bool, tag = "4")]
    pub is_disabled: bool,
    #[prost(string, tag = "5")]
    pub created_at: String,
    #[prost(string, tag = "6")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSkillRegistriesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSkillRegistriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<SkillRegistry>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateSkillRegistryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub repository_url: String,
    #[prost(string, optional, tag = "3")]
    pub branch: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub source_type: Option<String>,
    #[prost(string, repeated, tag = "5")]
    pub compatible_agents: Vec<String>,
    #[prost(string, optional, tag = "6")]
    pub auth_type: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub auth_credential: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SyncSkillRegistryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteSkillRegistryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteSkillRegistryResponse {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TogglePlatformRegistryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(bool, tag = "3")]
    pub disabled: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TogglePlatformRegistryResponse {
    #[prost(message, repeated, tag = "1")]
    pub overrides: Vec<SkillRegistryOverride>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSkillRegistryOverridesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSkillRegistryOverridesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<SkillRegistryOverride>,
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

    fn sample_registry() -> SkillRegistry {
        SkillRegistry {
            id: 7,
            organization_id: Some(42),
            repository_url: "https://github.com/agentsmesh/skills-bundle".into(),
            branch: "main".into(),
            source_type: "auto".into(),
            detected_type: Some("collection".into()),
            compatible_agents: vec!["claude-code".into(), "codex".into()],
            auth_type: "github_pat".into(),
            last_synced_at: Some("2026-05-12T13:16:10Z".into()),
            last_commit_sha: Some("abc123def456".into()),
            sync_status: "success".into(),
            sync_error: None,
            skill_count: 12,
            is_active: true,
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-09T00:00:00Z".into(),
        }
    }

    #[test]
    fn skill_registry_round_trip_preserves_every_field() {
        let original = sample_registry();
        let bytes = original.encode_to_vec();
        let decoded = SkillRegistry::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListSkillRegistriesResponse {
            items: vec![sample_registry()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListSkillRegistriesResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.total, 1);
        assert_eq!(decoded.limit, 20);
        assert_eq!(decoded.offset, 0);
    }

    #[test]
    fn create_request_round_trip_with_optionals_set() {
        let original = CreateSkillRegistryRequest {
            org_slug: "acme".into(),
            repository_url: "https://github.com/example/skills".into(),
            branch: Some("dev".into()),
            source_type: Some("collection".into()),
            compatible_agents: vec!["claude-code".into()],
            auth_type: Some("github_pat".into()),
            auth_credential: Some("ghp_secret".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateSkillRegistryRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn optional_offset_zero_distinguishable_from_absent() {
        // Conventions §5: `optional int32 offset = 2;` must distinguish
        // "absent" from "explicit 0". This is the trap PR/#368 lineage hit
        // on REST (`c.DefaultQuery("offset", "0")` lost the distinction).
        let with_zero = ListSkillRegistriesRequest {
            org_slug: "acme".into(),
            offset: Some(0),
            limit: None,
        };
        let absent = ListSkillRegistriesRequest {
            org_slug: "acme".into(),
            offset: None,
            limit: None,
        };
        let zero_bytes = with_zero.encode_to_vec();
        let absent_bytes = absent.encode_to_vec();
        assert_ne!(zero_bytes, absent_bytes,
            "explicit zero must encode different bytes from absent field");

        let r1 = ListSkillRegistriesRequest::decode(&*zero_bytes).unwrap();
        let r2 = ListSkillRegistriesRequest::decode(&*absent_bytes).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn delete_request_response_round_trip() {
        let req = DeleteSkillRegistryRequest { org_slug: "acme".into(), id: 99 };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, DeleteSkillRegistryRequest::decode(&*req_bytes).unwrap());

        let resp = DeleteSkillRegistryResponse {};
        let resp_bytes = resp.encode_to_vec();
        assert!(resp_bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(resp, DeleteSkillRegistryResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn toggle_response_carries_overrides() {
        let original = TogglePlatformRegistryResponse {
            overrides: vec![SkillRegistryOverride {
                id: 1,
                organization_id: 42,
                registry_id: 7,
                is_disabled: true,
                created_at: "2026-05-12T00:00:00Z".into(),
                updated_at: "2026-05-12T00:00:00Z".into(),
            }],
        };
        let bytes = original.encode_to_vec();
        let decoded = TogglePlatformRegistryResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
