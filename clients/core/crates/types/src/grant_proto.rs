// Hand-maintained `prost::Message` mirrors of `proto/grant/v1/grant.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift. NO `Serialize` / `Deserialize` derives —
// binary wire only (conventions §2.5, §3).
//
// Org-scoped service (conventions §3.5): every request carries
// `org_slug = 1` and is resolved by ResolveOrgScope.

#[derive(Clone, PartialEq, prost::Message)]
pub struct ResourceGrantUser {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub email: String,
    #[prost(string, tag = "3")]
    pub username: String,
    #[prost(string, optional, tag = "4")]
    pub name: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ResourceGrant {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub resource_type: String,
    #[prost(string, tag = "3")]
    pub resource_id: String,
    #[prost(int64, tag = "4")]
    pub user_id: i64,
    #[prost(int64, tag = "5")]
    pub granted_by: i64,
    #[prost(string, tag = "6")]
    pub created_at: String,
    #[prost(message, optional, tag = "7")]
    pub user: Option<ResourceGrantUser>,
    #[prost(message, optional, tag = "8")]
    pub granted_by_user: Option<ResourceGrantUser>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListGrantsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub resource_type: String,
    #[prost(string, tag = "3")]
    pub resource_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<ResourceGrant>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateGrantRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub resource_type: String,
    #[prost(string, tag = "3")]
    pub resource_id: String,
    #[prost(int64, tag = "4")]
    pub user_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteGrantRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub resource_type: String,
    #[prost(string, tag = "3")]
    pub resource_id: String,
    #[prost(int64, tag = "4")]
    pub grant_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteGrantResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_user() -> ResourceGrantUser {
        ResourceGrantUser {
            id: 7,
            email: "alice@example.com".into(),
            username: "alice".into(),
            name: Some("Alice".into()),
        }
    }

    fn sample_grant() -> ResourceGrant {
        ResourceGrant {
            id: 42,
            resource_type: "pod".into(),
            resource_id: "pod-key-abc".into(),
            user_id: 7,
            granted_by: 3,
            created_at: "2026-05-12T13:16:10Z".into(),
            user: Some(sample_user()),
            granted_by_user: Some(ResourceGrantUser {
                id: 3,
                email: "admin@example.com".into(),
                username: "admin".into(),
                name: None,
            }),
        }
    }

    #[test]
    fn resource_grant_round_trip() {
        let original = sample_grant();
        let bytes = original.encode_to_vec();
        let decoded = ResourceGrant::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn list_grants_response_round_trip() {
        let original = ListGrantsResponse {
            items: vec![sample_grant()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListGrantsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn create_grant_request_round_trip() {
        let req = CreateGrantRequest {
            org_slug: "acme".into(),
            resource_type: "runner".into(),
            resource_id: "42".into(),
            user_id: 7,
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, CreateGrantRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn delete_grant_request_round_trip() {
        let req = DeleteGrantRequest {
            org_slug: "acme".into(),
            resource_type: "repository".into(),
            resource_id: "99".into(),
            grant_id: 42,
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, DeleteGrantRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn resource_grant_without_associations_round_trips() {
        let bare = ResourceGrant {
            id: 1,
            resource_type: "pod".into(),
            resource_id: "abc".into(),
            user_id: 2,
            granted_by: 3,
            created_at: "2026-05-12T00:00:00Z".into(),
            user: None,
            granted_by_user: None,
        };
        let bytes = bare.encode_to_vec();
        let decoded = ResourceGrant::decode(&*bytes).unwrap();
        assert_eq!(bare, decoded);
        assert!(decoded.user.is_none());
        assert!(decoded.granted_by_user.is_none());
    }
}
