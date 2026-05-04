use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    #[napi]
    pub async fn sso_discover(&self, email: String) -> napi::Result<String> {
        let svc = self.sso.lock().await;
        svc.discover(&email).await.map_err(err)
    }

    #[napi]
    pub async fn sso_ldap_auth(&self, domain: String, json: String) -> napi::Result<String> {
        let svc = self.sso.lock().await;
        svc.ldap_auth(&domain, &json).await.map_err(err)
    }
}
