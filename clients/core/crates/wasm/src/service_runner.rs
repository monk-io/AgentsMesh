use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::RunnerService;
use agentsmesh_state::runner_state::RunnerState;
use agentsmesh_types::proto_runner_state_v1::ApplyRunnerStatusEventRequest;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRunnerService(pub(crate) RunnerService);

#[wasm_bindgen]
impl WasmRunnerService {
    pub(crate) fn new(client: Arc<ApiClient>, state: RunnerState) -> Self {
        Self(RunnerService::new(client, state))
    }

    pub fn runners_json(&self) -> String { self.0.runners_json() }
    pub fn available_runners_json(&self) -> String { self.0.available_runners_json() }

    pub fn current_runner_json(&self) -> JsValue {
        match self.0.current_runner_json() {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_runners(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.replace_cached_runners(req_bytes)
    }

    pub fn replace_available_runners(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.replace_available_runners(req_bytes)
    }

    pub fn set_current_runner_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.set_current_runner_proto(req_bytes)
    }

    pub fn patch_cached_runner(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.patch_cached_runner(req_bytes)
    }

    pub fn remove_cached_runner(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.remove_cached_runner(req_bytes)
    }

    pub fn apply_runner_status_event(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ApplyRunnerStatusEventRequest::decode(req_bytes)
            .map_err(|e| format!("decode apply_runner_status_event: {e}"))?;
        self.0.update_runner_status(req.runner_id, &req.status);
        Ok(())
    }

    pub async fn list_runner_pods(
        &self, id: i64, status: Option<String>, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        self.0.list_runner_pods(id, status, limit, offset).await
    }

    pub async fn get_auth_status(&self, auth_key: &str) -> Result<String, String> {
        self.0.get_auth_status(auth_key).await
    }

    pub async fn authorize_runner(&self, request_json: &str) -> Result<String, String> {
        self.0.authorize_runner(request_json).await
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes the request via @bufbuild/protobuf .toBinary(), passes the
    // Uint8Array in, receives a Uint8Array back, decodes via .fromBinary().
    // No JSON intermediate; conventions §2.5 forbids it on the client.
    //
    // js_name is camelCase to match the existing JS-side conventions; the
    // `_connect` suffix marks the migration lane so the legacy JSON methods
    // can coexist until all 26 services flip.

    #[wasm_bindgen(js_name = listRunnersConnect)]
    pub async fn list_runners_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_runners_connect(request).await
    }

    #[wasm_bindgen(js_name = listAvailableRunnersConnect)]
    pub async fn list_available_runners_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_available_runners_connect(request).await
    }

    #[wasm_bindgen(js_name = getRunnerConnect)]
    pub async fn get_runner_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_runner_connect(request).await
    }

    #[wasm_bindgen(js_name = updateRunnerConnect)]
    pub async fn update_runner_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.update_runner_connect(request).await
    }

    #[wasm_bindgen(js_name = deleteRunnerConnect)]
    pub async fn delete_runner_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.delete_runner_connect(request).await
    }

    #[wasm_bindgen(js_name = upgradeRunnerConnect)]
    pub async fn upgrade_runner_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.upgrade_runner_connect(request).await
    }

    #[wasm_bindgen(js_name = requestLogUploadConnect)]
    pub async fn request_log_upload_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.request_log_upload_connect(request).await
    }

    #[wasm_bindgen(js_name = listRunnerLogsConnect)]
    pub async fn list_runner_logs_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_runner_logs_connect(request).await
    }

    #[wasm_bindgen(js_name = querySandboxesConnect)]
    pub async fn query_sandboxes_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.query_sandboxes_connect(request).await
    }

    #[wasm_bindgen(js_name = createRunnerTokenConnect)]
    pub async fn create_runner_token_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.create_runner_token_connect(request).await
    }

    #[wasm_bindgen(js_name = listRunnerTokensConnect)]
    pub async fn list_runner_tokens_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_runner_tokens_connect(request).await
    }

    #[wasm_bindgen(js_name = deleteRunnerTokenConnect)]
    pub async fn delete_runner_token_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.delete_runner_token_connect(request).await
    }
}
