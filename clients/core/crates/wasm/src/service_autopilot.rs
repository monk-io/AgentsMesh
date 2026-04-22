use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::AutopilotService;
use agentsmesh_state::autopilot_state::AutopilotState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmAutopilotService(pub(crate) AutopilotService);

#[wasm_bindgen]
impl WasmAutopilotService {
    pub(crate) fn new(client: Arc<ApiClient>, state: AutopilotState) -> Self {
        Self(AutopilotService::new(client, state))
    }

    pub fn controllers_json(&self) -> String { self.0.controllers_json() }

    pub fn current_controller_json(&self) -> JsValue {
        match self.0.current_controller_json() {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_controller_by_pod_key_json(&self, pod_key: &str) -> JsValue {
        match self.0.get_controller_by_pod_key_json(pod_key) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_iterations_json(&self, key: &str) -> JsValue {
        match self.0.get_iterations_json(key) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_thinking_json(&self, key: &str) -> JsValue {
        match self.0.get_thinking_json(key) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn get_thinking_history_json(&self, key: &str) -> JsValue {
        match self.0.get_thinking_history_json(key) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn set_controllers(&self, json: &str) { self.0.set_controllers(json); }
    pub fn set_current_controller(&self, json: &str) { self.0.set_current_controller(json); }
    pub fn add_controller(&self, json: &str) { self.0.add_controller(json); }

    pub fn update_controller(&self, key: &str, json: &str) {
        self.0.update_controller(key, json);
    }

    pub fn remove_controller(&self, key: &str) { self.0.remove_controller(key); }

    pub fn set_iterations(&self, key: &str, json: &str) {
        self.0.set_iterations(key, json);
    }

    pub fn add_iteration(&self, key: &str, json: &str) {
        self.0.add_iteration(key, json);
    }

    pub fn update_thinking(&self, key: &str, json: &str) {
        self.0.update_thinking(key, json);
    }

    pub async fn fetch_controllers(&self) -> Result<String, String> {
        self.0.fetch_controllers().await
    }

    pub async fn fetch_controller(&self, key: &str) -> Result<String, String> {
        self.0.fetch_controller(key).await
    }

    pub async fn create_controller(&self, request_json: &str) -> Result<String, String> {
        self.0.create_controller(request_json).await
    }

    pub async fn pause_controller(&self, key: &str) -> Result<(), String> {
        self.0.pause_controller(key).await
    }

    pub async fn resume_controller(&self, key: &str) -> Result<(), String> {
        self.0.resume_controller(key).await
    }

    pub async fn stop_controller(&self, key: &str) -> Result<(), String> {
        self.0.stop_controller(key).await
    }

    pub async fn approve_controller(&self, key: &str, request_json: &str) -> Result<(), String> {
        self.0.approve_controller(key, request_json).await
    }

    pub async fn takeover_controller(&self, key: &str) -> Result<(), String> {
        self.0.takeover_controller(key).await
    }

    pub async fn handback_controller(&self, key: &str) -> Result<(), String> {
        self.0.handback_controller(key).await
    }

    pub async fn fetch_iterations(&self, key: &str) -> Result<String, String> {
        self.0.fetch_iterations(key).await
    }
}
