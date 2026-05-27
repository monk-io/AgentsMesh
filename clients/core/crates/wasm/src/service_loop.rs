use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::LoopService;
use agentsmesh_state::loop_state::LoopState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLoopService(pub(crate) LoopService);

#[wasm_bindgen]
impl WasmLoopService {
    pub(crate) fn new(client: Arc<ApiClient>, state: LoopState) -> Self {
        Self(LoopService::new(client, state))
    }

    pub fn loops_json(&self) -> String { self.0.loops_json() }

    pub fn current_loop_json(&self) -> JsValue {
        match self.0.current_loop_json() {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    pub fn runs_json(&self) -> String { self.0.runs_json() }

    pub fn get_loop_by_slug_json(&self, slug: &str) -> JsValue {
        match self.0.get_loop_by_slug_json(slug) {
            Some(s) => JsValue::from_str(&s),
            None => JsValue::NULL,
        }
    }

    // -------- Proto-state mutations (binary wire) --------

    pub fn replace_cached_loops(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.replace_cached_loops(bytes)
    }

    pub fn set_current_loop(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.set_current_loop(bytes)
    }

    pub fn clear_current_loop(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.clear_current_loop(bytes)
    }

    pub fn patch_loop_from_action(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.patch_loop_from_action(bytes)
    }

    pub fn insert_loop_run(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.insert_loop_run(bytes)
    }

    pub fn replace_cached_runs(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.replace_cached_runs(bytes)
    }

    pub fn append_cached_runs(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.append_cached_runs(bytes)
    }

    pub fn patch_loop_run_status(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.patch_loop_run_status(bytes)
    }

    pub fn clear_loop_runs(&self, bytes: &[u8]) -> Result<(), String> {
        self.0.clear_loop_runs(bytes)
    }

    // -------- Connect-RPC (binary wire) --------

    #[wasm_bindgen(js_name = listLoopsConnect)]
    pub async fn list_loops_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_loops_connect(request).await
    }

    #[wasm_bindgen(js_name = getLoopConnect)]
    pub async fn get_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_loop_connect(request).await
    }

    #[wasm_bindgen(js_name = createLoopConnect)]
    pub async fn create_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.create_loop_connect(request).await
    }

    #[wasm_bindgen(js_name = updateLoopConnect)]
    pub async fn update_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.update_loop_connect(request).await
    }

    #[wasm_bindgen(js_name = deleteLoopConnect)]
    pub async fn delete_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.delete_loop_connect(request).await
    }

    #[wasm_bindgen(js_name = enableLoopConnect)]
    pub async fn enable_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.loop_action_connect("enable", request).await
    }

    #[wasm_bindgen(js_name = disableLoopConnect)]
    pub async fn disable_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.loop_action_connect("disable", request).await
    }

    #[wasm_bindgen(js_name = triggerLoopConnect)]
    pub async fn trigger_loop_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.trigger_loop_connect(request).await
    }

    #[wasm_bindgen(js_name = listRunsConnect)]
    pub async fn list_runs_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_runs_connect(request).await
    }

    #[wasm_bindgen(js_name = cancelRunConnect)]
    pub async fn cancel_run_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.cancel_run_connect(request).await
    }
}
