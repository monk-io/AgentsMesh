use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    #[napi]
    pub async fn runner_update_runner(&self, id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.update_runner(id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn runner_list_runner_pods(&self, id: i64, status: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.list_runner_pods(id, status, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn runner_get_auth_status(&self, request_bytes: Vec<u8>) -> napi::Result<Vec<u8>> {
        let svc = self.runner.lock().await;
        svc.get_auth_status_connect(&request_bytes).await.map_err(err)
    }

    #[napi]
    pub async fn runner_authorize_runner(&self, request_bytes: Vec<u8>) -> napi::Result<Vec<u8>> {
        let svc = self.runner.lock().await;
        svc.authorize_runner_connect(&request_bytes).await.map_err(err)
    }
}
