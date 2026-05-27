// NAPI auth bindings — Buffer (proto bytes) flavour.
//
// Each method mirrors a JSON-string counterpart on `AppState` (see
// lib.rs:auth_*) but returns prost-encoded bytes instead of
// `serde_json::to_string`. Callers on the Electron main / renderer side
// decode using the corresponding `proto.auth.v1.*` (wire reuse) or
// `proto.auth_state.v1.*` (state-only) message schema.
//
// Cohabitation strategy: the legacy JSON methods stay mounted until all
// callers cut over. This file ONLY adds the new proto surface — it does
// not remove the JSON twins. Renderer migration lives in a follow-up PR
// (ElectronAuthService cutover; e2e contract fixtures regen).

use napi_derive::napi;
use prost::Message;

use agentsmesh_auth::BootstrapResult;
use agentsmesh_state::auth_types::{AuthSession, AuthTokens, Organization, User};
use agentsmesh_types::{proto_auth_state_v1 as auth_state, proto_auth_v1 as auth_proto,
    proto_org_v1 as org_proto};

use crate::{err, AppState};

// ---- DTO → proto mapping helpers ----
//
// AuthManager keeps its own serde DTOs (auth_types::*) because the
// persisted-session JSON the disk store reads/writes uses the legacy
// snake_case shape. The mapping helpers down-project from the DTOs to
// the proto wire shape so the NAPI surface speaks bytes-only.

fn user_to_proto(u: &User) -> auth_proto::User {
    auth_proto::User {
        id: u.id,
        email: u.email.clone(),
        username: u.username.clone(),
        name: u.name.clone(),
        avatar_url: u.avatar_url.clone(),
        is_email_verified: u.is_email_verified,
    }
}

fn org_to_proto(o: &Organization) -> org_proto::Organization {
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

fn session_to_login_response(s: AuthSession) -> auth_proto::LoginResponse {
    auth_proto::LoginResponse {
        token: s.token,
        refresh_token: s.refresh_token,
        expires_in: s.expires_in.unwrap_or(0),
        user: Some(user_to_proto(&s.user)),
    }
}

fn tokens_to_refresh_response(t: AuthTokens) -> auth_proto::RefreshTokenResponse {
    auth_proto::RefreshTokenResponse {
        token: t.token,
        refresh_token: t.refresh_token,
        expires_in: t.expires_in.unwrap_or(0),
    }
}

fn bootstrap_to_proto(r: BootstrapResult) -> auth_state::BootstrapResult {
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

// ---- NAPI surface ----

#[napi]
impl AppState {
    #[napi]
    pub async fn auth_login_proto(
        &self,
        email: String,
        password: String,
    ) -> napi::Result<Vec<u8>> {
        let session = self.auth.login(&email, &password).await.map_err(err)?;
        Ok(session_to_login_response(session).encode_to_vec())
    }

    #[napi]
    pub async fn auth_refresh_token_proto(&self) -> napi::Result<Vec<u8>> {
        let tokens = self.auth.refresh_token().await.map_err(err)?;
        Ok(tokens_to_refresh_response(tokens).encode_to_vec())
    }

    #[napi]
    pub async fn auth_fetch_organizations_proto(&self) -> napi::Result<Vec<u8>> {
        let orgs = self.auth.fetch_organizations().await.map_err(err)?;
        let items: Vec<org_proto::Organization> = orgs.iter().map(org_to_proto).collect();
        Ok(auth_state::OrganizationsList { items }.encode_to_vec())
    }

    #[napi]
    pub async fn auth_bootstrap_proto(&self) -> napi::Result<Vec<u8>> {
        let result = self.auth.bootstrap().await;
        Ok(bootstrap_to_proto(result).encode_to_vec())
    }

    // None when no session is loaded — matches the legacy
    // `auth_get_current_user_json` shape semantics.
    #[napi]
    pub fn auth_get_current_user_proto(&self) -> Option<Vec<u8>> {
        self.auth
            .current_user()
            .map(|u| user_to_proto(&u).encode_to_vec())
    }
}
