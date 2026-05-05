use agentsmesh_api_client::AuthTokenStore;

use crate::manager::AuthManager;
use crate::state::STORAGE_KEY;

impl AuthTokenStore for AuthManager {
    fn get_token(&self) -> Option<String> {
        self.state.read().unwrap_or_else(|e| e.into_inner()).token.clone()
    }

    fn get_refresh_token(&self) -> Option<String> {
        self.state.read().unwrap_or_else(|e| e.into_inner()).refresh_token.clone()
    }

    fn set_tokens(&self, token: String, refresh_token: String) {
        {
            let mut state = self.state.write().unwrap_or_else(|e| e.into_inner());
            state.token = Some(token);
            state.refresh_token = Some(refresh_token);
        }
        self.persist();
    }

    fn clear_tokens(&self) {
        self.state.write().unwrap_or_else(|e| e.into_inner()).clear();
        self.storage.remove(STORAGE_KEY);
    }

    fn get_current_org_slug(&self) -> Option<String> {
        self.state
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .current_org
            .as_ref()
            .map(|o| o.slug.clone())
    }
}
