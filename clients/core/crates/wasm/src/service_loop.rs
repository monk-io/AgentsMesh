use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::LoopService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLoopService(pub(crate) LoopService);

#[wasm_bindgen]
impl WasmLoopService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self(LoopService::new(client))
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
