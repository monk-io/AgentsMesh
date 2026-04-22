use agentsmesh_state::user_state::UserState;
use agentsmesh_types::{User, UserIdentity};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmUserState {
    inner: UserState,
}

#[wasm_bindgen]
impl WasmUserState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: UserState::with_storage(crate::new_memory_backend()) }
    }

    pub fn profile_json(&self) -> JsValue {
        match self.inner.profile() {
            Some(p) => JsValue::from_str(&serde_json::to_string(p).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn identities_json(&self) -> String {
        serde_json::to_string(self.inner.identities()).unwrap_or_default()
    }

    pub fn set_profile(&mut self, json: &str) {
        let profile = if json.is_empty() { None } else { serde_json::from_str::<User>(json).ok() };
        self.inner.set_profile(profile);
    }

    pub fn add_identity(&mut self, json: &str) {
        if let Ok(identity) = serde_json::from_str::<UserIdentity>(json) {
            self.inner.add_identity(identity);
        }
    }

    pub fn remove_identity(&mut self, id: &str) {
        self.inner.remove_identity(id);
    }
}
