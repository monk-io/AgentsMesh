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

    pub fn replace_cached_controllers(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.replace_cached_controllers(req_bytes)
    }

    pub fn set_current_controller_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.set_current_controller_proto(req_bytes)
    }

    pub fn insert_controller(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.insert_controller(req_bytes)
    }

    pub fn patch_controller(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.patch_controller(req_bytes)
    }

    pub fn remove_controller_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.remove_controller_proto(req_bytes)
    }

    pub fn replace_cached_iterations(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.replace_cached_iterations(req_bytes)
    }

    pub fn append_iteration(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.append_iteration(req_bytes)
    }

    pub fn update_thinking_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        self.0.update_thinking_proto(req_bytes)
    }

    // -------- Connect-RPC (binary wire) --------

    #[wasm_bindgen(js_name = listAutopilotsConnect)]
    pub async fn list_autopilots_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_autopilots_connect(request).await
    }

    #[wasm_bindgen(js_name = getAutopilotConnect)]
    pub async fn get_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_autopilot_connect(request).await
    }

    #[wasm_bindgen(js_name = createAutopilotConnect)]
    pub async fn create_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.create_autopilot_connect(request).await
    }

    #[wasm_bindgen(js_name = pauseAutopilotConnect)]
    pub async fn pause_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.action_autopilot_connect("pause", request).await
    }

    #[wasm_bindgen(js_name = resumeAutopilotConnect)]
    pub async fn resume_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.action_autopilot_connect("resume", request).await
    }

    #[wasm_bindgen(js_name = stopAutopilotConnect)]
    pub async fn stop_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.action_autopilot_connect("stop", request).await
    }

    #[wasm_bindgen(js_name = takeoverAutopilotConnect)]
    pub async fn takeover_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.action_autopilot_connect("takeover", request).await
    }

    #[wasm_bindgen(js_name = handbackAutopilotConnect)]
    pub async fn handback_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.action_autopilot_connect("handback", request).await
    }

    #[wasm_bindgen(js_name = approveAutopilotConnect)]
    pub async fn approve_autopilot_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.approve_autopilot_connect(request).await
    }

    #[wasm_bindgen(js_name = getIterationsConnect)]
    pub async fn get_iterations_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_iterations_connect(request).await
    }
}
