use agentsmesh_state::org_state::{OrgMemberView, OrgState};
use agentsmesh_state::auth_types::Organization;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmOrgState {
    inner: OrgState,
}

#[wasm_bindgen]
impl WasmOrgState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: OrgState::with_storage(crate::new_memory_backend()) }
    }

    pub fn organizations_json(&self) -> String {
        serde_json::to_string(self.inner.organizations()).unwrap_or_default()
    }

    pub fn current_org_json(&self) -> JsValue {
        match self.inner.current_org() {
            Some(o) => JsValue::from_str(&serde_json::to_string(o).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn members_json(&self) -> String {
        serde_json::to_string(self.inner.members()).unwrap_or_default()
    }

    pub fn set_organizations(&mut self, json: &str) {
        if let Ok(orgs) = serde_json::from_str::<Vec<Organization>>(json) {
            self.inner.set_organizations(orgs);
        }
    }

    pub fn add_organization(&mut self, json: &str) {
        if let Ok(org) = serde_json::from_str::<Organization>(json) {
            self.inner.add_organization(org);
        }
    }

    pub fn update_organization(&mut self, id: f64, json: &str) {
        if let Ok(org) = serde_json::from_str::<Organization>(json) {
            self.inner.update_organization(id as i64, org);
        }
    }

    pub fn remove_organization(&mut self, id: f64) {
        self.inner.remove_organization(id as i64);
    }

    pub fn set_current_org(&mut self, json: &str) {
        let org = if json.is_empty() { None } else { serde_json::from_str::<Organization>(json).ok() };
        self.inner.set_current_org(org);
    }

    pub fn set_members(&mut self, json: &str) {
        if let Ok(members) = serde_json::from_str::<Vec<OrgMemberView>>(json) {
            self.inner.set_members(members);
        }
    }

    pub fn add_member(&mut self, json: &str) {
        if let Ok(member) = serde_json::from_str::<OrgMemberView>(json) {
            self.inner.add_member(member);
        }
    }

    pub fn update_member(&mut self, user_id: f64, json: &str) {
        if let Ok(member) = serde_json::from_str::<OrgMemberView>(json) {
            self.inner.update_member(user_id as i64, member);
        }
    }

    pub fn remove_member(&mut self, id: &str) {
        self.inner.remove_member(id);
    }
}
