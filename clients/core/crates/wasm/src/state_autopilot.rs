use agentsmesh_state::autopilot_state::{AutopilotController, AutopilotIteration, AutopilotState};
use agentsmesh_types::proto_autopilot_state_v1::{
    AppendIterationRequest, AutopilotControllerSnapshot, AutopilotIterationSnapshot,
    InsertControllerRequest, PatchControllerRequest, RemoveControllerRequest,
    ReplaceCachedControllersRequest, ReplaceCachedIterationsRequest,
    SetCurrentControllerRequest, UpdateThinkingRequest,
};
use prost::Message;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmAutopilotState {
    inner: AutopilotState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

fn from_snapshot(s: AutopilotControllerSnapshot) -> AutopilotController {
    AutopilotController {
        autopilot_controller_key: s.autopilot_controller_key,
        pod_key: s.pod_key,
        status: s.status,
        phase: s.phase,
        prompt: s.prompt,
        max_iterations: s.max_iterations,
        iteration_timeout_sec: s.iteration_timeout_sec,
        no_progress_threshold: s.no_progress_threshold,
        same_error_threshold: s.same_error_threshold,
        approval_timeout_min: s.approval_timeout_min,
        current_iteration: s.current_iteration,
        control_agent_slug: s.control_agent_slug,
        circuit_breaker_state: s.circuit_breaker_state,
        circuit_breaker_reason: s.circuit_breaker_reason,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}

fn from_iteration_snapshot(s: AutopilotIterationSnapshot) -> AutopilotIteration {
    AutopilotIteration {
        id: s.id,
        controller_key: s.controller_key,
        iteration_number: s.iteration_number,
        status: s.status,
        result: s.result,
        started_at: s.started_at,
        completed_at: s.completed_at,
    }
}

#[wasm_bindgen]
impl WasmAutopilotState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: AutopilotState::new() }
    }

    pub fn controllers_json(&self) -> String {
        serde_json::to_string(self.inner.controllers()).unwrap_or_default()
    }

    pub fn current_controller_json(&self) -> JsValue {
        match self.inner.current_controller() {
            Some(c) => JsValue::from_str(&serde_json::to_string(c).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_controllers(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedControllersRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_controllers(req.controllers.into_iter().map(from_snapshot).collect());
        Ok(())
    }

    pub fn set_current_controller_proto(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentControllerRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_current_controller(req.controller.map(from_snapshot));
        Ok(())
    }

    pub fn insert_controller(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertControllerRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(c) = req.controller {
            self.inner.add_controller(from_snapshot(c));
        }
        Ok(())
    }

    pub fn patch_controller(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchControllerRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(c) = req.controller {
            self.inner.update_controller(&req.autopilot_controller_key, from_snapshot(c));
        }
        Ok(())
    }

    pub fn remove_controller(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = RemoveControllerRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.remove_controller(&req.autopilot_controller_key);
        Ok(())
    }

    pub fn get_iterations_json(&self, key: &str) -> JsValue {
        match self.inner.get_iterations(key) {
            Some(iters) => JsValue::from_str(&serde_json::to_string(iters).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_iterations(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedIterationsRequest::decode(req_bytes).map_err(decode_err)?;
        let iters = req.iterations.into_iter().map(from_iteration_snapshot).collect();
        self.inner.set_iterations(req.autopilot_controller_key, iters);
        Ok(())
    }

    pub fn append_iteration(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = AppendIterationRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(iter) = req.iteration {
            self.inner.add_iteration(req.autopilot_controller_key, from_iteration_snapshot(iter));
        }
        Ok(())
    }

    pub fn get_thinking_json(&self, key: &str) -> JsValue {
        match self.inner.get_thinking(key) {
            Some(t) => JsValue::from_str(&serde_json::to_string(t).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn get_thinking_history_json(&self, key: &str) -> JsValue {
        match self.inner.get_thinking_history(key) {
            Some(h) => JsValue::from_str(&serde_json::to_string(h).unwrap_or_default()),
            None => JsValue::from_str("[]"),
        }
    }

    pub fn update_thinking(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = UpdateThinkingRequest::decode(req_bytes).map_err(decode_err)?;
        if let Ok(thinking) = serde_json::from_str::<Value>(&req.thinking_json) {
            self.inner.update_thinking(req.autopilot_controller_key, thinking);
        }
        Ok(())
    }

    pub fn get_controller_by_pod_key_json(&self, pod_key: &str) -> JsValue {
        match self.inner.get_controller_by_pod_key(pod_key) {
            Some(c) => JsValue::from_str(&serde_json::to_string(c).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }
}
