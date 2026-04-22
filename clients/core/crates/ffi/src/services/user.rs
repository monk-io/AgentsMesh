use crate::core::AgentsMeshCore;
use crate::dto::UserDto;
use crate::error::CoreError;

/// Strongly-typed `User` API — hits the backend `/auth/me` endpoint.
#[uniffi::export]
impl AgentsMeshCore {
    /// Fetch the current authenticated user from the server. Useful after
    /// `restore_session` to confirm the token is still valid before routing
    /// the app past the login gate.
    pub async fn fetch_me(&self) -> Result<UserDto, CoreError> {
        let user = self.api.get_me().await?;
        Ok(user.into())
    }
}
