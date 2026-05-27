use agentsmesh_state::pod_state::PodState;
use agentsmesh_types::proto_pod_v1::Pod;
use agentsmesh_types::proto_pod_state_v1::{
    InsertCreatedPodRequest, PatchPodPerpetualRequest,
    ApplyPodStatusEventRequest, ApplyPodTitleEventRequest,
    ApplyPodAliasEventRequest, ApplyAgentStatusEventRequest,
    ReplaceCachedPodsRequest, AppendCachedPodsRequest,
    MarkPodTerminatedRequest,
};
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPodState {
    inner: PodState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

#[wasm_bindgen]
impl WasmPodState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: PodState::with_storage(crate::new_memory_backend()) }
    }

    pub fn pods_json(&self) -> String {
        serde_json::to_string(self.inner.pods()).unwrap_or_default()
    }

    pub fn current_pod_json(&self) -> JsValue {
        match self.inner.current_pod() {
            Some(pod) => JsValue::from_str(
                &serde_json::to_string(pod).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn get_pod_json(&self, pod_key: &str) -> JsValue {
        match self.inner.get_pod(pod_key) {
            Some(pod) => JsValue::from_str(
                &serde_json::to_string(pod).unwrap_or_default(),
            ),
            None => JsValue::NULL,
        }
    }

    pub fn insert_created_pod(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertCreatedPodRequest::decode(req_bytes).map_err(decode_err)?;
        let pod = req.pod.ok_or_else(|| JsValue::from_str("missing pod"))?;
        let ts = if req.client_timestamp_ms == 0 { None } else { Some(req.client_timestamp_ms) };
        self.inner.upsert_pod(pod, ts);
        Ok(())
    }

    pub fn patch_pod_perpetual(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchPodPerpetualRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.patch_perpetual(&req.pod_key, req.perpetual);
        Ok(())
    }

    pub fn apply_pod_status_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyPodStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_pod_status(
            &req.pod_key,
            &req.status,
            req.agent_status.as_deref(),
            req.error_code.as_deref(),
            req.error_message.as_deref(),
            None,
        );
        Ok(())
    }

    pub fn apply_pod_title_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyPodTitleEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_pod_title(&req.pod_key, &req.title, None);
        Ok(())
    }

    pub fn apply_pod_alias_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyPodAliasEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_pod_alias(&req.pod_key, req.alias.as_deref().unwrap_or(""));
        Ok(())
    }

    pub fn apply_agent_status_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyAgentStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_agent_status(&req.pod_key, &req.agent_status);
        Ok(())
    }

    pub fn replace_cached_pods(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedPodsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_pods(req.pods);
        Ok(())
    }

    pub fn append_cached_pods(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = AppendCachedPodsRequest::decode(req_bytes).map_err(decode_err)?;
        for pod in req.pods {
            self.inner.upsert_pod(pod, None);
        }
        Ok(())
    }

    pub fn mark_pod_terminated(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = MarkPodTerminatedRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_pod_status(&req.pod_key, "terminated", None, None, None, None);
        Ok(())
    }

    pub fn remove_pod(&mut self, pod_key: &str) {
        self.inner.remove_pod(pod_key);
    }

    pub fn update_init_progress(
        &mut self, pod_key: &str, phase: &str, progress: f64, message: Option<String>,
    ) {
        self.inner.update_init_progress(pod_key, phase, progress, message.as_deref());
    }

    pub fn clear_init_progress(&mut self, pod_key: &str) {
        self.inner.clear_init_progress(pod_key);
    }
}
