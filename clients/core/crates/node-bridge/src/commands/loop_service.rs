use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    // ── Read-side JSON getters (selectors) ──

    #[napi]
    pub async fn loop_svc_loops_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
        Ok(svc.loops_json())
    }

    #[napi]
    pub async fn loop_svc_current_loop_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
        Ok(svc.current_loop_json().unwrap_or_default())
    }

    #[napi]
    pub async fn loop_svc_runs_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
        Ok(svc.runs_json())
    }

    #[napi]
    pub async fn loop_svc_get_loop_by_slug_json(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
        Ok(svc.get_loop_by_slug_json(&slug).unwrap_or_default())
    }

    // ── Proto-bytes mutators (mirror WasmLoopService) ──

    #[napi]
    pub async fn loop_svc_replace_cached_loops(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.replace_cached_loops(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_set_current_loop(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.set_current_loop(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_clear_current_loop(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.clear_current_loop(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_patch_loop_from_action(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.patch_loop_from_action(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_insert_loop_run(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.insert_loop_run(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_replace_cached_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.replace_cached_runs(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_append_cached_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.append_cached_runs(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_patch_loop_run_status(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.patch_loop_run_status(&req_bytes).map_err(err)
    }

    #[napi]
    pub async fn loop_svc_clear_loop_runs(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
        svc.clear_loop_runs(&req_bytes).map_err(err)
    }
}
