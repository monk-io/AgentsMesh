// Hand-maintained `prost::Message` mirrors of
// `proto/repository/v1/repository.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` runs at build time to catch
// drift (watch list §8). NO `Serialize`/`Deserialize` derives on these
// structs — binary wire only (conventions §2.5, §3).
//
// PR #329 / #342 / #343 lineage: the legacy Rust DTO dropped
// `imported_by_user_id`, `webhook_config`, `preparation_script`,
// `preparation_timeout`, and `is_active`. The proto SSOT carries the full
// 19-field backend surface so future drift becomes compile-time.

#[derive(Clone, PartialEq, prost::Message)]
pub struct Repository {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(string, tag = "3")]
    pub provider_type: String,
    #[prost(string, tag = "4")]
    pub provider_base_url: String,
    #[prost(string, tag = "5")]
    pub http_clone_url: String,
    #[prost(string, tag = "6")]
    pub ssh_clone_url: String,
    #[prost(string, tag = "7")]
    pub external_id: String,
    #[prost(string, tag = "8")]
    pub name: String,
    #[prost(string, tag = "9")]
    pub slug: String,
    #[prost(string, tag = "10")]
    pub default_branch: String,
    #[prost(string, optional, tag = "11")]
    pub ticket_prefix: Option<String>,
    #[prost(string, tag = "12")]
    pub visibility: String,
    #[prost(int64, optional, tag = "13")]
    pub imported_by_user_id: Option<i64>,
    #[prost(string, optional, tag = "14")]
    pub preparation_script: Option<String>,
    #[prost(int32, optional, tag = "15")]
    pub preparation_timeout: Option<i32>,
    #[prost(bool, tag = "16")]
    pub is_active: bool,
    #[prost(message, optional, tag = "17")]
    pub webhook_config: Option<RepositoryWebhookConfig>,
    #[prost(string, tag = "18")]
    pub created_at: String,
    #[prost(string, tag = "19")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RepositoryWebhookConfig {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub url: String,
    #[prost(string, repeated, tag = "3")]
    pub events: Vec<String>,
    #[prost(bool, tag = "4")]
    pub is_active: bool,
    #[prost(bool, tag = "5")]
    pub needs_manual_setup: bool,
    #[prost(string, optional, tag = "6")]
    pub last_error: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub created_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Branch {
    #[prost(string, tag = "1")]
    pub name: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MergeRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int32, tag = "2")]
    pub mr_iid: i32,
    #[prost(string, tag = "3")]
    pub title: String,
    #[prost(string, tag = "4")]
    pub state: String,
    #[prost(string, tag = "5")]
    pub mr_url: String,
    #[prost(string, tag = "6")]
    pub source_branch: String,
    #[prost(string, tag = "7")]
    pub target_branch: String,
    #[prost(string, optional, tag = "8")]
    pub pipeline_status: Option<String>,
    #[prost(int64, optional, tag = "9")]
    pub pipeline_id: Option<i64>,
    #[prost(string, optional, tag = "10")]
    pub pipeline_url: Option<String>,
    #[prost(int64, optional, tag = "11")]
    pub ticket_id: Option<i64>,
    #[prost(int64, optional, tag = "12")]
    pub pod_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct WebhookStatus {
    #[prost(bool, tag = "1")]
    pub registered: bool,
    #[prost(string, optional, tag = "2")]
    pub webhook_id: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub webhook_url: Option<String>,
    #[prost(string, repeated, tag = "4")]
    pub events: Vec<String>,
    #[prost(bool, tag = "5")]
    pub is_active: bool,
    #[prost(bool, tag = "6")]
    pub needs_manual_setup: bool,
    #[prost(string, optional, tag = "7")]
    pub last_error: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub registered_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct WebhookSecret {
    #[prost(string, tag = "1")]
    pub webhook_url: String,
    #[prost(string, tag = "2")]
    pub webhook_secret: String,
    #[prost(string, repeated, tag = "3")]
    pub events: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct WebhookResult {
    #[prost(int64, tag = "1")]
    pub repo_id: i64,
    #[prost(bool, tag = "2")]
    pub registered: bool,
    #[prost(string, optional, tag = "3")]
    pub webhook_id: Option<String>,
    #[prost(bool, tag = "4")]
    pub needs_manual_setup: bool,
    #[prost(string, optional, tag = "5")]
    pub manual_webhook_url: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub manual_webhook_secret: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub error_message: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoriesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Repository>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRepositoryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateRepositoryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub provider_type: String,
    #[prost(string, tag = "3")]
    pub provider_base_url: String,
    #[prost(string, optional, tag = "4")]
    pub http_clone_url: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub ssh_clone_url: Option<String>,
    #[prost(string, tag = "6")]
    pub external_id: String,
    #[prost(string, tag = "7")]
    pub name: String,
    #[prost(string, tag = "8")]
    pub slug: String,
    #[prost(string, optional, tag = "9")]
    pub default_branch: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub ticket_prefix: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub visibility: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateRepositoryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub default_branch: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub ticket_prefix: Option<String>,
    #[prost(bool, optional, tag = "6")]
    pub is_active: Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub http_clone_url: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub ssh_clone_url: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryResponse {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryBranchesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, tag = "3")]
    pub access_token: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SyncRepositoryBranchesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, tag = "3")]
    pub access_token: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryBranchesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Branch>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryMergeRequestsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, optional, tag = "3")]
    pub branch: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub state: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRepositoryMergeRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<MergeRequest>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RegisterRepositoryWebhookRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RegisterRepositoryWebhookResponse {
    #[prost(message, optional, tag = "1")]
    pub result: Option<WebhookResult>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryWebhookRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteRepositoryWebhookResponse {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRepositoryWebhookStatusRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRepositoryWebhookSecretRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MarkRepositoryWebhookConfiguredRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MarkRepositoryWebhookConfiguredResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_webhook_config() -> RepositoryWebhookConfig {
        RepositoryWebhookConfig {
            id: "wh-abc123".into(),
            url: "https://hook.example.com/abc".into(),
            events: vec!["push".into(), "pull_request".into()],
            is_active: true,
            needs_manual_setup: false,
            last_error: None,
            created_at: Some("2026-05-08T00:00:00Z".into()),
        }
    }

    fn sample_repository() -> Repository {
        Repository {
            id: 42,
            organization_id: 7,
            provider_type: "github".into(),
            provider_base_url: "https://github.com".into(),
            http_clone_url: "https://github.com/acme/demo.git".into(),
            ssh_clone_url: "git@github.com:acme/demo.git".into(),
            external_id: "1234567".into(),
            name: "demo".into(),
            slug: "acme/demo".into(),
            default_branch: "main".into(),
            ticket_prefix: Some("ACME-".into()),
            visibility: "organization".into(),
            imported_by_user_id: Some(99),
            preparation_script: Some("npm install".into()),
            preparation_timeout: Some(600),
            is_active: true,
            webhook_config: Some(sample_webhook_config()),
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-09T00:00:00Z".into(),
        }
    }

    // PR #329 / #342 / #343 lineage pin: every backend field has to survive
    // the wire. The legacy Rust DTO dropped 5 fields (imported_by_user_id,
    // webhook_config, preparation_script, preparation_timeout, is_active)
    // — this test makes that bug class compile-time-impossible going forward.
    #[test]
    fn repository_preserves_every_backend_field() {
        let original = sample_repository();
        let bytes = original.encode_to_vec();
        let decoded = Repository::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.imported_by_user_id, Some(99));
        assert!(decoded.webhook_config.is_some(), "webhook_config tag 17 must round-trip");
        assert_eq!(decoded.preparation_script.as_deref(), Some("npm install"));
        assert_eq!(decoded.preparation_timeout, Some(600));
        assert!(decoded.is_active);
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListRepositoriesResponse {
            items: vec![sample_repository()],
            total: 1,
            limit: 50,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListRepositoriesResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
    }

    #[test]
    fn create_request_round_trip_with_optionals_set() {
        let original = CreateRepositoryRequest {
            org_slug: "acme".into(),
            provider_type: "github".into(),
            provider_base_url: "https://github.com".into(),
            http_clone_url: Some("https://github.com/acme/x.git".into()),
            ssh_clone_url: None,
            external_id: "1".into(),
            name: "x".into(),
            slug: "acme/x".into(),
            default_branch: Some("main".into()),
            ticket_prefix: Some("X-".into()),
            visibility: Some("private".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateRepositoryRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn update_request_optionals_distinguishable() {
        // `is_active` as `optional bool` distinguishes "unset" from "false":
        // a PATCH that omits is_active must NOT flip an active repo to
        // inactive. Plain proto3 bool would conflate the two.
        let omit = UpdateRepositoryRequest {
            org_slug: "acme".into(),
            id: 1,
            name: Some("new-name".into()),
            default_branch: None,
            ticket_prefix: None,
            is_active: None,
            http_clone_url: None,
            ssh_clone_url: None,
        };
        let set_false = UpdateRepositoryRequest {
            org_slug: "acme".into(),
            id: 1,
            name: Some("new-name".into()),
            default_branch: None,
            ticket_prefix: None,
            is_active: Some(false),
            http_clone_url: None,
            ssh_clone_url: None,
        };
        assert_ne!(omit.encode_to_vec(), set_false.encode_to_vec(),
            "omitted is_active vs explicit false must encode differently");
    }

    #[test]
    fn optional_offset_zero_distinguishable_from_absent() {
        // Conventions §5: `optional int32 offset` must distinguish absent
        // from explicit 0. `c.DefaultQuery("offset", "0")` on REST lost this
        // distinction — binary wire preserves it.
        let with_zero = ListRepositoriesRequest {
            org_slug: "acme".into(),
            offset: Some(0),
            limit: None,
        };
        let absent = ListRepositoriesRequest {
            org_slug: "acme".into(),
            offset: None,
            limit: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
    }

    #[test]
    fn webhook_status_round_trip() {
        let original = WebhookStatus {
            registered: true,
            webhook_id: Some("wh-99".into()),
            webhook_url: Some("https://hook.example.com/99".into()),
            events: vec!["push".into()],
            is_active: true,
            needs_manual_setup: false,
            last_error: None,
            registered_at: Some("2026-05-08T00:00:00Z".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = WebhookStatus::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    // WebhookSecret is the one-time-secret response: webhook_secret tag 2
    // MUST survive the wasm hop. Same failure-mode class as PR #345's
    // raw_key bug.
    #[test]
    fn webhook_secret_preserves_secret_field() {
        let original = WebhookSecret {
            webhook_url: "https://hook.example.com/secret".into(),
            webhook_secret: "shhh-don-t-tell".into(),
            events: vec!["push".into(), "pull_request".into()],
        };
        let bytes = original.encode_to_vec();
        let decoded = WebhookSecret::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.webhook_secret, "shhh-don-t-tell");
    }

    #[test]
    fn webhook_result_round_trip_manual_setup() {
        let original = WebhookResult {
            repo_id: 7,
            registered: false,
            webhook_id: None,
            needs_manual_setup: true,
            manual_webhook_url: Some("https://hook.example.com/manual".into()),
            manual_webhook_secret: Some("manual-shhh".into()),
            error_message: Some("provider not supported".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = WebhookResult::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn merge_request_round_trip_preserves_pipeline() {
        let original = MergeRequest {
            id: 7,
            mr_iid: 42,
            title: "feat: x".into(),
            state: "opened".into(),
            mr_url: "https://gitlab.example.com/acme/x/-/merge_requests/42".into(),
            source_branch: "feat/x".into(),
            target_branch: "main".into(),
            pipeline_status: Some("running".into()),
            pipeline_id: Some(99),
            pipeline_url: Some("https://gitlab.example.com/acme/x/-/pipelines/99".into()),
            ticket_id: Some(123),
            pod_id: Some(456),
        };
        let bytes = original.encode_to_vec();
        let decoded = MergeRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn delete_request_response_round_trip() {
        let req = DeleteRepositoryRequest { org_slug: "acme".into(), id: 99 };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, DeleteRepositoryRequest::decode(&*req_bytes).unwrap());

        let resp = DeleteRepositoryResponse {};
        let resp_bytes = resp.encode_to_vec();
        assert!(resp_bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(resp, DeleteRepositoryResponse::decode(&*resp_bytes).unwrap());
    }
}
