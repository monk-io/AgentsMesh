use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::ExtensionService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmExtensionService(pub(crate) ExtensionService);

#[wasm_bindgen]
impl WasmExtensionService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self(ExtensionService::new(client))
    }

    pub async fn list_skill_registries(&self) -> Result<String, String> {
        self.0.list_skill_registries().await
    }

    pub async fn create_skill_registry(&self, json: &str) -> Result<String, String> {
        self.0.create_skill_registry(json).await
    }

    pub async fn sync_skill_registry(&self, id: i64) -> Result<(), String> {
        self.0.sync_skill_registry(id).await
    }

    pub async fn toggle_skill_registry(&self, id: i64, json: &str) -> Result<String, String> {
        self.0.toggle_skill_registry(id, json).await
    }

    pub async fn delete_skill_registry(&self, id: i64) -> Result<(), String> {
        self.0.delete_skill_registry(id).await
    }

    pub async fn list_skill_registry_overrides(&self) -> Result<String, String> {
        self.0.list_skill_registry_overrides().await
    }

    pub async fn list_market_skills(
        &self, query: Option<String>, category: Option<String>,
    ) -> Result<String, String> {
        self.0.list_market_skills(query, category).await
    }

    pub async fn list_market_mcp_servers(
        &self, query: Option<String>, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        self.0.list_market_mcp_servers(query, limit, offset).await
    }

    pub async fn list_repo_skills(
        &self, repo_id: i64, scope: Option<String>,
    ) -> Result<String, String> {
        self.0.list_repo_skills(repo_id, scope).await
    }

    pub async fn install_skill_from_market(
        &self, repo_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.install_skill_from_market(repo_id, json).await
    }

    pub async fn install_skill_from_github(
        &self, repo_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.install_skill_from_github(repo_id, json).await
    }

    pub async fn update_skill(
        &self, repo_id: i64, install_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.update_skill(repo_id, install_id, json).await
    }

    pub async fn uninstall_skill(&self, repo_id: i64, install_id: i64) -> Result<(), String> {
        self.0.uninstall_skill(repo_id, install_id).await
    }

    pub async fn list_repo_mcp_servers(
        &self, repo_id: i64, scope: Option<String>,
    ) -> Result<String, String> {
        self.0.list_repo_mcp_servers(repo_id, scope).await
    }

    pub async fn install_mcp_from_market(
        &self, repo_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.install_mcp_from_market(repo_id, json).await
    }

    pub async fn install_custom_mcp_server(
        &self, repo_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.install_custom_mcp_server(repo_id, json).await
    }

    pub async fn update_mcp_server(
        &self, repo_id: i64, install_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.update_mcp_server(repo_id, install_id, json).await
    }

    pub async fn uninstall_mcp_server(
        &self, repo_id: i64, install_id: i64,
    ) -> Result<(), String> {
        self.0.uninstall_mcp_server(repo_id, install_id).await
    }

    pub async fn install_skill_from_upload(
        &self, repo_id: i64, file_data: js_sys::Uint8Array,
        file_name: &str, scope: Option<String>,
    ) -> Result<String, String> {
        let bytes = file_data.to_vec();
        self.0.install_skill_from_upload(repo_id, bytes, file_name, scope).await
    }
}
