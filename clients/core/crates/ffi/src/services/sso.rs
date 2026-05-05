use agentsmesh_types::LdapAuthRequest;

use crate::core::AgentsMeshCore;
use crate::dto::{AuthSessionDto, SSOConfigDto};
use crate::error::CoreError;

/// SSO discovery + LDAP sign-in for enterprise tenants.
/// OAuth / SAML flows are handled via `SSOAuthUrlParams` on the browser;
/// iOS defers those to ASWebAuthenticationSession and does not need FFI
/// support for the redirect leg.
#[uniffi::export(async_runtime = "tokio")]
impl AgentsMeshCore {
    /// Discover which SSO providers are configured for an email's domain.
    /// Returns an empty vec if no enterprise SSO is configured.
    pub async fn sso_discover(&self, email: String) -> Result<Vec<SSOConfigDto>, CoreError> {
        let resp = self.api.sso_discover(&email).await?;
        Ok(resp.configs.into_iter().map(SSOConfigDto::from).collect())
    }

    /// LDAP credential-based sign-in.
    pub async fn sso_ldap_login(
        &self,
        domain: String,
        username: String,
        password: String,
    ) -> Result<AuthSessionDto, CoreError> {
        let req = LdapAuthRequest { username, password };
        let session = self.api.sso_ldap_auth(&domain, &req).await?;
        Ok(session.into())
    }
}
