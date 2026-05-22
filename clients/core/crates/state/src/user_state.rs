use std::sync::Arc;

use agentsmesh_persistence::StorageBackend;

use crate::auth_types::{User, UserIdentity};
use crate::persist_helpers::JsonStore;

pub struct UserState {
    profile: Option<User>,
    identities: Vec<UserIdentity>,
    store: Option<JsonStore>,
}

impl UserState {
    pub fn new() -> Self {
        Self { profile: None, identities: Vec::new(), store: None }
    }

    pub fn with_storage(backend: Arc<dyn StorageBackend>) -> Self {
        let store = JsonStore::new(backend, "user_state");
        let profile = store.load_one::<User>("profile");
        let identities = store.load_one::<Vec<UserIdentity>>("identities")
            .unwrap_or_default();
        Self { profile, identities, store: Some(store) }
    }

    pub fn profile(&self) -> Option<&User> { self.profile.as_ref() }
    pub fn identities(&self) -> &[UserIdentity] { &self.identities }

    pub fn set_profile(&mut self, profile: Option<User>) {
        self.profile = profile;
        if let Some(store) = &self.store {
            if let Some(ref p) = self.profile {
                store.save("profile", p);
            } else {
                store.delete("profile");
            }
        }
    }

    pub fn add_identity(&mut self, identity: UserIdentity) {
        self.identities.push(identity);
        self.persist_identities();
    }

    pub fn remove_identity(&mut self, id: &str) {
        self.identities.retain(|i| i.id.to_string() != id);
        self.persist_identities();
    }

    fn persist_identities(&self) {
        if let Some(store) = &self.store {
            store.save("identities", &self.identities);
        }
    }
}

impl Default for UserState {
    fn default() -> Self { Self::new() }
}
