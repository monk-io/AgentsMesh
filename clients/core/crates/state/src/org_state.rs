use std::sync::Arc;

use agentsmesh_persistence::StorageBackend;
use agentsmesh_types::Organization;
use serde::{Deserialize, Serialize};

use crate::persist_helpers::JsonStore;

/// Flat view of an organization member, projected from `proto.org.v1.OrgMember`
/// joined with the embedded `User`. Client-side aggregated shape — not a wire
/// type. Lives in the state crate because it is purely a cache projection
/// (no Connect-RPC counterpart).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMemberView {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: Option<String>,
}

pub struct OrgState {
    organizations: Vec<Organization>,
    current_org: Option<Organization>,
    members: Vec<OrgMemberView>,
    org_store: Option<JsonStore>,
    member_store: Option<JsonStore>,
}

impl OrgState {
    pub fn new() -> Self {
        Self { organizations: Vec::new(), current_org: None, members: Vec::new(), org_store: None, member_store: None }
    }

    pub fn with_storage(backend: Arc<dyn StorageBackend>) -> Self {
        let org_store = JsonStore::new(backend.clone(), "organizations");
        let member_store = JsonStore::new(backend, "org_members");
        let organizations = org_store.load_all();
        let members = member_store.load_all();
        Self { organizations, current_org: None, members, org_store: Some(org_store), member_store: Some(member_store) }
    }

    pub fn organizations(&self) -> &[Organization] { &self.organizations }
    pub fn current_org(&self) -> Option<&Organization> { self.current_org.as_ref() }
    pub fn members(&self) -> &[OrgMemberView] { &self.members }

    pub fn set_organizations(&mut self, orgs: Vec<Organization>) {
        self.organizations = orgs;
        if let Some(store) = &self.org_store {
            store.replace_all(&self.organizations, |o| o.id.to_string());
        }
    }

    pub fn add_organization(&mut self, org: Organization) {
        if let Some(store) = &self.org_store {
            store.save(&org.id.to_string(), &org);
        }
        self.organizations.push(org);
    }

    pub fn update_organization(&mut self, id: i64, updated: Organization) {
        if let Some(o) = self.organizations.iter_mut().find(|o| o.id == id) {
            *o = updated.clone();
            if let Some(store) = &self.org_store {
                store.save(&id.to_string(), o);
            }
        }
        if self.current_org.as_ref().is_some_and(|c| c.id == id) {
            self.current_org = Some(updated);
        }
    }

    pub fn remove_organization(&mut self, id: i64) {
        self.organizations.retain(|o| o.id != id);
        if let Some(store) = &self.org_store { store.delete(&id.to_string()); }
        if self.current_org.as_ref().is_some_and(|c| c.id == id) {
            self.current_org = None;
        }
    }

    pub fn set_current_org(&mut self, org: Option<Organization>) {
        self.current_org = org;
    }

    pub fn set_members(&mut self, members: Vec<OrgMemberView>) {
        self.members = members;
        if let Some(store) = &self.member_store {
            store.replace_all(&self.members, |m| m.id.to_string());
        }
    }

    pub fn add_member(&mut self, member: OrgMemberView) {
        if let Some(store) = &self.member_store {
            store.save(&member.id.to_string(), &member);
        }
        self.members.push(member);
    }

    pub fn update_member(&mut self, user_id: i64, updated: OrgMemberView) {
        if let Some(m) = self.members.iter_mut().find(|m| m.user_id == user_id) {
            *m = updated;
            if let Some(store) = &self.member_store {
                store.save(&m.id.to_string(), m);
            }
        }
    }

    pub fn remove_member(&mut self, id: &str) {
        self.members.retain(|m| m.id.to_string() != id);
        if let Some(store) = &self.member_store { store.delete(id); }
    }
}

impl Default for OrgState {
    fn default() -> Self { Self::new() }
}
