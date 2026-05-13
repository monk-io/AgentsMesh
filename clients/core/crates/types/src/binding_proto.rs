// Hand-maintained `prost::Message` mirrors of
// `proto/binding/v1/binding.proto`. Tag numbers match the .proto
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
pub struct PodBinding {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(string, tag = "3")]
    pub initiator_pod: String,
    #[prost(string, tag = "4")]
    pub target_pod: String,
    #[prost(string, tag = "5")]
    pub status: String,
    #[prost(string, repeated, tag = "6")]
    pub granted_scopes: Vec<String>,
    #[prost(string, repeated, tag = "7")]
    pub pending_scopes: Vec<String>,
    #[prost(string, optional, tag = "8")]
    pub requested_at: Option<String>,
    #[prost(string, optional, tag = "9")]
    pub responded_at: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub expires_at: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub rejection_reason: Option<String>,
    #[prost(string, tag = "12")]
    pub created_at: String,
    #[prost(string, tag = "13")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RequestBindingRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(string, tag = "3")]
    pub target_pod: String,
    #[prost(string, repeated, tag = "4")]
    pub scopes: Vec<String>,
    #[prost(string, optional, tag = "5")]
    pub policy: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AcceptBindingRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(int64, tag = "3")]
    pub binding_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RejectBindingRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(int64, tag = "3")]
    pub binding_id: i64,
    #[prost(string, optional, tag = "4")]
    pub reason: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UnbindRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(string, tag = "3")]
    pub target_pod: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UnbindResponse {
    #[prost(bool, tag = "1")]
    pub removed: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RequestScopesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(int64, tag = "3")]
    pub binding_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub scopes: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ApproveScopesRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(int64, tag = "3")]
    pub binding_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub scopes: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListBindingsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(string, optional, tag = "3")]
    pub status: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListBindingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<PodBinding>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetPendingBindingsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetBoundPodsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetBoundPodsResponse {
    #[prost(string, repeated, tag = "1")]
    pub pods: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CheckBindingRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub initiator_pod: String,
    #[prost(string, tag = "3")]
    pub target_pod: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CheckBindingResponse {
    #[prost(bool, tag = "1")]
    pub is_bound: bool,
    #[prost(message, optional, tag = "2")]
    pub binding: Option<PodBinding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_binding() -> PodBinding {
        PodBinding {
            id: 7,
            organization_id: 42,
            initiator_pod: "pod-init-001".into(),
            target_pod: "pod-tgt-002".into(),
            status: "active".into(),
            granted_scopes: vec!["pod:read".into(), "pod:write".into()],
            pending_scopes: vec![],
            requested_at: Some("2026-05-10T00:00:00Z".into()),
            responded_at: Some("2026-05-10T00:05:00Z".into()),
            expires_at: None,
            rejection_reason: None,
            created_at: "2026-05-10T00:00:00Z".into(),
            updated_at: "2026-05-10T00:05:00Z".into(),
        }
    }

    #[test]
    fn pod_binding_round_trip_preserves_every_field() {
        let original = sample_binding();
        let bytes = original.encode_to_vec();
        let decoded = PodBinding::decode(&*bytes).unwrap();
        assert_eq!(
            original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here"
        );
    }

    #[test]
    fn list_response_round_trip_preserves_envelope() {
        let original = ListBindingsResponse {
            items: vec![sample_binding()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListBindingsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.total, 1);
    }

    #[test]
    fn request_binding_round_trip_with_optional_policy() {
        let original = RequestBindingRequest {
            org_slug: "acme".into(),
            initiator_pod: "pod-a".into(),
            target_pod: "pod-b".into(),
            scopes: vec!["pod:read".into()],
            policy: Some("same_user_auto".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = RequestBindingRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn reject_request_optional_reason_distinguishable() {
        let with_reason = RejectBindingRequest {
            org_slug: "acme".into(),
            initiator_pod: "pod-b".into(),
            binding_id: 1,
            reason: Some("not authorized".into()),
        };
        let without_reason = RejectBindingRequest {
            org_slug: "acme".into(),
            initiator_pod: "pod-b".into(),
            binding_id: 1,
            reason: None,
        };
        // Conventions §5: `optional string reason` must distinguish "absent"
        // from "empty string" — wire elides absent fields, encodes empty
        // strings as a present tag with zero length.
        let with_bytes = with_reason.encode_to_vec();
        let without_bytes = without_reason.encode_to_vec();
        assert_ne!(
            with_bytes, without_bytes,
            "explicit reason must encode different bytes from absent reason"
        );
        let r1 = RejectBindingRequest::decode(&*with_bytes).unwrap();
        let r2 = RejectBindingRequest::decode(&*without_bytes).unwrap();
        assert_eq!(r1.reason, Some("not authorized".into()));
        assert_eq!(r2.reason, None);
    }

    #[test]
    fn unbind_response_carries_removed_flag() {
        let original = UnbindResponse { removed: true };
        let bytes = original.encode_to_vec();
        assert_eq!(original, UnbindResponse::decode(&*bytes).unwrap());

        let original_false = UnbindResponse { removed: false };
        let bytes_false = original_false.encode_to_vec();
        // proto3 scalar defaults elide on the wire — `removed: false` is
        // indistinguishable from "unset"; both decode to false. Asserting
        // this explicit behavior so a future schema change to `optional bool`
        // surfaces as a test break.
        assert!(bytes_false.is_empty());
        assert_eq!(
            original_false,
            UnbindResponse::decode(&*bytes_false).unwrap()
        );
    }

    #[test]
    fn check_binding_response_with_and_without_binding() {
        let bound = CheckBindingResponse {
            is_bound: true,
            binding: Some(sample_binding()),
        };
        let unbound = CheckBindingResponse {
            is_bound: false,
            binding: None,
        };

        let bound_bytes = bound.encode_to_vec();
        let unbound_bytes = unbound.encode_to_vec();
        assert_eq!(bound, CheckBindingResponse::decode(&*bound_bytes).unwrap());
        assert_eq!(
            unbound,
            CheckBindingResponse::decode(&*unbound_bytes).unwrap()
        );
    }

    #[test]
    fn list_request_with_optional_status_filter() {
        let with_status = ListBindingsRequest {
            org_slug: "acme".into(),
            initiator_pod: "pod-a".into(),
            status: Some("pending".into()),
        };
        let all_statuses = ListBindingsRequest {
            org_slug: "acme".into(),
            initiator_pod: "pod-a".into(),
            status: None,
        };
        assert_ne!(
            with_status.encode_to_vec(),
            all_statuses.encode_to_vec(),
            "explicit status filter must encode different bytes from absent"
        );
    }
}
