use napi_derive::napi;

use agentsmesh_types::proto_runner_state_v1::{
    PatchCachedRunnerRequest, RemoveCachedRunnerRequest, ReplaceAvailableRunnersRequest,
    ReplaceCachedRunnersRequest, SetCurrentRunnerRequest,
};
use prost::Message as _;

use crate::AppState;

// Runner state surface over the shared `runtime.state` (dispatch-hook SSOT),
// mirroring app_channel.rs / app_pod.rs. Keeps `runtime.state.runners` fed by
// both fetch baseline and EventBus dispatch so the post-dispatch snapshot
// (main/realtime.ts) reflects the full runner picture for the renderer mirror.
fn decode_err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(format!("decode: {e}"))
}

#[napi]
impl AppState {
    // ── Snapshot reads ──

    #[napi]
    pub fn app_runners_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().runners.runners()).unwrap_or_default()
    }

    #[napi]
    pub fn app_available_runners_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().runners.available_runners())
            .unwrap_or_default()
    }

    #[napi]
    pub fn app_current_runner_json(&self) -> String {
        match self.runtime.state.read().runners.current_runner() {
            Some(r) => serde_json::to_string(r).unwrap_or_default(),
            None => String::new(),
        }
    }

    // ── Fetch-mirror mutators → runtime.state baseline ──

    #[napi]
    pub fn app_runner_replace_cached(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceCachedRunnersRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().runners.set_runners(req.runners);
        Ok(())
    }

    #[napi]
    pub fn app_runner_replace_available(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceAvailableRunnersRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().runners.set_available_runners(req.runners);
        Ok(())
    }

    #[napi]
    pub fn app_runner_set_current(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = SetCurrentRunnerRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().runners.set_current_runner(req.runner);
        Ok(())
    }

    #[napi]
    pub fn app_runner_patch(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = PatchCachedRunnerRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        if let Some(runner) = req.runner {
            self.runtime.state.write().runners.upsert_runner(runner);
        }
        Ok(())
    }

    #[napi]
    pub fn app_runner_remove(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = RemoveCachedRunnerRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().runners.remove_runner(req.runner_id);
        Ok(())
    }
}
