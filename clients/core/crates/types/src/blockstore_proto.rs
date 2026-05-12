// Hand-maintained `prost::Message` mirrors of
// `proto/blockstore/v1/blockstore.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` runs at build time to catch
// drift (watch list §8). NO `Serialize`/`Deserialize` derives on these
// structs — binary wire only (conventions §2.5, §3).
//
// Block.data / .meta / BlockOp.{payload, forward, inverse, context} ride
// as opaque UTF-8 JSON `String`s on the wire (the per-blocktype schemas
// would otherwise pollute proto with 200+ subschemas). Callers serialise /
// parse with `serde_json::Value` at the boundary.

#[derive(Clone, PartialEq, prost::Message)]
pub struct Block {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, tag = "3")]
    pub r#type: String,
    #[prost(string, tag = "4")]
    pub data_json: String,
    #[prost(string, optional, tag = "5")]
    pub text: Option<String>,
    #[prost(string, tag = "6")]
    pub meta_json: String,
    #[prost(int64, tag = "7")]
    pub created_by: i64,
    #[prost(string, tag = "8")]
    pub created_at: String,
    #[prost(string, tag = "9")]
    pub updated_at: String,
    #[prost(string, optional, tag = "10")]
    pub deleted_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct BlockRef {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, tag = "3")]
    pub from_id: String,
    #[prost(string, tag = "4")]
    pub to_id: String,
    #[prost(string, tag = "5")]
    pub rel: String,
    #[prost(string, optional, tag = "6")]
    pub order_key: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub anchor: Option<String>,
    #[prost(string, tag = "8")]
    pub meta_json: String,
    #[prost(int64, tag = "9")]
    pub created_by: i64,
    #[prost(string, tag = "10")]
    pub created_at: String,
    #[prost(string, tag = "11")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct BlockOp {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, optional, tag = "3")]
    pub idempotency_key: Option<String>,
    #[prost(string, tag = "4")]
    pub actor_type: String,
    #[prost(int64, tag = "5")]
    pub actor_id: i64,
    #[prost(string, tag = "6")]
    pub op: String,
    #[prost(string, optional, tag = "7")]
    pub target_block: Option<String>,
    #[prost(int64, optional, tag = "8")]
    pub target_ref: Option<i64>,
    #[prost(string, tag = "9")]
    pub payload_json: String,
    #[prost(string, tag = "10")]
    pub forward_json: String,
    #[prost(string, tag = "11")]
    pub inverse_json: String,
    #[prost(string, tag = "12")]
    pub context_json: String,
    #[prost(int64, optional, tag = "13")]
    pub parent_op_id: Option<i64>,
    #[prost(string, tag = "14")]
    pub applied_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Workspace {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(string, tag = "3")]
    pub slug: String,
    #[prost(string, tag = "4")]
    pub name: String,
    #[prost(string, optional, tag = "5")]
    pub root_block_id: Option<String>,
    #[prost(string, tag = "6")]
    pub created_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct OpEnvelope {
    #[prost(string, tag = "1")]
    pub op: String,
    #[prost(string, tag = "2")]
    pub payload_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ApplyOpsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(message, repeated, tag = "3")]
    pub ops: Vec<OpEnvelope>,
    #[prost(string, optional, tag = "4")]
    pub idempotency_key: Option<String>,
    #[prost(int64, optional, tag = "5")]
    pub parent_op_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ApplyOpsResponse {
    #[prost(int64, repeated, tag = "1")]
    pub op_ids: Vec<i64>,
    #[prost(bool, tag = "2")]
    pub was_replay: bool,
    #[prost(int64, optional, tag = "3")]
    pub parent_op_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListWorkspacesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListWorkspacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Workspace>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct EnsureDefaultWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub slug: String,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteWorkspaceResponse {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetBlockRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListChildrenRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(string, optional, tag = "3")]
    pub rel: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ChildrenResult {
    #[prost(message, repeated, tag = "1")]
    pub blocks: Vec<Block>,
    #[prost(message, repeated, tag = "2")]
    pub refs: Vec<BlockRef>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListBacklinksRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListBacklinksResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<BlockRef>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetSubtreeRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, tag = "3")]
    pub root_id: String,
    #[prost(int32, optional, tag = "4")]
    pub max_depth: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct StreamOpsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(int64, optional, tag = "3")]
    pub after: Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct StreamOpsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<BlockOp>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ExportWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ExportWorkspaceResponse {
    #[prost(string, tag = "1")]
    pub export_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListTypeDefsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListTypeDefsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Block>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetBlockAtRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(int64, optional, tag = "3")]
    pub op_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SearchHit {
    #[prost(string, tag = "1")]
    pub block_id: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(string, tag = "3")]
    pub snippet: String,
    #[prost(float, tag = "4")]
    pub score: f32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SemanticSearchRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, tag = "3")]
    pub query: String,
    #[prost(int32, optional, tag = "4")]
    pub top_k: Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub min_score: Option<f32>,
    #[prost(string, optional, tag = "6")]
    pub r#type: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SemanticSearchResponse {
    #[prost(message, repeated, tag = "1")]
    pub hits: Vec<SearchHit>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MemoryRetrieveRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub workspace_id: String,
    #[prost(string, tag = "3")]
    pub query: String,
    #[prost(int32, optional, tag = "4")]
    pub k: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MemoryRetrieveResponse {
    #[prost(message, repeated, tag = "1")]
    pub memories: Vec<SearchHit>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_block() -> Block {
        Block {
            id: "11111111-2222-3333-4444-555555555555".into(),
            workspace_id: "00000000-0000-0000-0000-000000000001".into(),
            r#type: "page".into(),
            data_json: r#"{"title":"Home"}"#.into(),
            text: Some("Home".into()),
            meta_json: "{}".into(),
            created_by: 42,
            created_at: "2026-05-12T12:00:00Z".into(),
            updated_at: "2026-05-12T12:30:00Z".into(),
            deleted_at: None,
        }
    }

    fn sample_ref() -> BlockRef {
        BlockRef {
            id: 7,
            workspace_id: "00000000-0000-0000-0000-000000000001".into(),
            from_id: "11111111-2222-3333-4444-555555555555".into(),
            to_id: "11111111-2222-3333-4444-555555555556".into(),
            rel: "nest".into(),
            order_key: Some("a0".into()),
            anchor: None,
            meta_json: "{}".into(),
            created_by: 42,
            created_at: "2026-05-12T12:00:00Z".into(),
            updated_at: "2026-05-12T12:00:00Z".into(),
        }
    }

    fn sample_op() -> BlockOp {
        BlockOp {
            id: 99,
            workspace_id: "00000000-0000-0000-0000-000000000001".into(),
            idempotency_key: Some("k1".into()),
            actor_type: "user".into(),
            actor_id: 42,
            op: "createBlock".into(),
            target_block: Some("11111111-2222-3333-4444-555555555555".into()),
            target_ref: None,
            payload_json: r#"{"id":"11111111-2222-3333-4444-555555555555"}"#.into(),
            forward_json: r#"{"id":"11111111-2222-3333-4444-555555555555","type":"page"}"#.into(),
            inverse_json: r#"{"id":"11111111-2222-3333-4444-555555555555"}"#.into(),
            context_json: r#"{"trace_id":"abc"}"#.into(),
            parent_op_id: None,
            applied_at: "2026-05-12T12:00:00Z".into(),
        }
    }

    #[test]
    fn block_round_trip_preserves_every_field() {
        let original = sample_block();
        let bytes = original.encode_to_vec();
        let decoded = Block::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn block_ref_round_trip() {
        let original = sample_ref();
        let bytes = original.encode_to_vec();
        let decoded = BlockRef::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn block_op_round_trip_preserves_context_json() {
        let original = sample_op();
        let bytes = original.encode_to_vec();
        let decoded = BlockOp::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        // context_json is the audit slot (migration 000118) absent from the
        // legacy serde DTO. Pin it round-trips so audit consumers can rely on
        // its presence at tag 12.
        assert_eq!(decoded.context_json, r#"{"trace_id":"abc"}"#);
    }

    #[test]
    fn apply_ops_request_round_trip() {
        let original = ApplyOpsRequest {
            org_slug: "acme".into(),
            workspace_id: "00000000-0000-0000-0000-000000000001".into(),
            ops: vec![
                OpEnvelope { op: "createBlock".into(), payload_json: r#"{"id":"a"}"#.into() },
                OpEnvelope { op: "addRef".into(), payload_json: r#"{"from":"a","to":"b","rel":"nest"}"#.into() },
            ],
            idempotency_key: Some("k1".into()),
            parent_op_id: Some(5),
        };
        let bytes = original.encode_to_vec();
        let decoded = ApplyOpsRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.ops.len(), 2);
        assert_eq!(decoded.ops[0].op, "createBlock");
    }

    #[test]
    fn apply_ops_response_envelope_round_trip() {
        let original = ApplyOpsResponse {
            op_ids: vec![1, 2, 3],
            was_replay: true,
            parent_op_id: Some(7),
        };
        let bytes = original.encode_to_vec();
        let decoded = ApplyOpsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn list_workspaces_response_round_trip() {
        let original = ListWorkspacesResponse {
            items: vec![Workspace {
                id: "00000000-0000-0000-0000-000000000001".into(),
                organization_id: 7,
                slug: "default".into(),
                name: "Default Workspace".into(),
                root_block_id: Some("11111111-2222-3333-4444-555555555555".into()),
                created_at: "2026-05-12T00:00:00Z".into(),
            }],
            total: 1,
            limit: 0,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListWorkspacesResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn children_result_round_trip_preserves_blocks_and_refs() {
        let original = ChildrenResult {
            blocks: vec![sample_block()],
            refs: vec![sample_ref()],
        };
        let bytes = original.encode_to_vec();
        let decoded = ChildrenResult::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        // Confirms tag 1 (blocks) and tag 2 (refs) don't swap — a transposed
        // tag in ChildrenResult would surface here (blocks↔refs both lists).
        assert_eq!(decoded.blocks.len(), 1);
        assert_eq!(decoded.refs.len(), 1);
    }

    #[test]
    fn stream_ops_response_round_trip() {
        let original = StreamOpsResponse {
            items: vec![sample_op()],
            total: 1,
            limit: 200,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = StreamOpsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn search_hit_score_round_trip() {
        let original = SearchHit {
            block_id: "11111111-2222-3333-4444-555555555555".into(),
            r#type: "page".into(),
            snippet: "matched text".into(),
            score: 0.875,
        };
        let bytes = original.encode_to_vec();
        let decoded = SearchHit::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!((decoded.score - 0.875).abs() < f32::EPSILON);
    }

    #[test]
    fn semantic_search_request_optional_top_k_distinguishable() {
        // Conventions §5: explicit zero must differ from absent so the server
        // can default appropriately.
        let with_zero = SemanticSearchRequest {
            org_slug: "acme".into(),
            workspace_id: "ws".into(),
            query: "q".into(),
            top_k: Some(0),
            min_score: None,
            r#type: None,
        };
        let absent = SemanticSearchRequest {
            org_slug: "acme".into(),
            workspace_id: "ws".into(),
            query: "q".into(),
            top_k: None,
            min_score: None,
            r#type: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
    }

    #[test]
    fn delete_workspace_response_round_trip() {
        let resp = DeleteWorkspaceResponse {};
        let bytes = resp.encode_to_vec();
        assert!(bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(resp, DeleteWorkspaceResponse::decode(&*bytes).unwrap());
    }

    #[test]
    fn export_workspace_response_round_trip() {
        let original = ExportWorkspaceResponse {
            export_json: r#"{"blocks":[],"refs":[],"ops":[]}"#.into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = ExportWorkspaceResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
