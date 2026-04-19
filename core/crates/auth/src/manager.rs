use std::sync::{Arc, RwLock};

use agentsmesh_types::User;

use crate::error::AuthError;
use crate::state::{AuthState, STORAGE_KEY};
use crate::storage::PersistentStorage;

pub struct AuthManager {
    pub(crate) state: Arc<RwLock<AuthState>>,
    pub(crate) storage: Arc<dyn PersistentStorage>,
    pub(crate) base_url: String,
    pub(crate) http: reqwest::Client,
}

impl AuthManager {
    pub fn new(base_url: String, storage: Arc<dyn PersistentStorage>) -> Self {
        Self {
            state: Arc::new(RwLock::new(AuthState::default())),
            storage,
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.state.read().unwrap_or_else(|e| e.into_inner()).token.is_some()
    }

    pub fn current_user(&self) -> Option<User> {
        self.state.read().unwrap_or_else(|e| e.into_inner()).user.clone()
    }

    pub fn restore_session(&self) -> Result<bool, AuthError> {
        let json = match self.storage.get(STORAGE_KEY) {
            Some(v) => v,
            None => return Ok(false),
        };

        let restored: AuthState = serde_json::from_str(&json)
            .map_err(|e| AuthError::Storage(e.to_string()))?;

        if restored.token.is_none() {
            return Ok(false);
        }

        *self.state.write().unwrap_or_else(|e| e.into_inner()) = restored;
        Ok(true)
    }

    pub(crate) fn persist(&self) {
        let state = self.state.read().unwrap_or_else(|e| e.into_inner());
        if let Ok(json) = serde_json::to_string(&*state) {
            self.storage.set(STORAGE_KEY, &json);
        }
    }

    pub(crate) fn bearer_header(&self) -> Result<String, AuthError> {
        let state = self.state.read().unwrap_or_else(|e| e.into_inner());
        state
            .token
            .as_ref()
            .map(|t| format!("Bearer {t}"))
            .ok_or(AuthError::NotAuthenticated)
    }
}
