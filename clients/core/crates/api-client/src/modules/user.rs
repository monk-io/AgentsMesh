use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn get_me(&self) -> Result<User, ApiError> {
        self.get_resource("/api/v1/users/me", "user").await
    }

    pub async fn get_organizations(&self) -> Result<OrganizationListResponse, ApiError> {
        self.get("/api/v1/users/me/organizations").await
    }
}
