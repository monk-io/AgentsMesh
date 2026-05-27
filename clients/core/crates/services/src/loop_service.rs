use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::loop_state::{LoopData, LoopRunData, LoopState};
use agentsmesh_types::proto_loop_v1::{Loop as ProtoLoop, LoopRun as ProtoLoopRun};
use agentsmesh_types::proto_loop_state_v1 as ls;
use prost::Message;

pub struct LoopService {
    client: Arc<ApiClient>,
    state: RwLock<LoopState>,
}

fn loop_from_proto(p: ProtoLoop) -> LoopData {
    LoopData {
        id: p.id,
        slug: p.slug,
        name: p.name,
        description: p.description,
        schedule: None,
        is_enabled: false,
        status: Some(p.status),
        agent_slug: Some(p.agent_slug),
        permission_mode: Some(p.permission_mode),
        prompt_template: Some(p.prompt_template),
        config_overrides: serde_json::from_str(&p.config_overrides_json).ok(),
        prompt_variables: serde_json::from_str(&p.prompt_variables_json).ok(),
        execution_mode: Some(p.execution_mode),
        autopilot_config: serde_json::from_str(&p.autopilot_config_json).ok(),
        sandbox_strategy: Some(p.sandbox_strategy),
        session_persistence: Some(p.session_persistence),
        concurrency_policy: Some(p.concurrency_policy),
        max_concurrent_runs: Some(p.max_concurrent_runs),
        max_retained_runs: Some(p.max_retained_runs),
        timeout_minutes: Some(p.timeout_minutes),
        idle_timeout_sec: Some(p.idle_timeout_sec),
        total_runs: Some(p.total_runs),
        successful_runs: Some(p.successful_runs),
        failed_runs: Some(p.failed_runs),
        active_run_count: Some(p.active_run_count),
        last_run_at: p.last_run_at,
        created_at: Some(p.created_at),
        updated_at: Some(p.updated_at),
        used_env_bundles: p.used_env_bundles,
    }
}

fn run_from_proto(p: ProtoLoopRun) -> LoopRunData {
    LoopRunData {
        id: p.id,
        loop_slug: String::new(),
        run_number: Some(p.run_number),
        status: p.status,
        pod_key: p.pod_key,
        started_at: p.started_at,
        completed_at: p.completed_at,
        error_message: p.error_message,
        created_at: Some(p.created_at),
    }
}

impl LoopService {
    pub fn new(client: Arc<ApiClient>, state: LoopState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    pub fn loops_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_loops()).unwrap_or_default()
    }

    pub fn current_loop_json(&self) -> Option<String> {
        self.state.read().unwrap().get_current_loop()
            .map(|l| serde_json::to_string(l).unwrap_or_default())
    }

    pub fn runs_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_runs()).unwrap_or_default()
    }

    pub fn get_loop_by_slug_json(&self, slug: &str) -> Option<String> {
        self.state.read().unwrap().get_loop_by_slug(slug)
            .map(|l| serde_json::to_string(l).unwrap_or_default())
    }

    // -------- Legacy JSON mutations (NAPI backward-compat only) --------
    //
    // The wasm bridge no longer exposes these — TS renderer code goes
    // through the proto-bytes methods below. Kept here because the
    // node-bridge desktop layer still wires them up directly. NAPI
    // proto-ization is Phase 5 of the proto-state migration.

    pub fn set_loops(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<LoopData>>(json) {
            self.state.write().unwrap().set_loops(v);
        }
    }

    pub fn set_current_loop_json(&self, json: &str) {
        let l = if json.is_empty() { None } else { serde_json::from_str::<LoopData>(json).ok() };
        self.state.write().unwrap().set_current_loop(l);
    }

    pub fn update_loop_local(&self, slug: &str, json: &str) {
        if let Ok(l) = serde_json::from_str::<LoopData>(json) {
            self.state.write().unwrap().update_loop(slug, l);
        }
    }

    pub fn add_run(&self, json: &str) {
        if let Ok(r) = serde_json::from_str::<LoopRunData>(json) {
            self.state.write().unwrap().add_run(r);
        }
    }

    pub fn set_runs(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<LoopRunData>>(json) {
            self.state.write().unwrap().set_runs(v);
        }
    }

    pub fn append_runs(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<LoopRunData>>(json) {
            self.state.write().unwrap().append_runs(v);
        }
    }

    pub fn update_run_status(&self, run_id: i64, status: &str) {
        self.state.write().unwrap().update_run_status(run_id, status);
    }

    pub fn clear_runs(&self) {
        self.state.write().unwrap().clear_runs();
    }

    // -------- Proto-state mutations (binary wire) --------

    pub fn replace_cached_loops(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::ReplaceCachedLoopsRequest::decode(bytes)
            .map_err(|e| format!("decode replace_cached_loops: {e}"))?;
        let loops = req.loops.into_iter().map(loop_from_proto).collect();
        self.state.write().unwrap().set_loops(loops);
        Ok(())
    }

    pub fn set_current_loop(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::SetCurrentLoopRequest::decode(bytes)
            .map_err(|e| format!("decode set_current_loop: {e}"))?;
        let loop_data = req.r#loop.map(loop_from_proto);
        self.state.write().unwrap().set_current_loop(loop_data);
        Ok(())
    }

    pub fn clear_current_loop(&self, bytes: &[u8]) -> Result<(), String> {
        let _ = ls::ClearCurrentLoopRequest::decode(bytes)
            .map_err(|e| format!("decode clear_current_loop: {e}"))?;
        self.state.write().unwrap().set_current_loop(None);
        Ok(())
    }

    pub fn patch_loop_from_action(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::PatchLoopFromActionRequest::decode(bytes)
            .map_err(|e| format!("decode patch_loop_from_action: {e}"))?;
        let loop_data = req.r#loop.ok_or_else(|| "missing loop".to_string())?;
        self.state.write().unwrap().update_loop(&req.slug, loop_from_proto(loop_data));
        Ok(())
    }

    pub fn insert_loop_run(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::InsertLoopRunRequest::decode(bytes)
            .map_err(|e| format!("decode insert_loop_run: {e}"))?;
        let run = req.run.ok_or_else(|| "missing run".to_string())?;
        self.state.write().unwrap().add_run(run_from_proto(run));
        Ok(())
    }

    pub fn replace_cached_runs(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::ReplaceCachedRunsRequest::decode(bytes)
            .map_err(|e| format!("decode replace_cached_runs: {e}"))?;
        let runs = req.runs.into_iter().map(run_from_proto).collect();
        self.state.write().unwrap().set_runs(runs);
        Ok(())
    }

    pub fn append_cached_runs(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::AppendCachedRunsRequest::decode(bytes)
            .map_err(|e| format!("decode append_cached_runs: {e}"))?;
        let runs: Vec<LoopRunData> = req.runs.into_iter().map(run_from_proto).collect();
        self.state.write().unwrap().append_runs(runs);
        Ok(())
    }

    pub fn patch_loop_run_status(&self, bytes: &[u8]) -> Result<(), String> {
        let req = ls::PatchLoopRunStatusRequest::decode(bytes)
            .map_err(|e| format!("decode patch_loop_run_status: {e}"))?;
        self.state.write().unwrap().update_run_status(req.run_id, &req.status);
        Ok(())
    }

    pub fn clear_loop_runs(&self, bytes: &[u8]) -> Result<(), String> {
        let _ = ls::ClearLoopRunsRequest::decode(bytes)
            .map_err(|e| format!("decode clear_loop_runs: {e}"))?;
        self.state.write().unwrap().clear_runs();
        Ok(())
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // 10 Connect lanes — request bytes in, response bytes out. State is
    // bypassed (caller is the TS adapter); the existing REST methods
    // above keep updating the LoopState during the dual-track migration
    // so realtime event handlers stay correct.

    pub async fn list_loops_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::ListLoopsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_loops request: {e}"))?;
        let resp = self.client.list_loops_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_loop_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::GetLoopRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_loop request: {e}"))?;
        let resp = self.client.get_loop_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_loop_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::CreateLoopRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_loop request: {e}"))?;
        let resp = self.client.create_loop_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_loop_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::UpdateLoopRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_loop request: {e}"))?;
        let resp = self.client.update_loop_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_loop_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::DeleteLoopRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_loop request: {e}"))?;
        let resp = self.client.delete_loop_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn loop_action_connect(
        &self, action: &str, request_bytes: &[u8],
    ) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::LoopActionRequest::decode(request_bytes)
            .map_err(|e| format!("decode loop_action request: {e}"))?;
        let resp = match action {
            "enable" => self.client.enable_loop_connect(&req).await,
            "disable" => self.client.disable_loop_connect(&req).await,
            other => return Err(format!("unknown loop action: {other}")),
        }.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn trigger_loop_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::TriggerLoopRequest::decode(request_bytes)
            .map_err(|e| format!("decode trigger_loop request: {e}"))?;
        let resp = self.client.trigger_loop_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_runs_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::ListRunsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_runs request: {e}"))?;
        let resp = self.client.list_loop_runs_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn cancel_run_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        use agentsmesh_types::proto_loop_v1 as lp;
        let req = lp::CancelRunRequest::decode(request_bytes)
            .map_err(|e| format!("decode cancel_run request: {e}"))?;
        let resp = self.client.cancel_loop_run_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
