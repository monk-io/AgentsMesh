// NAPI auth proto ↔ AuthManager DTO converters. Mirrors the wasm
// counterpart in clients/core/crates/wasm/src/auth_proto_convert.rs —
// kept local to node-bridge so the DTO ↔ wire mapping stays close to
// the NAPI surface that depends on it.

use agentsmesh_auth::BootstrapResult;
use agentsmesh_state::auth_types::{AuthSession, AuthTokens, Organization, User};
use agentsmesh_types::{proto_auth_state_v1 as auth_state, proto_auth_v1 as auth_proto,
    proto_org_v1 as org_proto};

pub(crate) fn user_to_proto(u: &User) -> auth_proto::User {
    auth_proto::User {
        id: u.id,
        email: u.email.clone(),
        username: u.username.clone(),
        name: u.name.clone(),
        avatar_url: u.avatar_url.clone(),
        is_email_verified: u.is_email_verified,
    }
}

pub(crate) fn org_to_proto(o: &Organization) -> org_proto::Organization {
    // The DTO carries 7 fields; proto.org.v1.Organization has 9 (adds
    // created_at + updated_at). AuthManager never reads those timestamps
    // so empty strings are correct here — the SSOT remains the
    // server-side row, which downstream callers re-fetch through
    // proto.org.v1.OrgService when needed.
    org_proto::Organization {
        id: o.id,
        name: o.name.clone(),
        slug: o.slug.clone(),
        logo_url: o.logo_url.clone(),
        subscription_plan: o.subscription_plan.clone().unwrap_or_default(),
        subscription_status: o.subscription_status.clone().unwrap_or_default(),
        role: o.role.clone(),
        created_at: String::new(),
        updated_at: String::new(),
    }
}

pub(crate) fn session_to_login_response(s: AuthSession) -> auth_proto::LoginResponse {
    auth_proto::LoginResponse {
        token: s.token,
        refresh_token: s.refresh_token,
        expires_in: s.expires_in.unwrap_or(0),
        user: Some(user_to_proto(&s.user)),
    }
}

pub(crate) fn tokens_to_refresh_response(t: AuthTokens) -> auth_proto::RefreshTokenResponse {
    auth_proto::RefreshTokenResponse {
        token: t.token,
        refresh_token: t.refresh_token,
        expires_in: t.expires_in.unwrap_or(0),
    }
}

pub(crate) fn bootstrap_to_proto(r: BootstrapResult) -> auth_state::BootstrapResult {
    use agentsmesh_auth::BootstrapCleanupReason as R;
    let cleanup_str = |r: R| match r {
        R::BaseUrlMismatch => "base_url_mismatch",
        R::TokenExpiredAndRefreshFailed => "token_expired_and_refresh_failed",
        R::UnauthorizedFromIdentityCall => "unauthorized_from_identity_call",
        R::StorageCorrupt => "storage_corrupt",
        R::LegacyDataPurged => "legacy_data_purged",
    };
    match r {
        BootstrapResult::Anonymous => auth_state::BootstrapResult {
            kind: "anonymous".into(),
            ..Default::default()
        },
        BootstrapResult::AnonymousAfterCleanup { reason } => auth_state::BootstrapResult {
            kind: "anonymous_after_cleanup".into(),
            cleanup_reason: Some(cleanup_str(reason).into()),
            ..Default::default()
        },
        BootstrapResult::Authenticated { user, current_org } => auth_state::BootstrapResult {
            kind: "authenticated".into(),
            user: Some(user_to_proto(&user)),
            current_org: current_org.as_ref().map(org_to_proto),
            ..Default::default()
        },
    }
}

pub(crate) fn user_from_proto(u: &auth_proto::User) -> User {
    User {
        id: u.id,
        email: u.email.clone(),
        username: u.username.clone(),
        name: u.name.clone(),
        avatar_url: u.avatar_url.clone(),
        is_email_verified: u.is_email_verified,
    }
}

pub(crate) fn org_from_proto(o: &org_proto::Organization) -> Organization {
    let subscription_plan = if o.subscription_plan.is_empty() {
        None
    } else {
        Some(o.subscription_plan.clone())
    };
    let subscription_status = if o.subscription_status.is_empty() {
        None
    } else {
        Some(o.subscription_status.clone())
    };
    Organization {
        id: o.id,
        name: o.name.clone(),
        slug: o.slug.clone(),
        role: o.role.clone(),
        logo_url: o.logo_url.clone(),
        subscription_plan,
        subscription_status,
    }
}
