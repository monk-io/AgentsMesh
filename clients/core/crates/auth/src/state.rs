use agentsmesh_types::{Organization, User};
use serde::{Deserialize, Serialize};

pub(crate) const STORAGE_KEY: &str = "agentsmesh-auth";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthState {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: Option<User>,
    pub current_org: Option<Organization>,
    pub organizations: Vec<Organization>,
}

impl AuthState {
    pub fn apply_session(&mut self, session: &agentsmesh_types::AuthSession) {
        self.token = Some(session.token.clone());
        self.refresh_token = Some(session.refresh_token.clone());
        self.user = Some(session.user.clone());
    }

    pub fn apply_tokens(&mut self, tokens: &agentsmesh_types::AuthTokens) {
        self.token = Some(tokens.token.clone());
        self.refresh_token = Some(tokens.refresh_token.clone());
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
