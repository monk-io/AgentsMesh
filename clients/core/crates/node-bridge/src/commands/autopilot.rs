use napi_derive::napi;
use crate::AppState;

#[napi]
impl AppState {
    #[napi]
    pub async fn autopilot_controllers_json(&self) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.controllers_json())
    }

    #[napi]
    pub async fn autopilot_current_controller_json(&self) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.current_controller_json().unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_controller_by_pod_key_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_controller_by_pod_key_json(&pod_key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_iterations_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_iterations_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_thinking_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_thinking_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_thinking_history_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_thinking_history_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_replace_cached_controllers(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.replace_cached_controllers(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_set_current_controller_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.set_current_controller_proto(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_insert_controller(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.insert_controller(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_patch_controller(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.patch_controller(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_remove_controller_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.remove_controller_proto(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_replace_cached_iterations(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.replace_cached_iterations(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_append_iteration(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.append_iteration(&req_bytes).map_err(napi::Error::from_reason)
    }

    #[napi]
    pub async fn autopilot_update_thinking_proto(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
        svc.update_thinking_proto(&req_bytes).map_err(napi::Error::from_reason)
    }

}
