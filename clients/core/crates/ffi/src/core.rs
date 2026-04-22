use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_auth::AuthManager;

use crate::callbacks::StorageCallback;
use crate::dto::{OrganizationDto, UserDto};
use crate::error::CoreError;
use crate::storage_bridge::StorageBridge;

#[derive(uniffi::Object)]
pub struct AgentsMeshCore {
    pub(crate) auth: Arc<AuthManager>,
    pub(crate) api: Arc<ApiClient>,
}

#[uniffi::export]
impl AgentsMeshCore {
    #[uniffi::constructor]
    pub fn new(base_url: String, storage: Box<dyn StorageCallback>) -> Self {
        let bridge = Arc::new(StorageBridge::new(Arc::from(storage)));
        let auth = Arc::new(AuthManager::new(base_url.clone(), bridge));
        let api = Arc::new(ApiClient::new(base_url, auth.clone()));
        Self { auth, api }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth.is_authenticated()
    }

    pub fn restore_session(&self) -> Result<bool, CoreError> {
        self.auth.restore_session().map_err(CoreError::from)
    }

    /// Strongly-typed current-user accessor for Swift/Kotlin.
    pub fn get_current_user(&self) -> Option<UserDto> {
        self.auth.current_user().map(UserDto::from)
    }

    /// Strongly-typed current-org accessor for Swift/Kotlin.
    pub fn get_current_org(&self) -> Option<OrganizationDto> {
        self.auth.get_current_org().map(OrganizationDto::from)
    }

    /// Strongly-typed organization list accessor for Swift/Kotlin.
    pub fn get_organizations(&self) -> Vec<OrganizationDto> {
        self.auth
            .get_organizations()
            .into_iter()
            .map(OrganizationDto::from)
            .collect()
    }

    // ── Legacy JSON accessors ──
    // Retained for WASM/node-bridge parity until those frontends migrate off.

    pub fn get_current_user_json(&self) -> Option<String> {
        self.auth
            .current_user()
            .and_then(|u| serde_json::to_string(&u).ok())
    }

    pub fn get_current_org_json(&self) -> Option<String> {
        self.auth
            .get_current_org()
            .and_then(|o| serde_json::to_string(&o).ok())
    }

    pub fn get_organizations_json(&self) -> Result<String, CoreError> {
        let orgs = self.auth.get_organizations();
        Ok(serde_json::to_string(&orgs)?)
    }
}
