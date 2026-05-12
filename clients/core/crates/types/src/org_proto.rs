// Hand-maintained `prost::Message` mirrors of `proto/org/v1/org.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift (watch list §8). NO `Serialize` /
// `Deserialize` derives — binary wire only (conventions §2.5, §3).

#[derive(Clone, PartialEq, prost::Message)]
pub struct Organization {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub slug: String,
    #[prost(string, optional, tag = "4")]
    pub logo_url: Option<String>,
    #[prost(string, tag = "5")]
    pub subscription_plan: String,
    #[prost(string, tag = "6")]
    pub subscription_status: String,
    #[prost(string, optional, tag = "7")]
    pub role: Option<String>,
    #[prost(string, tag = "8")]
    pub created_at: String,
    #[prost(string, tag = "9")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct OrganizationMember {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub organization_id: i64,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(string, tag = "4")]
    pub role: String,
    #[prost(string, tag = "5")]
    pub joined_at: String,
    #[prost(message, optional, tag = "6")]
    pub user: Option<MemberUser>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MemberUser {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub email: String,
    #[prost(string, tag = "3")]
    pub username: String,
    #[prost(string, optional, tag = "4")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub avatar_url: Option<String>,
}

// --- User-scoped requests/responses ---

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListMyOrgsRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListMyOrgsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Organization>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateOrgRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub slug: String,
    #[prost(string, optional, tag = "3")]
    pub logo_url: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreatePersonalOrgRequest {}

// --- Org-scoped requests/responses ---

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetOrgRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateOrgRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub logo_url: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteOrgRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteOrgResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListMembersRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListMembersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<OrganizationMember>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct InviteMemberRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, optional, tag = "2")]
    pub email: Option<String>,
    #[prost(int64, optional, tag = "3")]
    pub user_id: Option<i64>,
    #[prost(string, tag = "4")]
    pub role: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct InviteMemberResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RemoveMemberRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RemoveMemberResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateMemberRoleRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub role: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateMemberRoleResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_org() -> Organization {
        Organization {
            id: 42,
            name: "Acme".into(),
            slug: "acme".into(),
            logo_url: Some("https://cdn.example.com/logo.png".into()),
            subscription_plan: "pro".into(),
            subscription_status: "active".into(),
            role: Some("owner".into()),
            created_at: "2026-05-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
        }
    }

    fn sample_member() -> OrganizationMember {
        OrganizationMember {
            id: 7,
            organization_id: 42,
            user_id: 100,
            role: "admin".into(),
            joined_at: "2026-05-08T00:00:00Z".into(),
            user: Some(MemberUser {
                id: 100,
                email: "alice@example.com".into(),
                username: "alice".into(),
                name: Some("Alice Smith".into()),
                avatar_url: None,
            }),
        }
    }

    #[test]
    fn organization_round_trip_preserves_every_field() {
        let original = sample_org();
        let bytes = original.encode_to_vec();
        let decoded = Organization::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn organization_platform_defaults_round_trip() {
        let original = Organization {
            id: 1,
            name: "Personal".into(),
            slug: "alice-workspace".into(),
            logo_url: None,
            subscription_plan: "based".into(),
            subscription_status: "trialing".into(),
            role: None,
            created_at: "2026-05-12T00:00:00Z".into(),
            updated_at: "2026-05-12T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = Organization::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.logo_url.is_none());
        assert!(decoded.role.is_none());
    }

    #[test]
    fn member_with_nested_user_round_trips() {
        let original = sample_member();
        let bytes = original.encode_to_vec();
        let decoded = OrganizationMember::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        let nested = decoded.user.unwrap();
        assert_eq!(nested.email, "alice@example.com");
    }

    #[test]
    fn list_my_orgs_response_round_trip() {
        let original = ListMyOrgsResponse {
            items: vec![sample_org()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListMyOrgsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
    }

    #[test]
    fn list_members_response_round_trip() {
        let original = ListMembersResponse {
            items: vec![sample_member()],
            total: 1,
            limit: 50,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListMembersResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn create_org_request_round_trip() {
        let original = CreateOrgRequest {
            name: "New Org".into(),
            slug: "new-org".into(),
            logo_url: Some("https://cdn.example.com/x.png".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateOrgRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn update_org_partial_request_round_trip() {
        let original = UpdateOrgRequest {
            org_slug: "acme".into(),
            name: Some("Acme Inc".into()),
            logo_url: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = UpdateOrgRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.logo_url.is_none(),
            "absent optional must remain absent post-round-trip");
    }

    #[test]
    fn invite_member_email_only_round_trip() {
        let original = InviteMemberRequest {
            org_slug: "acme".into(),
            email: Some("bob@example.com".into()),
            user_id: None,
            role: "member".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = InviteMemberRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn invite_member_user_id_only_round_trip() {
        let original = InviteMemberRequest {
            org_slug: "acme".into(),
            email: None,
            user_id: Some(99),
            role: "admin".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = InviteMemberRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn list_members_optional_offset_zero_distinguishable_from_absent() {
        // Conventions §5: `optional int32 offset = 2;` must distinguish
        // "absent" from "explicit 0" — pagination drifted on REST when this
        // was lost (watch list §3).
        let with_zero = ListMembersRequest {
            org_slug: "acme".into(),
            offset: Some(0),
            limit: None,
        };
        let absent = ListMembersRequest {
            org_slug: "acme".into(),
            offset: None,
            limit: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
        let r1 = ListMembersRequest::decode(&*with_zero.encode_to_vec()).unwrap();
        let r2 = ListMembersRequest::decode(&*absent.encode_to_vec()).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn delete_org_request_response_round_trip() {
        let req = DeleteOrgRequest { org_slug: "acme".into() };
        let req_bytes = req.encode_to_vec();
        assert_eq!(req, DeleteOrgRequest::decode(&*req_bytes).unwrap());

        let resp = DeleteOrgResponse { message: "Organization deleted".into() };
        let resp_bytes = resp.encode_to_vec();
        assert_eq!(resp, DeleteOrgResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn update_member_role_round_trip() {
        let req = UpdateMemberRoleRequest {
            org_slug: "acme".into(),
            user_id: 100,
            role: "admin".into(),
        };
        let bytes = req.encode_to_vec();
        let decoded = UpdateMemberRoleRequest::decode(&*bytes).unwrap();
        assert_eq!(req, decoded);
    }

    #[test]
    fn list_my_orgs_request_is_empty_on_wire() {
        let req = ListMyOrgsRequest {};
        let bytes = req.encode_to_vec();
        assert!(bytes.is_empty(), "user-scoped list-my-orgs has no payload");
        assert_eq!(req, ListMyOrgsRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn create_personal_org_request_is_empty_on_wire() {
        let req = CreatePersonalOrgRequest {};
        let bytes = req.encode_to_vec();
        assert!(bytes.is_empty(), "personal-org request payload is server-derived");
        assert_eq!(req, CreatePersonalOrgRequest::decode(&*bytes).unwrap());
    }
}
