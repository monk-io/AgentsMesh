use napi_derive::napi;
use crate::AppState;

#[napi]
impl AppState {
    #[napi]
    pub async fn channel_channel_pods_json(&self, id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
        Ok(svc.channel_pods_json(id))
    }

    #[napi]
    pub async fn channel_channel_members_json(&self, id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
        Ok(svc.channel_members_json(id))
    }
}
