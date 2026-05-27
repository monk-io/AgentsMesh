use napi_derive::napi;
use crate::AppState;

#[napi]
impl AppState {
    #[napi]
    pub async fn pod_pods_json(&self) -> napi::Result<String> {
        let svc = self.pod.lock().await;
        Ok(svc.pods_json())
    }

    #[napi]
    pub async fn pod_current_pod_json(&self) -> napi::Result<String> {
        let svc = self.pod.lock().await;
        Ok(svc.current_pod_json().unwrap_or_default())
    }

    #[napi]
    pub async fn pod_get_pod_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.pod.lock().await;
        Ok(svc.get_pod_json(&pod_key).unwrap_or_default())
    }
}
