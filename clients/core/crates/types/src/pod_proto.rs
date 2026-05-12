// Hand-maintained `prost::Message` mirrors of `proto/pod/v1/pod.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift (watch list §8). NO `Serialize` /
// `Deserialize` derives — binary wire only (conventions §2.5, §3).
//
// Pod has 31 prost-tagged fields plus 6 nested sub-messages. The 200-line
// CLAUDE.md soft cap is exceeded here because splitting the prost-tag
// declarations across files would defeat `validate_prost_tags` (it expects
// one Rust file per .proto file). Acceptable per CLAUDE.md note "210-line
// file is acceptable if splitting would break cohesion".

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodRunnerInfo {
    #[prost(int64, optional, tag = "1")]
    pub id: Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub node_id: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub status: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodAgentInfo {
    #[prost(string, optional, tag = "1")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub slug: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodRepositoryInfo {
    #[prost(int64, optional, tag = "1")]
    pub id: Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub slug: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub provider_type: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodTicketInfo {
    #[prost(int64, optional, tag = "1")]
    pub id: Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub slug: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub title: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodLoopInfo {
    #[prost(int64, optional, tag = "1")]
    pub id: Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub slug: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodCreatedByInfo {
    #[prost(int64, optional, tag = "1")]
    pub id: Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub username: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Pod {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub pod_key: String,
    #[prost(string, tag = "3")]
    pub status: String,
    #[prost(string, tag = "4")]
    pub agent_status: String,
    #[prost(string, optional, tag = "5")]
    pub alias: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub title: Option<String>,
    #[prost(string, tag = "7")]
    pub agent_slug: String,
    #[prost(int64, optional, tag = "8")]
    pub runner_id: Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub created_by_id: Option<i64>,
    #[prost(message, optional, tag = "10")]
    pub runner: Option<PodRunnerInfo>,
    #[prost(message, optional, tag = "11")]
    pub agent: Option<PodAgentInfo>,
    #[prost(message, optional, tag = "12")]
    pub repository: Option<PodRepositoryInfo>,
    #[prost(message, optional, tag = "13")]
    pub ticket: Option<PodTicketInfo>,
    #[prost(message, optional, tag = "14")]
    pub r#loop: Option<PodLoopInfo>,
    #[prost(message, optional, tag = "15")]
    pub created_by: Option<PodCreatedByInfo>,
    #[prost(string, optional, tag = "16")]
    pub prompt: Option<String>,
    #[prost(string, optional, tag = "17")]
    pub branch_name: Option<String>,
    #[prost(string, optional, tag = "18")]
    pub sandbox_path: Option<String>,
    #[prost(string, tag = "19")]
    pub interaction_mode: String,
    #[prost(bool, tag = "20")]
    pub perpetual: bool,
    #[prost(int32, tag = "21")]
    pub restart_count: i32,
    #[prost(string, optional, tag = "22")]
    pub last_restart_at: Option<String>,
    #[prost(string, optional, tag = "23")]
    pub started_at: Option<String>,
    #[prost(string, optional, tag = "24")]
    pub finished_at: Option<String>,
    #[prost(string, optional, tag = "25")]
    pub last_activity: Option<String>,
    #[prost(string, tag = "26")]
    pub created_at: String,
    #[prost(string, tag = "27")]
    pub updated_at: String,
    #[prost(string, optional, tag = "28")]
    pub error_code: Option<String>,
    #[prost(string, optional, tag = "29")]
    pub error_message: Option<String>,
    #[prost(string, optional, tag = "30")]
    pub source_pod_key: Option<String>,
    #[prost(string, optional, tag = "31")]
    pub session_id: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PodConnectionInfo {
    #[prost(string, tag = "1")]
    pub relay_url: String,
    #[prost(string, tag = "2")]
    pub token: String,
    #[prost(string, tag = "3")]
    pub pod_key: String,
    #[prost(string, tag = "4")]
    pub local_relay_url: String,
    #[prost(string, tag = "5")]
    pub local_token: String,
    #[prost(string, tag = "6")]
    pub local_relay_node_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListPodsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, optional, tag = "2")]
    pub status: Option<String>,
    #[prost(int64, optional, tag = "3")]
    pub created_by_id: Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub limit: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub offset: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListPodsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Pod>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListPodsByTicketRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub ticket_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListPodsByTicketResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Pod>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetPodRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetPodConnectionRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreatePodRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub agent_slug: String,
    #[prost(int64, optional, tag = "3")]
    pub runner_id: Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub ticket_slug: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub alias: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub agentfile_layer: Option<String>,
    #[prost(int64, optional, tag = "7")]
    pub repository_id: Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub credential_profile_id: Option<i64>,
    #[prost(int32, tag = "9")]
    pub cols: i32,
    #[prost(int32, tag = "10")]
    pub rows: i32,
    #[prost(string, optional, tag = "11")]
    pub source_pod_key: Option<String>,
    #[prost(bool, optional, tag = "12")]
    pub resume_agent_session: Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub perpetual: Option<bool>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreatePodResponse {
    #[prost(message, optional, tag = "1")]
    pub pod: Option<Pod>,
    #[prost(string, optional, tag = "2")]
    pub warning: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TerminatePodRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TerminatePodResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdatePodAliasRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
    #[prost(string, optional, tag = "3")]
    pub alias: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdatePodAliasResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdatePodPerpetualRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
    #[prost(bool, tag = "3")]
    pub perpetual: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdatePodPerpetualResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SendPodPromptRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
    #[prost(string, tag = "3")]
    pub prompt: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SendPodPromptResponse {
    #[prost(string, tag = "1")]
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_pod() -> Pod {
        Pod {
            id: 42,
            pod_key: "pod-abc".into(),
            status: "running".into(),
            agent_status: "idle".into(),
            alias: Some("dev-pod".into()),
            title: Some("My Pod".into()),
            agent_slug: "claude-code".into(),
            runner_id: Some(7),
            created_by_id: Some(10),
            runner: Some(PodRunnerInfo {
                id: Some(7),
                node_id: Some("runner-7".into()),
                status: Some("online".into()),
            }),
            agent: Some(PodAgentInfo {
                name: Some("Claude Code".into()),
                slug: Some("claude-code".into()),
            }),
            repository: Some(PodRepositoryInfo {
                id: Some(3),
                name: Some("my-repo".into()),
                slug: Some("my-repo".into()),
                provider_type: Some("github".into()),
            }),
            ticket: Some(PodTicketInfo {
                id: Some(5),
                slug: Some("TK-5".into()),
                title: Some("Fix bug".into()),
            }),
            r#loop: Some(PodLoopInfo {
                id: Some(1),
                name: Some("CI Loop".into()),
                slug: Some("ci-loop".into()),
            }),
            created_by: Some(PodCreatedByInfo {
                id: Some(10),
                username: Some("alice".into()),
                name: Some("Alice".into()),
            }),
            prompt: Some("hello".into()),
            branch_name: Some("feature/x".into()),
            sandbox_path: Some("/sandbox/x".into()),
            interaction_mode: "pty".into(),
            perpetual: true,
            restart_count: 2,
            last_restart_at: Some("2026-05-12T13:16:10Z".into()),
            started_at: Some("2026-05-12T13:16:00Z".into()),
            finished_at: None,
            last_activity: Some("2026-05-12T13:16:15Z".into()),
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
            error_code: None,
            error_message: None,
            source_pod_key: Some("pod-source".into()),
            session_id: Some("sess-xyz".into()),
        }
    }

    #[test]
    fn pod_round_trip_preserves_every_field() {
        let original = sample_pod();
        let bytes = original.encode_to_vec();
        let decoded = Pod::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake surfaces as field-value mismatch here");
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListPodsResponse {
            items: vec![sample_pod()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListPodsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.total, 1);
        assert_eq!(decoded.limit, 20);
        assert_eq!(decoded.offset, 0);
    }

    #[test]
    fn create_pod_response_keeps_warning_envelope() {
        // Regression for 986a38ca6 — `{pod, warning}` envelope MUST survive
        // round-trip. Dropping warning here would silently regress the
        // quota-near toast UX (PR #340 fix).
        let original = CreatePodResponse {
            pod: Some(sample_pod()),
            warning: Some("Pod quota near limit".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreatePodResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.pod.is_some());
        assert_eq!(decoded.warning.as_deref(), Some("Pod quota near limit"));
    }

    #[test]
    fn create_pod_response_without_warning() {
        let original = CreatePodResponse {
            pod: Some(sample_pod()),
            warning: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = CreatePodResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.warning.is_none());
    }

    #[test]
    fn optional_offset_zero_distinguishable_from_absent() {
        let with_zero = ListPodsRequest {
            org_slug: "acme".into(),
            status: None,
            created_by_id: None,
            limit: None,
            offset: Some(0),
        };
        let absent = ListPodsRequest {
            org_slug: "acme".into(),
            status: None,
            created_by_id: None,
            limit: None,
            offset: None,
        };
        let zero_bytes = with_zero.encode_to_vec();
        let absent_bytes = absent.encode_to_vec();
        assert_ne!(zero_bytes, absent_bytes,
            "explicit zero must encode different bytes from absent field");

        let r1 = ListPodsRequest::decode(&*zero_bytes).unwrap();
        let r2 = ListPodsRequest::decode(&*absent_bytes).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn pod_connection_info_round_trip() {
        let original = PodConnectionInfo {
            relay_url: "wss://relay.example.com".into(),
            token: "tok-123".into(),
            pod_key: "pod-1".into(),
            local_relay_url: String::new(),
            local_token: String::new(),
            local_relay_node_id: String::new(),
        };
        let bytes = original.encode_to_vec();
        let decoded = PodConnectionInfo::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn update_alias_request_handles_clear_vs_unset() {
        let clear = UpdatePodAliasRequest {
            org_slug: "acme".into(),
            pod_key: "pod-1".into(),
            alias: Some(String::new()),
        };
        let unset = UpdatePodAliasRequest {
            org_slug: "acme".into(),
            pod_key: "pod-1".into(),
            alias: None,
        };
        let clear_decoded = UpdatePodAliasRequest::decode(&*clear.encode_to_vec()).unwrap();
        let unset_decoded = UpdatePodAliasRequest::decode(&*unset.encode_to_vec()).unwrap();
        assert_eq!(clear_decoded.alias, Some(String::new()));
        assert_eq!(unset_decoded.alias, None);
    }

    #[test]
    fn create_request_resume_without_agent_slug() {
        // Resume from terminated pod: agent_slug can be empty, source_pod_key set.
        let original = CreatePodRequest {
            org_slug: "acme".into(),
            agent_slug: String::new(),
            runner_id: Some(1),
            ticket_slug: None,
            alias: None,
            agentfile_layer: None,
            repository_id: None,
            credential_profile_id: None,
            cols: 120,
            rows: 30,
            source_pod_key: Some("pod-source".into()),
            resume_agent_session: Some(true),
            perpetual: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = CreatePodRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.agent_slug.is_empty());
        assert_eq!(decoded.source_pod_key.as_deref(), Some("pod-source"));
    }

    #[test]
    fn list_pods_by_ticket_round_trip() {
        let original = ListPodsByTicketResponse {
            items: vec![sample_pod()],
            total: 1,
            limit: 0,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListPodsByTicketResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn terminate_response_round_trip() {
        let original = TerminatePodResponse { message: "Pod terminated".into() };
        let bytes = original.encode_to_vec();
        let decoded = TerminatePodResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
