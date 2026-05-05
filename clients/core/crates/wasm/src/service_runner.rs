use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::RunnerService;
use agentsmesh_state::runner_state::RunnerState;
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

    pub fn get_runner_json(&self, id: i64) -> JsValue {
        match self.0.get_runner_json(id) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn set_runners(&self, json: &str) { self.0.set_runners(json); }
    pub fn set_available_runners(&self, json: &str) { self.0.set_available_runners(json); }
    pub fn set_current_runner(&self, json: &str) { self.0.set_current_runner(json); }

    pub fn update_runner_local(&self, id: f64, json: &str) {
        self.0.update_runner_local(id, json);
    }

    pub fn update_runner_status(&self, id: i64, status: &str) {
        self.0.update_runner_status(id, status);
    }

    pub fn remove_runner_local(&self, id: i64) { self.0.remove_runner_local(id); }

    pub async fn fetch_runners(&self, status: Option<String>) -> Result<String, String> {
        self.0.fetch_runners(status).await
    }

    pub async fn fetch_available_runners(&self) -> Result<String, String> {
        self.0.fetch_available_runners().await
    }

    pub async fn fetch_runner(&self, id: i64) -> Result<String, String> {
        self.0.fetch_runner(id).await
    }

    pub async fn update_runner(&self, id: i64, request_json: &str) -> Result<String, String> {
        self.0.update_runner(id, request_json).await
    }

    pub async fn delete_runner(&self, id: i64) -> Result<(), String> {
        self.0.delete_runner(id).await
    }

    pub async fn create_token(&self, request_json: &str) -> Result<String, String> {
        self.0.create_token(request_json).await
    }

    pub async fn fetch_tokens(&self) -> Result<String, String> {
        self.0.fetch_tokens().await
    }

    pub async fn delete_token(&self, id: i64) -> Result<(), String> {
        self.0.delete_token(id).await
    }

    pub async fn list_runner_logs(&self, id: i64) -> Result<String, String> {
        self.0.list_runner_logs(id).await
    }

    pub async fn request_log_upload(&self, id: i64) -> Result<(), String> {
        self.0.request_log_upload(id).await
    }

    pub async fn upgrade_runner(&self, id: i64, request_json: &str) -> Result<String, String> {
        self.0.upgrade_runner(id, request_json).await
    }

    pub async fn list_runner_pods(
        &self, id: i64, status: Option<String>, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        self.0.list_runner_pods(id, status, limit, offset).await
    }

    pub async fn query_runner_sandboxes(&self, id: i64, request_json: &str) -> Result<String, String> {
        self.0.query_runner_sandboxes(id, request_json).await
    }

    pub async fn get_auth_status(&self, auth_key: &str) -> Result<String, String> {
        self.0.get_auth_status(auth_key).await
    }

    pub async fn authorize_runner(&self, request_json: &str) -> Result<String, String> {
        self.0.authorize_runner(request_json).await
    }
}
