// Hand-maintained `prost::Message` mirrors of
// `proto/runner_api/v1/runner.proto`. Tag numbers match the .proto
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
pub struct Runner {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub node_id: String,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(string, tag = "4")]
    pub status: String,
    #[prost(string, optional, tag = "5")]
    pub last_heartbeat: Option<String>,
    #[prost(int32, tag = "6")]
    pub current_pods: i32,
    #[prost(int32, tag = "7")]
    pub max_concurrent_pods: i32,
    #[prost(string, optional, tag = "8")]
    pub runner_version: Option<String>,
    #[prost(bool, tag = "9")]
    pub is_enabled: bool,
    #[prost(string, tag = "10")]
    pub visibility: String,
    #[prost(int64, optional, tag = "11")]
    pub registered_by_user_id: Option<i64>,
    #[prost(string, tag = "12")]
    pub host_info_json: String,
    #[prost(string, repeated, tag = "13")]
    pub available_agents: Vec<String>,
    #[prost(string, repeated, tag = "14")]
    pub tags: Vec<String>,
    #[prost(string, tag = "15")]
    pub created_at: String,
    #[prost(string, tag = "16")]
    pub updated_at: String,
    #[prost(int64, tag = "17")]
    pub organization_id: i64,
    #[prost(message, repeated, tag = "18")]
    pub agent_versions: Vec<AgentVersion>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AgentVersion {
    #[prost(string, tag = "1")]
    pub slug: String,
    #[prost(string, tag = "2")]
    pub version: String,
    #[prost(string, tag = "3")]
    pub path: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RelayConnectionInfo {
    #[prost(string, tag = "1")]
    pub pod_key: String,
    #[prost(string, tag = "2")]
    pub relay_url: String,
    #[prost(string, tag = "3")]
    pub session_id: String,
    #[prost(bool, tag = "4")]
    pub connected: bool,
    #[prost(int64, tag = "5")]
    pub connected_at: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RunnerToken {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub token: Option<String>,
    #[prost(int32, optional, tag = "4")]
    pub max_uses: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub used_count: Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub expires_at: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub created_at: Option<String>,
}

// ============== List / Get ==============

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnersRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, optional, tag = "2")]
    pub status: Option<String>,
    #[prost(int32, optional, tag = "3")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Runner>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
    #[prost(string, optional, tag = "5")]
    pub latest_runner_version: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAvailableRunnersRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAvailableRunnersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Runner>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRunnerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRunnerResponse {
    #[prost(message, optional, tag = "1")]
    pub runner: Option<Runner>,
    #[prost(message, repeated, tag = "2")]
    pub relay_connections: Vec<RelayConnectionInfo>,
    #[prost(string, optional, tag = "3")]
    pub latest_runner_version: Option<String>,
}

// ============== Update / Delete ==============

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateRunnerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(int32, optional, tag = "4")]
    pub max_concurrent_pods: Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_enabled: Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub visibility: Option<String>,
    #[prost(message, optional, tag = "7")]
    pub tags: Option<TagsUpdate>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TagsUpdate {
    #[prost(string, repeated, tag = "1")]
    pub values: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRunnerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRunnerResponse {}

// ============== Upgrade / Logs / Sandboxes ==============

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpgradeRunnerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, tag = "3")]
    pub target_version: String,
    #[prost(bool, tag = "4")]
    pub force: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpgradeRunnerResponse {
    #[prost(string, tag = "1")]
    pub request_id: String,
    #[prost(string, tag = "2")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RequestLogUploadRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RequestLogUploadResponse {
    #[prost(string, tag = "1")]
    pub request_id: String,
    #[prost(string, tag = "2")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnerLogsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(int32, optional, tag = "3")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnerLogsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<RunnerLog>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RunnerLog {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub runner_id: i64,
    #[prost(string, tag = "3")]
    pub request_id: String,
    #[prost(string, tag = "4")]
    pub status: String,
    #[prost(string, optional, tag = "5")]
    pub storage_key: Option<String>,
    #[prost(int64, tag = "6")]
    pub size_bytes: i64,
    #[prost(string, optional, tag = "7")]
    pub error_message: Option<String>,
    #[prost(int64, tag = "8")]
    pub requested_by_id: i64,
    #[prost(string, optional, tag = "9")]
    pub download_url: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub created_at: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub completed_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct QuerySandboxesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, repeated, tag = "3")]
    pub pod_keys: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct QuerySandboxesResponse {
    #[prost(message, repeated, tag = "1")]
    pub sandboxes: Vec<SandboxStatus>,
    #[prost(string, tag = "2")]
    pub error: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SandboxStatus {
    #[prost(string, tag = "1")]
    pub pod_key: String,
    #[prost(bool, tag = "2")]
    pub exists: bool,
    #[prost(bool, tag = "3")]
    pub can_resume: bool,
    #[prost(string, optional, tag = "4")]
    pub sandbox_path: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub repository_url: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub branch_name: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub current_commit: Option<String>,
    #[prost(int64, optional, tag = "8")]
    pub size_bytes: Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub last_modified: Option<i64>,
    #[prost(bool, optional, tag = "10")]
    pub has_uncommitted_changes: Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub error: Option<String>,
}

// ============== Tokens ==============

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateRunnerTokenRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, repeated, tag = "3")]
    pub labels: Vec<String>,
    #[prost(int32, optional, tag = "4")]
    pub max_uses: Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub expires_in_days: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnerTokensRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunnerTokensResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<RunnerToken>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRunnerTokenRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRunnerTokenResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_runner() -> Runner {
        Runner {
            id: 7,
            node_id: "node-abc".into(),
            description: "dev runner".into(),
            status: "online".into(),
            last_heartbeat: Some("2026-05-12T13:16:10Z".into()),
            current_pods: 2,
            max_concurrent_pods: 5,
            runner_version: Some("0.29.0".into()),
            is_enabled: true,
            visibility: "organization".into(),
            registered_by_user_id: Some(42),
            host_info_json: r#"{"os":"linux","arch":"arm64"}"#.into(),
            available_agents: vec!["claude-code".into(), "codex".into()],
            tags: vec!["prod".into(), "edge".into()],
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-10T00:00:00Z".into(),
            organization_id: 99,
            agent_versions: vec![AgentVersion {
                slug: "claude-code".into(),
                version: "1.2.3".into(),
                path: "/usr/local/bin/claude".into(),
            }],
        }
    }

    #[test]
    fn runner_round_trip_preserves_every_field() {
        let original = sample_runner();
        let bytes = original.encode_to_vec();
        let decoded = Runner::decode(&*bytes).unwrap();
        assert_eq!(
            original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here"
        );
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListRunnersResponse {
            items: vec![sample_runner()],
            total: 1,
            limit: 20,
            offset: 0,
            latest_runner_version: Some("0.30.0".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = ListRunnersResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.total, 1);
        assert_eq!(decoded.limit, 20);
        assert_eq!(decoded.offset, 0);
        assert_eq!(decoded.latest_runner_version, Some("0.30.0".into()));
    }

    #[test]
    fn get_runner_response_with_relay_connections() {
        let original = GetRunnerResponse {
            runner: Some(sample_runner()),
            relay_connections: vec![RelayConnectionInfo {
                pod_key: "pod-1".into(),
                relay_url: "ws://relay.example.com".into(),
                session_id: "sess-1".into(),
                connected: true,
                connected_at: 1715515170123,
            }],
            latest_runner_version: Some("0.29.0".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = GetRunnerResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn update_request_round_trip_with_tags_explicit_empty() {
        // TagsUpdate semantics: present with empty values = clear tags.
        // absent (None on outer) = leave tags unchanged.
        let with_clear = UpdateRunnerRequest {
            org_slug: "acme".into(),
            id: 7,
            description: Some("new desc".into()),
            max_concurrent_pods: Some(10),
            is_enabled: Some(false),
            visibility: Some("private".into()),
            tags: Some(TagsUpdate { values: vec![] }),
        };
        let bytes = with_clear.encode_to_vec();
        let decoded = UpdateRunnerRequest::decode(&*bytes).unwrap();
        assert_eq!(with_clear, decoded);
        assert!(decoded.tags.is_some());
        assert!(decoded.tags.unwrap().values.is_empty());

        let no_change = UpdateRunnerRequest {
            org_slug: "acme".into(),
            id: 7,
            description: None,
            max_concurrent_pods: None,
            is_enabled: None,
            visibility: None,
            tags: None,
        };
        let bytes2 = no_change.encode_to_vec();
        let decoded2 = UpdateRunnerRequest::decode(&*bytes2).unwrap();
        assert!(decoded2.tags.is_none());
        // Must encode different bytes — distinguishing "no change" from "clear".
        assert_ne!(bytes, bytes2);
    }

    #[test]
    fn upgrade_response_round_trip() {
        let original = UpgradeRunnerResponse {
            request_id: "uuid-xyz".into(),
            message: "Upgrade command sent".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = UpgradeRunnerResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn sandbox_status_round_trip_with_all_optionals() {
        let original = SandboxStatus {
            pod_key: "pod-1".into(),
            exists: true,
            can_resume: true,
            sandbox_path: Some("/tmp/sb-1".into()),
            repository_url: Some("https://github.com/x/y".into()),
            branch_name: Some("main".into()),
            current_commit: Some("abc123".into()),
            size_bytes: Some(1024),
            last_modified: Some(1715515170),
            has_uncommitted_changes: Some(false),
            error: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = SandboxStatus::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn optional_offset_zero_distinguishable_from_absent() {
        let with_zero = ListRunnersRequest {
            org_slug: "acme".into(),
            status: None,
            offset: Some(0),
            limit: None,
        };
        let absent = ListRunnersRequest {
            org_slug: "acme".into(),
            status: None,
            offset: None,
            limit: None,
        };
        let zero_bytes = with_zero.encode_to_vec();
        let absent_bytes = absent.encode_to_vec();
        assert_ne!(
            zero_bytes, absent_bytes,
            "explicit zero must encode different bytes from absent field"
        );
        let r1 = ListRunnersRequest::decode(&*zero_bytes).unwrap();
        let r2 = ListRunnersRequest::decode(&*absent_bytes).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn create_token_request_round_trip() {
        let original = CreateRunnerTokenRequest {
            org_slug: "acme".into(),
            name: Some("dev-token".into()),
            labels: vec!["env=dev".into()],
            max_uses: Some(5),
            expires_in_days: Some(7),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateRunnerTokenRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn delete_request_response_round_trip() {
        let req = DeleteRunnerRequest {
            org_slug: "acme".into(),
            id: 99,
        };
        let req_bytes = req.encode_to_vec();
        assert_eq!(
            req,
            DeleteRunnerRequest::decode(&*req_bytes).unwrap()
        );
        let resp = DeleteRunnerResponse {};
        let resp_bytes = resp.encode_to_vec();
        assert!(resp_bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(
            resp,
            DeleteRunnerResponse::decode(&*resp_bytes).unwrap()
        );
    }
}
