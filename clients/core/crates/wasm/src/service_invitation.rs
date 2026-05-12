use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::InvitationService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInvitationService {
    inner: InvitationService,
}

#[wasm_bindgen]
impl WasmInvitationService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { inner: InvitationService::new(client) }
    }

    // -------- Legacy REST JSON methods (preserved during dual-track) --------

    pub async fn list(&self) -> Result<String, String> {
        self.inner.list().await
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        self.inner.create(json).await
    }

    pub async fn revoke(&self, id: i64) -> Result<(), String> {
        self.inner.revoke(id).await
    }

    pub async fn resend(&self, id: i64) -> Result<(), String> {
        self.inner.resend(id).await
    }

    pub async fn get_by_token(&self, token: &str) -> Result<String, String> {
        self.inner.get_by_token(token).await
    }

    pub async fn accept(&self, token: &str) -> Result<(), String> {
        self.inner.accept(token).await
    }

    pub async fn list_pending(&self) -> Result<String, String> {
        self.inner.list_pending().await
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes the request via @bufbuild/protobuf .toBinary(), passes the
    // Uint8Array in, receives a Uint8Array back, decodes via .fromBinary().
    // No JSON intermediate; conventions §2.5 forbids it on the client.
    //
    // js_name is camelCase; the `Connect` suffix marks the migration lane so
    // the legacy JSON methods can coexist until the UI fully cuts over.

    #[wasm_bindgen(js_name = listInvitationsConnect)]
    pub async fn list_invitations_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.list_invitations_connect(request).await
    }

    #[wasm_bindgen(js_name = createInvitationConnect)]
    pub async fn create_invitation_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.create_invitation_connect(request).await
    }

    #[wasm_bindgen(js_name = revokeInvitationConnect)]
    pub async fn revoke_invitation_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.revoke_invitation_connect(request).await
    }

    #[wasm_bindgen(js_name = resendInvitationConnect)]
    pub async fn resend_invitation_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.resend_invitation_connect(request).await
    }

    #[wasm_bindgen(js_name = acceptInvitationConnect)]
    pub async fn accept_invitation_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.accept_invitation_connect(request).await
    }

    #[wasm_bindgen(js_name = listPendingInvitationsConnect)]
    pub async fn list_pending_invitations_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.list_pending_invitations_connect(request).await
    }

    #[wasm_bindgen(js_name = getInvitationByTokenConnect)]
    pub async fn get_invitation_by_token_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.get_invitation_by_token_connect(request).await
    }
}
