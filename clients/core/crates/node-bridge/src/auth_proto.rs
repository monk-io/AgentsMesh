// NAPI auth bindings — Buffer (proto bytes) flavour.
//
// Each method mirrors a JSON-string counterpart on `AppState` (see
// lib.rs:auth_*) but returns prost-encoded bytes instead of
// `serde_json::to_string`. Callers on the Electron main / renderer side
// decode using the corresponding `proto.auth.v1.*` (wire reuse) or
// `proto.auth_state.v1.*` (state-only) message schema.
//
// Cohabitation strategy: the legacy JSON methods stay mounted until all
// callers cut over. Renderer migration of state mutators lives in the
// renderer cutover commit (web auth.ts now uses the proto-bytes setters).
//
// Converter helpers live in `auth_proto_convert.rs` to keep this file
// under the 200-line SRP cap.

use napi_derive::napi;
use prost::Message;

use agentsmesh_state::auth_types::{AuthSession, Organization};
use agentsmesh_types::{proto_auth_state_v1 as auth_state, proto_org_v1 as org_proto};

use crate::auth_proto_convert::{
    bootstrap_to_proto, org_from_proto, org_to_proto, session_to_login_response,
    tokens_to_refresh_response, user_from_proto, user_to_proto,
};
use crate::{err, AppState};

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

    // ---- Proto-bytes mutators (mirror wasm AuthManager.apply_session etc) ----

    #[napi]
    pub fn auth_apply_session_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = auth_state::ApplySessionRequest::decode(req_bytes.as_slice())
            .map_err(|e| err(format!("decode ApplySessionRequest: {e}")))?;
        let user_proto = req
            .user
            .ok_or_else(|| err("ApplySessionRequest.user missing"))?;
        let session = AuthSession {
            token: req.token,
            refresh_token: req.refresh_token,
            user: user_from_proto(&user_proto),
            expires_in: None,
            message: None,
        };
        self.auth.apply_session(&session);
        Ok(())
    }

    #[napi]
    pub fn auth_set_organizations_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = auth_state::SetOrganizationsRequest::decode(req_bytes.as_slice())
            .map_err(|e| err(format!("decode SetOrganizationsRequest: {e}")))?;
        let orgs: Vec<Organization> = req.items.iter().map(org_from_proto).collect();
        self.auth.replace_organizations(orgs);
        Ok(())
    }

    #[napi]
    pub fn auth_set_current_org_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = auth_state::SetCurrentOrgRequest::decode(req_bytes.as_slice())
            .map_err(|e| err(format!("decode SetCurrentOrgRequest: {e}")))?;
        match req.org.as_ref() {
            Some(o) => self.auth.set_current_org(Some(org_from_proto(o))),
            None => self.auth.set_current_org(None),
        }
        Ok(())
    }
}
