use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRegistry {
    pub id: i64,
    pub repository_url: Option<String>,
    pub branch: Option<String>,
    pub source_type: Option<String>,
    pub compatible_agents: Option<Vec<String>>,
    pub is_disabled: Option<bool>,
    pub last_synced_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillRegistryRequest {
    pub repository_url: String,
    pub branch: Option<String>,
    pub source_type: Option<String>,
    pub compatible_agents: Option<Vec<String>>,
    pub auth_type: Option<String>,
    pub auth_credential: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleRegistryRequest {
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRegistryOverride {
    pub id: i64,
    pub registry_id: Option<i64>,
    pub skill_slug: Option<String>,
    pub is_disabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSkill {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMcpServer {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub transport_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSkillInstall {
    pub id: i64,
    pub repository_id: Option<i64>,
    pub skill_slug: Option<String>,
    pub name: Option<String>,
    pub scope: Option<String>,
    pub is_enabled: Option<bool>,
    pub pinned_version: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMarketSkillRequest {
    pub market_item_id: i64,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallGithubSkillRequest {
    pub url: String,
    pub branch: Option<String>,
    pub path: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSkillInstallRequest {
    pub is_enabled: Option<bool>,
    pub pinned_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMcpServerInstall {
    pub id: i64,
    pub repository_id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub transport_type: Option<String>,
    pub scope: Option<String>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMarketMcpRequest {
    pub market_item_id: i64,
    pub env_vars: Option<serde_json::Value>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallCustomMcpRequest {
    pub name: String,
    pub slug: Option<String>,
    pub transport_type: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub http_url: Option<String>,
    pub http_headers: Option<serde_json::Value>,
    pub env_vars: Option<serde_json::Value>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMcpInstallRequest {
    pub is_enabled: Option<bool>,
    pub env_vars: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRegistryListResponse {
    #[serde(alias = "skill_registries")]
    pub registries: Vec<SkillRegistry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSkillListResponse {
    pub skills: Vec<MarketSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMcpServerListResponse {
    pub mcp_servers: Vec<MarketMcpServer>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRegistryOverrideListResponse {
    pub overrides: Vec<SkillRegistryOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSkillInstallListResponse {
    pub skills: Vec<RepoSkillInstall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMcpServerInstallListResponse {
    pub mcp_servers: Vec<RepoMcpServerInstall>,
}
