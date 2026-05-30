use std::sync::Arc;

use agentsmesh_state::app_state::AppState;
use agentsmesh_types::proto_runner_state_v1::{
    ApplyRunnerStatusEventRequest, PatchCachedRunnerRequest, RemoveCachedRunnerRequest,
    ReplaceAvailableRunnersRequest, ReplaceCachedRunnersRequest, SetCurrentRunnerRequest,
};
use parking_lot::RwLock;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRunnerState {
    state: Arc<RwLock<AppState>>,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

impl WasmRunnerState {
    pub(crate) fn from_runtime(state: Arc<RwLock<AppState>>) -> Self {
        Self { state }
    }
}

#[wasm_bindgen]
impl WasmRunnerState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::with_storage(crate::new_memory_backend()))),
        }
    }

    pub fn runners_json(&self) -> String {
        serde_json::to_string(self.state.read().runners.runners()).unwrap_or_default()
    }

    pub fn available_runners_json(&self) -> String {
        serde_json::to_string(self.state.read().runners.available_runners())
            .unwrap_or_default()
    }

    pub fn current_runner_json(&self) -> JsValue {
        match self.state.read().runners.current_runner() {
            Some(r) => JsValue::from_str(
                &serde_json::to_string(r).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn apply_runner_status_event(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyRunnerStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().runners.update_runner_status(req.runner_id, &req.status);
        Ok(())
    }

    pub fn replace_cached_runners(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedRunnersRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().runners.set_runners(req.runners);
        Ok(())
    }

    pub fn replace_available_runners(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceAvailableRunnersRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().runners.set_available_runners(req.runners);
        Ok(())
    }

    pub fn set_current_runner_proto(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentRunnerRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().runners.set_current_runner(req.runner);
        Ok(())
    }

    pub fn patch_cached_runner(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchCachedRunnerRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(runner) = req.runner {
            self.state.write().runners.upsert_runner(runner);
        }
        Ok(())
    }

    pub fn remove_cached_runner(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = RemoveCachedRunnerRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().runners.remove_runner(req.runner_id);
        Ok(())
    }
}
