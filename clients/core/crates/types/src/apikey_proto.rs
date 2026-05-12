// Hand-maintained `prost::Message` mirrors of
// `proto/apikey/v1/api_key.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` runs at build time to catch
// drift (watch list §8). NO `Serialize`/`Deserialize` derives on these
// structs — binary wire only (conventions §2.5, §3).
//
// PR #345 lineage: CreateApiKeyResponse is multi-field (api_key + raw_key)
// per conventions §9 exception — that's the bug class the proto explicitly
// designs against (raw_key disappearing on the wasm hop).

#[derive(Clone, PartialEq, prost::Message)]
pub struct ApiKey {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(string, optional, tag = "4")]
    pub description: Option<String>,
    #[prost(string, tag = "5")]
    pub key_prefix: String,
    #[prost(string, repeated, tag = "6")]
    pub scopes: Vec<String>,
    #[prost(bool, tag = "7")]
    pub is_enabled: bool,
    #[prost(string, optional, tag = "8")]
    pub expires_at: Option<String>,
    #[prost(string, optional, tag = "9")]
    pub last_used_at: Option<String>,
    #[prost(int64, tag = "10")]
    pub created_by: i64,
    #[prost(string, tag = "11")]
    pub created_at: String,
    #[prost(string, tag = "12")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListApiKeysRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListApiKeysResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<ApiKey>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetApiKeyRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateApiKeyRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(string, repeated, tag = "4")]
    pub scopes: Vec<String>,
    #[prost(int64, optional, tag = "5")]
    pub expires_in: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateApiKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub api_key: Option<ApiKey>,
    #[prost(string, tag = "2")]
    pub raw_key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateApiKeyRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub description: Option<String>,
    #[prost(string, repeated, tag = "5")]
    pub scopes: Vec<String>,
    #[prost(bool, optional, tag = "6")]
    pub is_enabled: Option<bool>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RevokeApiKeyRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RevokeApiKeyResponse {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteApiKeyRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteApiKeyResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_api_key() -> ApiKey {
        ApiKey {
            id: 42,
            organization_id: 7,
            name: "ci-bot".into(),
            description: Some("CI integration".into()),
            key_prefix: "amk_abcd1234".into(),
            scopes: vec!["pods:read".into(), "pods:write".into()],
            is_enabled: true,
            expires_at: None,
            last_used_at: Some("2026-05-09T10:00:00Z".into()),
            created_by: 1,
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-09T00:00:00Z".into(),
        }
    }

    #[test]
    fn api_key_round_trip_preserves_every_field() {
        let original = sample_api_key();
        let bytes = original.encode_to_vec();
        let decoded = ApiKey::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListApiKeysResponse {
            items: vec![sample_api_key()],
            total: 1,
            limit: 50,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListApiKeysResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.total, 1);
        assert_eq!(decoded.limit, 50);
        assert_eq!(decoded.offset, 0);
    }

    // Pinned by PR #345: raw_key MUST survive the wasm wrapper. The bug was
    // that a single-entity {api_key, raw_key} response was wrapped as
    // gin.H{"api_key": ..., "raw_key": ...} and a wrapper-stripping client
    // dropped raw_key. Proto-binary makes this physically impossible —
    // tag 2 stays tag 2 — and this test pins the contract.
    #[test]
    fn create_response_preserves_raw_key() {
        let original = CreateApiKeyResponse {
            api_key: Some(sample_api_key()),
            raw_key: "amk_abcd1234567890abcdef0123456789...".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateApiKeyResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.raw_key, "amk_abcd1234567890abcdef0123456789...");
        assert!(decoded.api_key.is_some(), "api_key tag 1 must round-trip alongside raw_key tag 2");
    }

    #[test]
    fn create_request_round_trip_with_optionals_set() {
        let original = CreateApiKeyRequest {
            org_slug: "acme".into(),
            name: "deploy-bot".into(),
            description: Some("CD pipeline".into()),
            scopes: vec!["pods:write".into()],
            expires_in: Some(86400),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateApiKeyRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn optional_offset_zero_distinguishable_from_absent() {
        // Conventions §5: `optional int32 offset = 2;` must distinguish
        // "absent" from "explicit 0". `c.DefaultQuery("offset", "0")` on
        // REST lost this distinction — binary wire preserves it.
        let with_zero = ListApiKeysRequest {
            org_slug: "acme".into(),
            offset: Some(0),
            limit: None,
        };
        let absent = ListApiKeysRequest {
            org_slug: "acme".into(),
            offset: None,
            limit: None,
        };
        let zero_bytes = with_zero.encode_to_vec();
        let absent_bytes = absent.encode_to_vec();
        assert_ne!(zero_bytes, absent_bytes,
            "explicit zero must encode different bytes from absent field");

        let r1 = ListApiKeysRequest::decode(&*zero_bytes).unwrap();
        let r2 = ListApiKeysRequest::decode(&*absent_bytes).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn delete_request_response_round_trip() {
        let req = DeleteApiKeyRequest { org_slug: "acme".into(), id: 99 };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, DeleteApiKeyRequest::decode(&*req_bytes).unwrap());

        let resp = DeleteApiKeyResponse {};
        let resp_bytes = resp.encode_to_vec();
        assert!(resp_bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(resp, DeleteApiKeyResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn revoke_request_response_round_trip() {
        let req = RevokeApiKeyRequest { org_slug: "acme".into(), id: 7 };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, RevokeApiKeyRequest::decode(&*req_bytes).unwrap());

        let resp = RevokeApiKeyResponse {};
        assert!(resp.encode_to_vec().is_empty());
    }

    #[test]
    fn update_request_optionals_distinguishable() {
        // `is_enabled` as `optional bool` distinguishes "unset" from "false":
        // a PATCH that omits is_enabled must NOT flip an enabled key to
        // disabled. Plain proto3 bool would conflate the two.
        let omit = UpdateApiKeyRequest {
            org_slug: "acme".into(),
            id: 1,
            name: Some("new-name".into()),
            description: None,
            scopes: vec![],
            is_enabled: None,
        };
        let set_false = UpdateApiKeyRequest {
            org_slug: "acme".into(),
            id: 1,
            name: Some("new-name".into()),
            description: None,
            scopes: vec![],
            is_enabled: Some(false),
        };
        assert_ne!(omit.encode_to_vec(), set_false.encode_to_vec(),
            "omitted is_enabled vs explicit false must encode differently");
    }
}
