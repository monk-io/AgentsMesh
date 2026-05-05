use agentsmesh_types::Organization;

use crate::error::{parse_error_response, AuthError};
use crate::manager::AuthManager;

#[derive(serde::Deserialize)]
struct OrgsResponse {
    organizations: Vec<Organization>,
}

impl AuthManager {
    pub async fn fetch_organizations(&self) -> Result<Vec<Organization>, AuthError> {
        let auth = self.bearer_header()?;
        let resp = self
            .http
            .get(format!(
                "{}/api/v1/users/me/organizations",
                self.base_url
            ))
            .header("Authorization", auth)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error_response(resp).await);
        }

        let wrapper: OrgsResponse = resp
            .json()
            .await
            .map_err(|e| AuthError::InvalidResponse(e.to_string()))?;
        let orgs = wrapper.organizations;

        {
            let mut state = self.state.write().unwrap_or_else(|e| e.into_inner());
            state.organizations = orgs.clone();
            if state.current_org.is_none() {
                if let Some(first) = orgs.first() {
                    state.current_org = Some(first.clone());
                }
            }
        }
        self.persist();
        Ok(orgs)
    }

    pub fn get_organizations(&self) -> Vec<Organization> {
        self.state.read().unwrap_or_else(|e| e.into_inner()).organizations.clone()
    }

    pub fn get_current_org(&self) -> Option<Organization> {
        self.state.read().unwrap_or_else(|e| e.into_inner()).current_org.clone()
    }

    pub fn set_current_org(&self, org: Organization) {
        self.state.write().unwrap_or_else(|e| e.into_inner()).current_org = Some(org);
        self.persist();
    }

    pub fn switch_org(&self, slug: &str) -> Result<(), AuthError> {
        let mut state = self.state.write().unwrap_or_else(|e| e.into_inner());
        let org = state
            .organizations
            .iter()
            .find(|o| o.slug == slug)
            .cloned()
            .ok_or_else(|| {
                AuthError::InvalidResponse(format!("organization '{slug}' not found"))
            })?;

        state.current_org = Some(org);
        drop(state);
        self.persist();
        Ok(())
    }
}
