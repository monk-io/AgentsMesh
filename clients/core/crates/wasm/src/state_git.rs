use agentsmesh_state::git_provider_state::GitProviderState;
use agentsmesh_state::credential_types::{ProviderRepository, RepositoryProvider};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGitProviderState {
    inner: GitProviderState,
}

#[wasm_bindgen]
impl WasmGitProviderState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: GitProviderState::with_storage(crate::new_memory_backend()) }
    }

    pub fn providers_json(&self) -> String {
        serde_json::to_string(self.inner.providers()).unwrap_or_default()
    }

    pub fn current_provider_json(&self) -> JsValue {
        match self.inner.current_provider() {
            Some(p) => JsValue::from_str(&serde_json::to_string(p).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn available_projects_json(&self) -> String {
        serde_json::to_string(self.inner.available_projects()).unwrap_or_default()
    }

    pub fn set_providers(&mut self, json: &str) {
        if let Ok(providers) = serde_json::from_str::<Vec<RepositoryProvider>>(json) {
            self.inner.set_providers(providers);
        }
    }

    pub fn set_current_provider(&mut self, json: &str) {
        let provider = if json.is_empty() { None } else { serde_json::from_str::<RepositoryProvider>(json).ok() };
        self.inner.set_current_provider(provider);
    }

    pub fn set_available_projects(&mut self, json: &str) {
        if let Ok(projects) = serde_json::from_str::<Vec<ProviderRepository>>(json) {
            self.inner.set_available_projects(projects);
        }
    }

    pub fn add_provider(&mut self, json: &str) {
        if let Ok(provider) = serde_json::from_str::<RepositoryProvider>(json) {
            self.inner.add_provider(provider);
        }
    }

    pub fn update_provider(&mut self, id: &str, json: &str) {
        if let Ok(provider) = serde_json::from_str::<RepositoryProvider>(json) {
            self.inner.update_provider(id, provider);
        }
    }

    pub fn remove_provider(&mut self, id: &str) {
        self.inner.remove_provider(id);
    }
}
