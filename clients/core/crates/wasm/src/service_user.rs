use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::UserApiService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmUserApiService {
    inner: UserApiService,
}

#[wasm_bindgen]
impl WasmUserApiService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { inner: UserApiService::new(client) }
    }

    // -------- Legacy REST JSON methods (preserved during dual-track) --------

    pub async fn get_me(&self) -> Result<String, String> {
        self.inner.get_me().await
    }

    pub async fn get_organizations(&self) -> Result<String, String> {
        self.inner.get_organizations().await
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes the request via @bufbuild/protobuf .toBinary(), passes the
    // Uint8Array in, receives a Uint8Array back, decodes via .fromBinary().
    // No JSON intermediate; conventions §2.5 forbids it on the client.
    //
    // js_name is camelCase; the `Connect` suffix marks the migration lane so
    // the legacy JSON methods can coexist until the UI fully cuts over.

    #[wasm_bindgen(js_name = getMeConnect)]
    pub async fn get_me_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.get_me_connect(request).await
    }

    #[wasm_bindgen(js_name = updateMeConnect)]
    pub async fn update_me_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.update_me_connect(request).await
    }

    #[wasm_bindgen(js_name = changePasswordConnect)]
    pub async fn change_password_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.change_password_connect(request).await
    }

    #[wasm_bindgen(js_name = listIdentitiesConnect)]
    pub async fn list_identities_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.list_identities_connect(request).await
    }

    #[wasm_bindgen(js_name = deleteIdentityConnect)]
    pub async fn delete_identity_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.delete_identity_connect(request).await
    }

    #[wasm_bindgen(js_name = searchUsersConnect)]
    pub async fn search_users_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.inner.search_users_connect(request).await
    }
}
