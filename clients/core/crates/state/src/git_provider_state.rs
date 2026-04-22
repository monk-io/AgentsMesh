use std::sync::Arc;

use agentsmesh_persistence::StorageBackend;
use agentsmesh_types::{ProviderRepository, RepositoryProvider};

use crate::persist_helpers::JsonStore;

pub struct GitProviderState {
    providers: Vec<RepositoryProvider>,
    current_provider: Option<RepositoryProvider>,
    available_projects: Vec<ProviderRepository>,
    store: Option<JsonStore>,
}

impl GitProviderState {
    pub fn new() -> Self {
        Self { providers: Vec::new(), current_provider: None, available_projects: Vec::new(), store: None }
    }

    pub fn with_storage(backend: Arc<dyn StorageBackend>) -> Self {
        let store = JsonStore::new(backend, "git_providers");
        let providers = store.load_all();
        Self { providers, current_provider: None, available_projects: Vec::new(), store: Some(store) }
    }

    pub fn providers(&self) -> &[RepositoryProvider] { &self.providers }
    pub fn current_provider(&self) -> Option<&RepositoryProvider> { self.current_provider.as_ref() }
    pub fn available_projects(&self) -> &[ProviderRepository] { &self.available_projects }

    pub fn set_providers(&mut self, providers: Vec<RepositoryProvider>) {
        self.providers = providers;
        if let Some(store) = &self.store { store.replace_all(&self.providers, |p| p.id.to_string()); }
    }

    pub fn set_current_provider(&mut self, provider: Option<RepositoryProvider>) {
        self.current_provider = provider;
    }

    pub fn set_available_projects(&mut self, projects: Vec<ProviderRepository>) {
        self.available_projects = projects;
    }

    pub fn add_provider(&mut self, provider: RepositoryProvider) {
        if let Some(store) = &self.store {
            store.save(&provider.id.to_string(), &provider);
        }
        self.providers.push(provider);
    }

    pub fn update_provider(&mut self, id: &str, provider: RepositoryProvider) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.id.to_string() == id) {
            *p = provider.clone();
            if let Some(store) = &self.store { store.save(id, p); }
        }
        if self.current_provider.as_ref().is_some_and(|p| p.id.to_string() == id) {
            self.current_provider = Some(provider);
        }
    }

    pub fn remove_provider(&mut self, id: &str) {
        self.providers.retain(|p| p.id.to_string() != id);
        if self.current_provider.as_ref().is_some_and(|p| p.id.to_string() == id) {
            self.current_provider = None;
        }
        if let Some(store) = &self.store { store.delete(id); }
    }
}

impl Default for GitProviderState {
    fn default() -> Self { Self::new() }
}
