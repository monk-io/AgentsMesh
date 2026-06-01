use std::sync::Arc;

use agentsmesh_state::app_state::AppState;
use agentsmesh_types::proto_repo_state_v1::{
    InsertRepositoryRequest, PatchRepositoryRequest, ReplaceBranchesRequest,
    ReplaceCachedRepositoriesRequest, SetCurrentRepoRequest,
};
use parking_lot::RwLock;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRepoState {
    state: Arc<RwLock<AppState>>,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

impl WasmRepoState {
    pub(crate) fn from_runtime(state: Arc<RwLock<AppState>>) -> Self {
        Self { state }
    }
}

#[wasm_bindgen]
impl WasmRepoState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::with_storage(crate::new_memory_backend()))),
        }
    }

    pub fn repositories_json(&self) -> String {
        serde_json::to_string(self.state.read().repo.repositories()).unwrap_or_default()
    }

    pub fn current_repo_json(&self) -> JsValue {
        match self.state.read().repo.current_repo() {
            Some(r) => JsValue::from_str(&serde_json::to_string(r).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn branches_json(&self) -> String {
        serde_json::to_string(self.state.read().repo.branches()).unwrap_or_default()
    }

    pub fn replace_cached_repositories(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedRepositoriesRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().repo.set_repositories(req.repositories);
        Ok(())
    }

    pub fn set_current_repo_proto(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentRepoRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().repo.set_current_repo(req.repository);
        Ok(())
    }

    pub fn replace_branches(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceBranchesRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().repo.set_branches(req.branches);
        Ok(())
    }

    pub fn insert_repository(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertRepositoryRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(repo) = req.repository {
            self.state.write().repo.add_repository(repo);
        }
        Ok(())
    }

    pub fn patch_repository(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchRepositoryRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(repo) = req.repository {
            self.state.write().repo.update_repository(&req.id, repo);
        }
        Ok(())
    }

    pub fn remove_repository(&self, id: &str) {
        self.state.write().repo.remove_repository(id);
    }
}
