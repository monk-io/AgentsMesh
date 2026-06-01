use napi_derive::napi;

use agentsmesh_state::loop_state::{LoopData, LoopRunData};
use agentsmesh_types::proto_loop_v1::{Loop as ProtoLoop, LoopRun as ProtoLoopRun};
use agentsmesh_types::proto_loop_state_v1::{
    AppendCachedRunsRequest, ClearCurrentLoopRequest, ClearLoopRunsRequest, InsertLoopRunRequest,
    PatchLoopFromActionRequest, PatchLoopRunStatusRequest, ReplaceCachedLoopsRequest,
    ReplaceCachedRunsRequest, SetCurrentLoopRequest,
};
use prost::Message as _;

use crate::AppState;

// Loop state surface over the shared `runtime.state` (dispatch-hook SSOT),
// mirroring app_autopilot.rs. The LoopRun* dispatch arms (event_dispatch.rs)
// already write `runtime.state.loops`; these fetch-mirror mutators keep that
// same store fed from the TS adapter so reads never diverge from realtime.
fn decode_err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(format!("decode: {e}"))
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

#[napi]
impl AppState {
    #[napi]
    pub fn app_loop_replace_cached_loops(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceCachedLoopsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        let loops = req.loops.into_iter().map(loop_from_proto).collect();
        self.runtime.state.write().loops.set_loops(loops);
        Ok(())
    }

    #[napi]
    pub fn app_loop_set_current_loop(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = SetCurrentLoopRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().loops.set_current_loop(req.r#loop.map(loop_from_proto));
        Ok(())
    }

    #[napi]
    pub fn app_loop_clear_current_loop(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        ClearCurrentLoopRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().loops.set_current_loop(None);
        Ok(())
    }

    #[napi]
    pub fn app_loop_patch_loop_from_action(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = PatchLoopFromActionRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        if let Some(l) = req.r#loop {
            self.runtime.state.write().loops.update_loop(&req.slug, loop_from_proto(l));
        }
        Ok(())
    }

    #[napi]
    pub fn app_loop_insert_loop_run(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = InsertLoopRunRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        if let Some(run) = req.run {
            self.runtime.state.write().loops.add_run(run_from_proto(run));
        }
        Ok(())
    }

    #[napi]
    pub fn app_loop_replace_cached_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceCachedRunsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        let runs = req.runs.into_iter().map(run_from_proto).collect();
        self.runtime.state.write().loops.set_runs(runs);
        Ok(())
    }

    #[napi]
    pub fn app_loop_append_cached_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = AppendCachedRunsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        let runs: Vec<LoopRunData> = req.runs.into_iter().map(run_from_proto).collect();
        self.runtime.state.write().loops.append_runs(runs);
        Ok(())
    }

    #[napi]
    pub fn app_loop_patch_loop_run_status(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = PatchLoopRunStatusRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().loops.update_run_status(req.run_id, &req.status);
        Ok(())
    }

    #[napi]
    pub fn app_loop_clear_loop_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        ClearLoopRunsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().loops.clear_runs();
        Ok(())
    }
}
