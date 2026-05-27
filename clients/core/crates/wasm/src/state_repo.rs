use agentsmesh_state::repo_state::RepoState;
use agentsmesh_types::proto_repo_state_v1::{
    InsertRepositoryRequest, PatchRepositoryRequest, ReplaceBranchesRequest,
    ReplaceCachedRepositoriesRequest, SetCurrentRepoRequest,
};
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRepoState {
    inner: RepoState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

#[wasm_bindgen]
impl WasmRepoState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: RepoState::with_storage(crate::new_memory_backend()) }
    }

    pub fn repositories_json(&self) -> String {
        serde_json::to_string(self.inner.repositories()).unwrap_or_default()
    }

    pub fn current_repo_json(&self) -> JsValue {
        match self.inner.current_repo() {
            Some(r) => JsValue::from_str(&serde_json::to_string(r).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn branches_json(&self) -> String {
        serde_json::to_string(self.inner.branches()).unwrap_or_default()
    }

    pub fn replace_cached_repositories(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedRepositoriesRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_repositories(req.repositories);
        Ok(())
    }

    pub fn set_current_repo_proto(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentRepoRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_current_repo(req.repository);
        Ok(())
    }

    pub fn replace_branches(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceBranchesRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_branches(req.branches);
        Ok(())
    }

    pub fn insert_repository(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertRepositoryRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(repo) = req.repository {
            self.inner.add_repository(repo);
        }
        Ok(())
    }

    pub fn patch_repository(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchRepositoryRequest::decode(req_bytes).map_err(decode_err)?;
        if let Some(repo) = req.repository {
            self.inner.update_repository(&req.id, repo);
        }
        Ok(())
    }

    pub fn remove_repository(&mut self, id: &str) {
        self.inner.remove_repository(id);
    }
}
