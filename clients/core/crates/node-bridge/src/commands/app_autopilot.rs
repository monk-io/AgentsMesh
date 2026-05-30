use napi_derive::napi;

use agentsmesh_state::autopilot_state::{AutopilotController, AutopilotIteration};
use agentsmesh_types::proto_autopilot_state_v1::{
    AutopilotControllerSnapshot, AutopilotIterationSnapshot, InsertControllerRequest,
    ReplaceCachedControllersRequest, ReplaceCachedIterationsRequest,
};
use prost::Message as _;

use crate::AppState;

// Autopilot state surface over the shared `runtime.state` (dispatch-hook SSOT),
// mirroring app_channel.rs / app_pod.rs / app_runner.rs. Keeps
// `runtime.state.autopilot` fed by fetch baseline so the post-dispatch snapshot
// (main/realtime.ts) carries the full controller + iterations + thinking the
// renderer mirror needs.
fn decode_err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(format!("decode: {e}"))
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

#[napi]
impl AppState {
    // ── Snapshot reads ──

    #[napi]
    pub fn app_autopilot_controllers_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().autopilot.controllers()).unwrap_or_default()
    }

    #[napi]
    pub fn app_autopilot_iterations_json(&self, key: String) -> String {
        match self.runtime.state.read().autopilot.get_iterations(&key) {
            Some(iters) => serde_json::to_string(iters).unwrap_or_default(),
            None => String::new(),
        }
    }

    #[napi]
    pub fn app_autopilot_thinking_json(&self, key: String) -> String {
        match self.runtime.state.read().autopilot.get_thinking(&key) {
            Some(t) => serde_json::to_string(t).unwrap_or_default(),
            None => String::new(),
        }
    }

    #[napi]
    pub fn app_autopilot_thinking_history_json(&self, key: String) -> String {
        match self.runtime.state.read().autopilot.get_thinking_history(&key) {
            Some(h) => serde_json::to_string(h).unwrap_or_else(|_| "[]".to_string()),
            None => "[]".to_string(),
        }
    }

    // ── Fetch-mirror mutators → runtime.state baseline ──

    #[napi]
    pub fn app_autopilot_replace_cached_controllers(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceCachedControllersRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime
            .state
            .write()
            .autopilot
            .set_controllers(req.controllers.into_iter().map(from_snapshot).collect());
        Ok(())
    }

    #[napi]
    pub fn app_autopilot_insert_controller(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = InsertControllerRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        if let Some(c) = req.controller {
            self.runtime.state.write().autopilot.add_controller(from_snapshot(c));
        }
        Ok(())
    }

    #[napi]
    pub fn app_autopilot_replace_cached_iterations(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceCachedIterationsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        let iters = req.iterations.into_iter().map(from_iteration_snapshot).collect();
        self.runtime
            .state
            .write()
            .autopilot
            .set_iterations(req.autopilot_controller_key, iters);
        Ok(())
    }
}
