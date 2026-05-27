use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::autopilot_state::AutopilotState;
use agentsmesh_state::autopilot_state::AutopilotController;
use agentsmesh_state::autopilot_state::AutopilotIteration;
use agentsmesh_types::proto_autopilot_state_v1::{
    AutopilotControllerSnapshot, AutopilotIterationSnapshot,
};

fn snapshot_to_controller(s: AutopilotControllerSnapshot) -> AutopilotController {
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

fn snapshot_to_iteration(s: AutopilotIterationSnapshot) -> AutopilotIteration {
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

pub struct AutopilotService {
    client: Arc<ApiClient>,
    state: RwLock<AutopilotState>,
}

impl AutopilotService {
    pub fn new(client: Arc<ApiClient>, state: AutopilotState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    pub fn controllers_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().controllers()).unwrap_or_default()
    }

    pub fn current_controller_json(&self) -> Option<String> {
        self.state.read().unwrap().current_controller()
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn get_controller_by_pod_key_json(&self, pod_key: &str) -> Option<String> {
        self.state.read().unwrap().get_controller_by_pod_key(pod_key)
            .map(|c| serde_json::to_string(c).unwrap_or_default())
    }

    pub fn get_iterations_json(&self, key: &str) -> Option<String> {
        self.state.read().unwrap().get_iterations(key)
            .map(|iters| serde_json::to_string(iters).unwrap_or_default())
    }

    pub fn get_thinking_json(&self, key: &str) -> Option<String> {
        self.state.read().unwrap().get_thinking(key)
            .map(|t| serde_json::to_string(t).unwrap_or_default())
    }

    pub fn get_thinking_history_json(&self, key: &str) -> Option<String> {
        self.state.read().unwrap().get_thinking_history(key)
            .map(|h| serde_json::to_string(h).unwrap_or_default())
    }

    pub fn replace_cached_controllers(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::ReplaceCachedControllersRequest;
        use prost::Message;
        let req = ReplaceCachedControllersRequest::decode(req_bytes)
            .map_err(|e| format!("decode replace_cached_controllers: {e}"))?;
        let controllers: Vec<AutopilotController> = req.controllers
            .into_iter()
            .map(snapshot_to_controller)
            .collect();
        self.state.write().unwrap().set_controllers(controllers);
        Ok(())
    }

    pub fn set_current_controller_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::SetCurrentControllerRequest;
        use prost::Message;
        let req = SetCurrentControllerRequest::decode(req_bytes)
            .map_err(|e| format!("decode set_current_controller: {e}"))?;
        self.state.write().unwrap().set_current_controller(req.controller.map(snapshot_to_controller));
        Ok(())
    }

    pub fn insert_controller(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::InsertControllerRequest;
        use prost::Message;
        let req = InsertControllerRequest::decode(req_bytes)
            .map_err(|e| format!("decode insert_controller: {e}"))?;
        if let Some(c) = req.controller {
            self.state.write().unwrap().add_controller(snapshot_to_controller(c));
        }
        Ok(())
    }

    pub fn patch_controller(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::PatchControllerRequest;
        use prost::Message;
        let req = PatchControllerRequest::decode(req_bytes)
            .map_err(|e| format!("decode patch_controller: {e}"))?;
        if let Some(c) = req.controller {
            self.state.write().unwrap().update_controller(&req.autopilot_controller_key, snapshot_to_controller(c));
        }
        Ok(())
    }

    pub fn remove_controller_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::RemoveControllerRequest;
        use prost::Message;
        let req = RemoveControllerRequest::decode(req_bytes)
            .map_err(|e| format!("decode remove_controller: {e}"))?;
        self.state.write().unwrap().remove_controller(&req.autopilot_controller_key);
        Ok(())
    }

    pub fn replace_cached_iterations(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::ReplaceCachedIterationsRequest;
        use prost::Message;
        let req = ReplaceCachedIterationsRequest::decode(req_bytes)
            .map_err(|e| format!("decode replace_cached_iterations: {e}"))?;
        let iters: Vec<AutopilotIteration> = req.iterations
            .into_iter()
            .map(snapshot_to_iteration)
            .collect();
        self.state.write().unwrap().set_iterations(req.autopilot_controller_key, iters);
        Ok(())
    }

    pub fn append_iteration(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::AppendIterationRequest;
        use prost::Message;
        let req = AppendIterationRequest::decode(req_bytes)
            .map_err(|e| format!("decode append_iteration: {e}"))?;
        if let Some(iter) = req.iteration {
            self.state.write().unwrap().add_iteration(req.autopilot_controller_key, snapshot_to_iteration(iter));
        }
        Ok(())
    }

    pub fn update_thinking_proto(&self, req_bytes: &[u8]) -> Result<(), String> {
        use agentsmesh_types::proto_autopilot_state_v1::UpdateThinkingRequest;
        use prost::Message;
        let req = UpdateThinkingRequest::decode(req_bytes)
            .map_err(|e| format!("decode update_thinking: {e}"))?;
        if let Ok(v) = serde_json::from_str(&req.thinking_json) {
            self.state.write().unwrap().update_thinking(req.autopilot_controller_key, v);
        }
        Ok(())
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Connect lanes — request bytes in, response bytes out. State is
    // bypassed (caller is the TS adapter / FFI consumer).

    pub async fn list_autopilots_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::ListAutopilotControllersRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_autopilots request: {e}"))?;
        let resp = self.client.list_autopilots_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_autopilot_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::GetAutopilotControllerRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_autopilot request: {e}"))?;
        let resp = self.client.get_autopilot_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_autopilot_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::CreateAutopilotControllerRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_autopilot request: {e}"))?;
        let resp = self.client.create_autopilot_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn action_autopilot_connect(
        &self, procedure: &str, request_bytes: &[u8],
    ) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::ActionRequest::decode(request_bytes)
            .map_err(|e| format!("decode action_autopilot request: {e}"))?;
        let resp = match procedure {
            "pause" => self.client.pause_autopilot_connect(&req).await,
            "resume" => self.client.resume_autopilot_connect(&req).await,
            "stop" => self.client.stop_autopilot_connect(&req).await,
            "takeover" => self.client.takeover_autopilot_connect(&req).await,
            "handback" => self.client.handback_autopilot_connect(&req).await,
            other => return Err(format!("unknown autopilot action: {other}")),
        }.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn approve_autopilot_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::ApproveRequest::decode(request_bytes)
            .map_err(|e| format!("decode approve_autopilot request: {e}"))?;
        let resp = self.client.approve_autopilot_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_iterations_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_autopilot_v1 as ap;
        use prost::Message;
        let req = ap::GetIterationsRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_iterations request: {e}"))?;
        let resp = self.client.get_autopilot_iterations_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
