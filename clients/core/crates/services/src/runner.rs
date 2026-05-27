use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::runner_state::RunnerState;
use agentsmesh_types::proto_runner_api_v1 as runner_proto;
use agentsmesh_types::proto_runner_api_v1::Runner;
use agentsmesh_types::proto_runner_state_v1::{
    PatchCachedRunnerRequest, RemoveCachedRunnerRequest, ReplaceAvailableRunnersRequest,
    ReplaceCachedRunnersRequest, SetCurrentRunnerRequest,
};
use prost::Message;

pub struct RunnerService {
    client: Arc<ApiClient>,
    state: RwLock<RunnerState>,
}

impl RunnerService {
    pub fn new(client: Arc<ApiClient>, state: RunnerState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    pub fn runners_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().runners()).unwrap_or_default()
    }

    pub fn available_runners_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().available_runners()).unwrap_or_default()
    }

    pub fn current_runner_json(&self) -> Option<String> {
        self.state.read().unwrap().current_runner()
            .map(|r| serde_json::to_string(r).unwrap_or_default())
    }

    pub fn get_runner_json(&self, id: i64) -> Option<String> {
        self.state.read().unwrap().get_runner(id)
            .map(|r| serde_json::to_string(r).unwrap_or_default())
    }

    pub fn replace_cached_runners(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceCachedRunnersRequest::decode(req_bytes)
            .map_err(|e| format!("decode replace_cached_runners: {e}"))?;
        self.state.write().unwrap().set_runners(req.runners);
        Ok(())
    }

    pub fn replace_available_runners(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = ReplaceAvailableRunnersRequest::decode(req_bytes)
            .map_err(|e| format!("decode replace_available_runners: {e}"))?;
        self.state.write().unwrap().set_available_runners(req.runners);
        Ok(())
    }

    pub fn set_current_runner_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = SetCurrentRunnerRequest::decode(req_bytes)
            .map_err(|e| format!("decode set_current_runner: {e}"))?;
        self.state.write().unwrap().set_current_runner(req.runner);
        Ok(())
    }

    pub fn patch_cached_runner(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = PatchCachedRunnerRequest::decode(req_bytes)
            .map_err(|e| format!("decode patch_cached_runner: {e}"))?;
        if let Some(r) = req.runner {
            let id = r.id;
            self.state.write().unwrap().update_runner(id, r);
        }
        Ok(())
    }

    pub fn remove_cached_runner(&self, req_bytes: &[u8]) -> Result<(), String> {
        let req = RemoveCachedRunnerRequest::decode(req_bytes)
            .map_err(|e| format!("decode remove_cached_runner: {e}"))?;
        self.state.write().unwrap().remove_runner(req.runner_id);
        Ok(())
    }

    pub fn update_runner_status(&self, id: i64, status: &str) {
        self.state.write().unwrap().update_runner_status(id, status);
    }

    pub async fn get_auth_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // No proto coverage on the wire for /runners/auth/<key> per se, but
        // GetRunnerAuthStatus is a Connect RPC — same path the legacy JSON
        // helper used, just expressed as wire-aligned proto bytes.
        let req = runner_proto::GetRunnerAuthStatusRequest::decode(request_bytes)
            .map_err(|e| format!("decode GetRunnerAuthStatusRequest: {e}"))?;
        let resp = self.client
            .get_runner_auth_status_connect(&req)
            .await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn authorize_runner_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Same reason as get_auth_status_connect — registration bootstrap.
        let mut req = runner_proto::AuthorizeRunnerRequest::decode(request_bytes)
            .map_err(|e| format!("decode AuthorizeRunnerRequest: {e}"))?;
        // The renderer can't always populate org_slug (registration happens
        // before the session knows which org the runner will land in). Fill
        // it from the session here for parity with the legacy helper.
        if req.org_slug.is_empty() {
            req.org_slug = self.client.current_org_slug();
        }
        let resp = self.client
            .authorize_runner_connect(&req)
            .await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes and returns prost-encoded
    // bytes — matching the wasm bridge's `Result<Vec<u8>, String>` surface
    // (conventions §2.5). Caller (TS) encodes via @bufbuild/protobuf .toBinary()
    // and decodes via .fromBinary().

    pub async fn list_runners_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::ListRunnersRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_runners request: {e}"))?;
        let resp = self.client.list_runners_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_available_runners_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::ListAvailableRunnersRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_available_runners request: {e}"))?;
        let resp = self.client.list_available_runners_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_runner_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::GetRunnerRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_runner request: {e}"))?;
        let resp = self.client.get_runner_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_runner_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::UpdateRunnerRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_runner request: {e}"))?;
        let resp = self.client.update_runner_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_runner_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::DeleteRunnerRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_runner request: {e}"))?;
        let resp = self.client.delete_runner_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn upgrade_runner_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::UpgradeRunnerRequest::decode(request_bytes)
            .map_err(|e| format!("decode upgrade_runner request: {e}"))?;
        let resp = self.client.upgrade_runner_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn request_log_upload_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::RequestLogUploadRequest::decode(request_bytes)
            .map_err(|e| format!("decode request_log_upload request: {e}"))?;
        let resp = self.client.request_log_upload_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_runner_logs_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::ListRunnerLogsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_runner_logs request: {e}"))?;
        let resp = self.client.list_runner_logs_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn query_sandboxes_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::QuerySandboxesRequest::decode(request_bytes)
            .map_err(|e| format!("decode query_sandboxes request: {e}"))?;
        let resp = self.client.query_sandboxes_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_runner_token_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::CreateRunnerTokenRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_runner_token request: {e}"))?;
        let resp = self.client.create_runner_token_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_runner_tokens_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::ListRunnerTokensRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_runner_tokens request: {e}"))?;
        let resp = self.client.list_runner_tokens_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_runner_token_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = runner_proto::DeleteRunnerTokenRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_runner_token request: {e}"))?;
        let resp = self.client.delete_runner_token_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
