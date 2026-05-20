use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    #[napi]
    pub async fn env_bundle_list(&self, kind: Option<String>, agent_slug: Option<String>) -> napi::Result<String> {
        let svc = self.env_bundle.lock().await;
        svc.list(kind.as_deref(), agent_slug.as_deref()).await.map_err(err)
    }

    #[napi]
    pub async fn env_bundle_get(&self, id: i64) -> napi::Result<String> {
        let svc = self.env_bundle.lock().await;
        svc.get(id).await.map_err(err)
    }

    #[napi]
    pub async fn env_bundle_create(&self, json: String) -> napi::Result<String> {
        let svc = self.env_bundle.lock().await;
        svc.create(&json).await.map_err(err)
    }

    #[napi]
    pub async fn env_bundle_update(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.env_bundle.lock().await;
        svc.update(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn env_bundle_delete(&self, id: i64) -> napi::Result<()> {
        let svc = self.env_bundle.lock().await;
        svc.delete(id).await.map_err(err)
    }

    #[napi]
    pub async fn env_bundle_set_primary(&self, id: i64) -> napi::Result<String> {
        let svc = self.env_bundle.lock().await;
        svc.set_primary(id).await.map_err(err)
    }
}
