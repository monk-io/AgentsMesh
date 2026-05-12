use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::autopilot_state::AutopilotState;
use agentsmesh_types::{
    AutopilotController, AutopilotIteration,
    CreateAutopilotRequest, ApproveAutopilotRequest,
};

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

    pub fn set_controllers(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<AutopilotController>>(json) {
            self.state.write().unwrap().set_controllers(v);
        }
    }

    pub fn set_current_controller(&self, json: &str) {
        let c = if json.is_empty() { None } else { serde_json::from_str::<AutopilotController>(json).ok() };
        self.state.write().unwrap().set_current_controller(c);
    }

    pub fn add_controller(&self, json: &str) {
        if let Ok(c) = serde_json::from_str::<AutopilotController>(json) {
            self.state.write().unwrap().add_controller(c);
        }
    }

    pub fn update_controller(&self, key: &str, json: &str) {
        if let Ok(c) = serde_json::from_str::<AutopilotController>(json) {
            self.state.write().unwrap().update_controller(key, c);
        }
    }

    pub fn remove_controller(&self, key: &str) {
        self.state.write().unwrap().remove_controller(key);
    }

    pub fn set_iterations(&self, key: &str, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<AutopilotIteration>>(json) {
            self.state.write().unwrap().set_iterations(key.to_string(), v);
        }
    }

    pub fn add_iteration(&self, key: &str, json: &str) {
        if let Ok(i) = serde_json::from_str::<AutopilotIteration>(json) {
            self.state.write().unwrap().add_iteration(key.to_string(), i);
        }
    }

    pub fn update_thinking(&self, key: &str, json: &str) {
        if let Ok(v) = serde_json::from_str(json) {
            self.state.write().unwrap().update_thinking(key.to_string(), v);
        }
    }

    pub async fn fetch_controllers(&self) -> Result<String, String> {
        let resp = self.client.list_autopilots().await.map_err(crate::wire)?;
        self.state.write().unwrap().set_controllers(resp.controllers.clone());
        serde_json::to_string(&resp.controllers).map_err(crate::wire)
    }

    pub async fn fetch_controller(&self, key: &str) -> Result<String, String> {
        let c: AutopilotController = self.client
            .get_autopilot(key)
            .await.map_err(crate::wire)?;
        let mut s = self.state.write().unwrap();
        s.add_controller(c.clone());
        s.set_current_controller(Some(c.clone()));
        drop(s);
        serde_json::to_string(&c).map_err(crate::wire)
    }

    pub async fn create_controller(&self, request_json: &str) -> Result<String, String> {
        let req: CreateAutopilotRequest = serde_json::from_str(request_json)
            .map_err(crate::wire)?;
        let c: AutopilotController = self.client
            .create_autopilot(&req)
            .await.map_err(crate::wire)?;
        let mut s = self.state.write().unwrap();
        s.add_controller(c.clone());
        s.set_current_controller(Some(c.clone()));
        drop(s);
        serde_json::to_string(&c).map_err(crate::wire)
    }

    pub async fn pause_controller(&self, key: &str) -> Result<(), String> {
        self.client.pause_autopilot(key).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn resume_controller(&self, key: &str) -> Result<(), String> {
        self.client.resume_autopilot(key).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn stop_controller(&self, key: &str) -> Result<(), String> {
        self.client.stop_autopilot(key).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn approve_controller(&self, key: &str, request_json: &str) -> Result<(), String> {
        let req: ApproveAutopilotRequest = serde_json::from_str(request_json)
            .map_err(crate::wire)?;
        self.client.approve_autopilot(key, &req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn takeover_controller(&self, key: &str) -> Result<(), String> {
        self.client.takeover_autopilot(key).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn handback_controller(&self, key: &str) -> Result<(), String> {
        self.client.handback_autopilot(key).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn fetch_iterations(&self, key: &str) -> Result<String, String> {
        let iterations = self.client
            .get_autopilot_iterations(key)
            .await.map_err(crate::wire)?;
        self.state.write().unwrap().set_iterations(key.to_string(), iterations.clone());
        serde_json::to_string(&iterations).map_err(crate::wire)
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // 10 Connect lanes — request bytes in, response bytes out. State is
    // bypassed (caller is the TS adapter); the existing REST methods
    // above keep updating the AutopilotState during the dual-track
    // migration so realtime event handlers stay correct.

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
