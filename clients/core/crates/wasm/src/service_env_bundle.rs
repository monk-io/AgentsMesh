use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::EnvBundleService;
use wasm_bindgen::prelude::*;

/// Wasm wrapper around EnvBundleService. Mirrors the facade method shape
/// (Strings in/out) for renderer consumption — the frontend just relays
/// JSON. `kind` and `agent_slug` are optional query filters; pass empty
/// string to omit.
#[wasm_bindgen]
pub struct WasmEnvBundleService {
    inner: EnvBundleService,
}

#[wasm_bindgen]
impl WasmEnvBundleService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { inner: EnvBundleService::new(client) }
    }

    pub async fn list(&self, kind: String, agent_slug: String) -> Result<String, String> {
        let k = if kind.is_empty() { None } else { Some(kind.as_str()) };
        let a = if agent_slug.is_empty() { None } else { Some(agent_slug.as_str()) };
        self.inner.list(k, a).await
    }

    pub async fn get(&self, id: i64) -> Result<String, String> {
        self.inner.get(id).await
    }

    pub async fn create(&self, json: String) -> Result<String, String> {
        self.inner.create(&json).await
    }

    pub async fn update(&self, id: i64, json: String) -> Result<String, String> {
        self.inner.update(id, &json).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.inner.delete(id).await
    }

    pub async fn set_primary(&self, id: i64) -> Result<String, String> {
        self.inner.set_primary(id).await
    }
}
