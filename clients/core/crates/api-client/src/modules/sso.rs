use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn sso_discover(&self, email: &str) -> Result<SSODiscoverResponse, ApiError> {
        self.public_get(&format!("/api/v1/auth/sso/discover?email={email}"))
            .await
    }

    pub async fn sso_ldap_auth(
        &self,
        domain: &str,
        data: &LdapAuthRequest,
    ) -> Result<AuthSession, ApiError> {
        self.public_post(&format!("/api/v1/auth/sso/{domain}/ldap"), data)
            .await
    }
}
