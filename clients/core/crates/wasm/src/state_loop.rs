
use agentsmesh_state::loop_state::{LoopData, LoopRunData, LoopState};
use agentsmesh_types::proto_loop_v1::{Loop as ProtoLoop, LoopRun as ProtoLoopRun};
use agentsmesh_types::proto_loop_state_v1::{
    AppendCachedRunsRequest, ClearCurrentLoopRequest, ClearLoopRunsRequest,
    InsertLoopRunRequest, PatchLoopFromActionRequest, PatchLoopRunStatusRequest,
    ReplaceCachedLoopsRequest, ReplaceCachedRunsRequest, SetCurrentLoopRequest,
};
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLoopState {
    inner: LoopState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
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

#[wasm_bindgen]
impl WasmLoopState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: LoopState::with_storage(crate::new_memory_backend()) }
    }

    pub fn loops_json(&self) -> String {
        serde_json::to_string(self.inner.get_loops()).unwrap_or_default()
    }

    pub fn current_loop_json(&self) -> JsValue {
        match self.inner.get_current_loop() {
            Some(l) => JsValue::from_str(
                &serde_json::to_string(l).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn runs_json(&self) -> String {
        serde_json::to_string(self.inner.get_runs()).unwrap_or_default()
    }

    pub fn get_loop_by_slug_json(&self, slug: &str) -> JsValue {
        match self.inner.get_loop_by_slug(slug) {
            Some(l) => JsValue::from_str(
                &serde_json::to_string(l).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_loops(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedLoopsRequest::decode(req_bytes).map_err(decode_err)?;
        let loops = req.loops.into_iter().map(loop_from_proto).collect();
        self.inner.set_loops(loops);
        Ok(())
    }

    pub fn set_current_loop(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentLoopRequest::decode(req_bytes).map_err(decode_err)?;
        let loop_data = req.r#loop.map(loop_from_proto);
        self.inner.set_current_loop(loop_data);
        Ok(())
    }

    pub fn clear_current_loop(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let _ = ClearCurrentLoopRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_current_loop(None);
        Ok(())
    }

    pub fn patch_loop_from_action(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchLoopFromActionRequest::decode(req_bytes).map_err(decode_err)?;
        let loop_data = req.r#loop.ok_or_else(|| JsValue::from_str("missing loop"))?;
        self.inner.update_loop(&req.slug, loop_from_proto(loop_data));
        Ok(())
    }

    pub fn insert_loop_run(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertLoopRunRequest::decode(req_bytes).map_err(decode_err)?;
        let run = req.run.ok_or_else(|| JsValue::from_str("missing run"))?;
        self.inner.add_run(run_from_proto(run));
        Ok(())
    }

    pub fn replace_cached_runs(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedRunsRequest::decode(req_bytes).map_err(decode_err)?;
        let runs = req.runs.into_iter().map(run_from_proto).collect();
        self.inner.set_runs(runs);
        Ok(())
    }

    pub fn append_cached_runs(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = AppendCachedRunsRequest::decode(req_bytes).map_err(decode_err)?;
        let runs: Vec<LoopRunData> = req.runs.into_iter().map(run_from_proto).collect();
        self.inner.append_runs(runs);
        Ok(())
    }

    pub fn patch_loop_run_status(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchLoopRunStatusRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_run_status(req.run_id, &req.status);
        Ok(())
    }

    pub fn clear_loop_runs(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let _ = ClearLoopRunsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.clear_runs();
        Ok(())
    }
}
