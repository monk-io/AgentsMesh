use agentsmesh_state::runner_state::RunnerState;
use agentsmesh_types::proto_runner_state_v1::ApplyRunnerStatusEventRequest;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRunnerState {
    inner: RunnerState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

#[wasm_bindgen]
impl WasmRunnerState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: RunnerState::with_storage(crate::new_memory_backend()) }
    }

    pub fn runners_json(&self) -> String {
        serde_json::to_string(self.inner.runners()).unwrap_or_default()
    }

    pub fn available_runners_json(&self) -> String {
        serde_json::to_string(self.inner.available_runners())
            .unwrap_or_default()
    }

    pub fn current_runner_json(&self) -> JsValue {
        match self.inner.current_runner() {
            Some(r) => JsValue::from_str(
                &serde_json::to_string(r).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn apply_runner_status_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyRunnerStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_runner_status(req.runner_id, &req.status);
        Ok(())
    }
}
