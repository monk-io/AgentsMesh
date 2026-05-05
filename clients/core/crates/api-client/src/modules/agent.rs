use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_agents(&self) -> Result<AgentListResponse, ApiError> {
        self.get(&self.org_path("/agents")).await
    }

    pub async fn get_agent_config_schema(
        &self,
        agent_slug: &str,
    ) -> Result<AgentConfigSchema, ApiError> {
        self.get_resource(&self.org_path(&format!("/agents/{agent_slug}/config-schema")), "schema").await
    }

    pub async fn list_user_agent_configs(
        &self,
    ) -> Result<UserAgentConfigListResponse, ApiError> {
        self.get("/api/v1/users/me/agent-configs").await
    }

    pub async fn get_user_agent_config(
        &self,
        agent_slug: &str,
    ) -> Result<UserAgentConfig, ApiError> {
        self.get_resource(&format!("/api/v1/users/me/agent-configs/{agent_slug}"), "config").await
    }

    pub async fn set_user_agent_config(
        &self,
        agent_slug: &str,
        data: &SetUserAgentConfigRequest,
    ) -> Result<UserAgentConfig, ApiError> {
        self.put(
            &format!("/api/v1/users/me/agent-configs/{agent_slug}"),
            data,
        )
        .await
    }

    pub async fn delete_user_agent_config(
        &self,
        agent_slug: &str,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/users/me/agent-configs/{agent_slug}"))
            .await
    }
}
