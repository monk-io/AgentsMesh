// Hand-maintained `prost::Message` mirrors of
// `proto/user/v1/user.proto`. Tag numbers match the .proto byte-for-byte;
// `tools/validate_prost_tags` runs at build time to catch drift (watch
// list §8). NO `Serialize`/`Deserialize` derives on these structs —
// binary wire only (conventions §2.5, §3).
//
// USER-SCOPED service — no `org_slug` field on any request (conventions
// §3.5 exception #1). The auth interceptor supplies the user ID
// server-side via TenantContext.UserID. Search is also user-scoped (a
// signed-in user looking up another user by query).
//
// SENSITIVE FIELDS DROPPED: password_hash, email verification token,
// password reset token, OAuth access/refresh tokens — never appear on the
// proto, mirroring `json:"-"` on the GORM struct.

// ============================================================================
// User — full profile (GetMe / UpdateMe response shape)
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct User {
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
    #[prost(bool, tag = "6")]
    pub is_active: bool,
    #[prost(bool, tag = "7")]
    pub is_system_admin: bool,
    #[prost(bool, tag = "8")]
    pub is_email_verified: bool,
    #[prost(string, optional, tag = "9")]
    pub last_login_at: Option<String>,
    #[prost(int64, optional, tag = "10")]
    pub default_git_credential_id: Option<i64>,
    #[prost(string, tag = "11")]
    pub created_at: String,
    #[prost(string, tag = "12")]
    pub updated_at: String,
}

// ============================================================================
// Identity — linked OAuth identity (subset of domain Identity)
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct Identity {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub provider: String,
    #[prost(string, tag = "4")]
    pub provider_user_id: String,
    #[prost(string, optional, tag = "5")]
    pub provider_username: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub token_expires_at: Option<String>,
    #[prost(string, tag = "7")]
    pub created_at: String,
    #[prost(string, tag = "8")]
    pub updated_at: String,
}

// ============================================================================
// UserSummary — public-facing search result shape (subset of User)
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct UserSummary {
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

// ============================================================================
// /me — profile RPCs
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetMeRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateMeRequest {
    #[prost(string, optional, tag = "1")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub avatar_url: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ChangePasswordRequest {
    #[prost(string, tag = "1")]
    pub current_password: String,
    #[prost(string, tag = "2")]
    pub new_password: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ChangePasswordResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

// ============================================================================
// /me/identities — identity RPCs
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListIdentitiesRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListIdentitiesResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Identity>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteIdentityRequest {
    #[prost(string, tag = "1")]
    pub provider: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteIdentityResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

// ============================================================================
// /search — user search RPCs
// ============================================================================

#[derive(Clone, PartialEq, prost::Message)]
pub struct SearchUsersRequest {
    #[prost(string, tag = "1")]
    pub q: String,
    #[prost(int32, optional, tag = "2")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SearchUsersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<UserSummary>,
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

    fn sample_user() -> User {
        User {
            id: 42,
            email: "alice@example.com".into(),
            username: "alice".into(),
            name: Some("Alice".into()),
            avatar_url: Some("https://cdn.example.com/a.png".into()),
            is_active: true,
            is_system_admin: false,
            is_email_verified: true,
            last_login_at: Some("2026-05-12T13:16:10Z".into()),
            default_git_credential_id: Some(7),
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
        }
    }

    fn sample_identity() -> Identity {
        Identity {
            id: 101,
            user_id: 42,
            provider: "github".into(),
            provider_user_id: "1234567".into(),
            provider_username: Some("alice-gh".into()),
            token_expires_at: Some("2026-06-01T00:00:00Z".into()),
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
        }
    }

    #[test]
    fn user_round_trip_preserves_every_field() {
        let original = sample_user();
        let bytes = original.encode_to_vec();
        let decoded = User::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn user_minimal_round_trip() {
        // Every optional unset — exercises the absent-field encoding.
        let original = User {
            id: 1,
            email: "bob@example.com".into(),
            username: "bob".into(),
            name: None,
            avatar_url: None,
            is_active: true,
            is_system_admin: false,
            is_email_verified: false,
            last_login_at: None,
            default_git_credential_id: None,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = User::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.name.is_none());
        assert!(decoded.avatar_url.is_none());
        assert!(decoded.last_login_at.is_none());
        assert!(decoded.default_git_credential_id.is_none());
    }

    #[test]
    fn identity_round_trip_preserves_every_field() {
        let original = sample_identity();
        let bytes = original.encode_to_vec();
        let decoded = Identity::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn user_summary_round_trip() {
        let original = UserSummary {
            id: 42,
            email: "alice@example.com".into(),
            username: "alice".into(),
            name: Some("Alice".into()),
            avatar_url: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = UserSummary::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.avatar_url.is_none(),
            "absent optional must remain absent post-round-trip");
    }

    #[test]
    fn update_me_partial_request_round_trip() {
        let original = UpdateMeRequest {
            name: Some("New Name".into()),
            avatar_url: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = UpdateMeRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert!(decoded.avatar_url.is_none());
    }

    #[test]
    fn change_password_round_trip() {
        let req = ChangePasswordRequest {
            current_password: "old".into(),
            new_password: "newpassword123".into(),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, ChangePasswordRequest::decode(&*bytes).unwrap());

        let resp = ChangePasswordResponse {
            message: "Password changed successfully".into(),
        };
        let resp_bytes = resp.encode_to_vec();
        assert_eq!(resp, ChangePasswordResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn delete_identity_round_trip() {
        let req = DeleteIdentityRequest {
            provider: "github".into(),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, DeleteIdentityRequest::decode(&*bytes).unwrap());

        let resp = DeleteIdentityResponse {
            message: "Identity removed".into(),
        };
        let resp_bytes = resp.encode_to_vec();
        assert_eq!(resp, DeleteIdentityResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn list_identities_response_round_trip() {
        let original = ListIdentitiesResponse {
            items: vec![sample_identity()],
            total: 1,
            limit: 4,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListIdentitiesResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
    }

    #[test]
    fn search_users_response_round_trip() {
        let original = SearchUsersResponse {
            items: vec![UserSummary {
                id: 1,
                email: "alice@example.com".into(),
                username: "alice".into(),
                name: Some("Alice".into()),
                avatar_url: None,
            }],
            total: 1,
            limit: 10,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = SearchUsersResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn search_users_optional_limit_zero_distinguishable_from_absent() {
        // Conventions §5: `optional int32 limit = 2;` must distinguish
        // "absent" (server applies default 10) from "explicit 0" (which the
        // handler clamps to default; still — the wire must round-trip the
        // user's intent).
        let with_zero = SearchUsersRequest {
            q: "a".into(),
            limit: Some(0),
        };
        let absent = SearchUsersRequest {
            q: "a".into(),
            limit: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
        let r1 = SearchUsersRequest::decode(&*with_zero.encode_to_vec()).unwrap();
        let r2 = SearchUsersRequest::decode(&*absent.encode_to_vec()).unwrap();
        assert_eq!(r1.limit, Some(0));
        assert_eq!(r2.limit, None);
    }

    #[test]
    fn get_me_request_is_empty_on_wire() {
        let req = GetMeRequest {};
        let bytes = req.encode_to_vec();
        assert!(bytes.is_empty(), "user-scoped /me has no payload");
        assert_eq!(req, GetMeRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn list_identities_request_is_empty_on_wire() {
        let req = ListIdentitiesRequest {};
        let bytes = req.encode_to_vec();
        assert!(bytes.is_empty(), "user-scoped /me/identities has no payload");
        assert_eq!(req, ListIdentitiesRequest::decode(&*bytes).unwrap());
    }
}
