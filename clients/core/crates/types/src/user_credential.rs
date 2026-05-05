use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredential {
    pub id: i64,
    pub name: String,
    pub credential_type: Option<String>,
    pub repository_provider_id: Option<i64>,
    pub host_pattern: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGitCredentialRequest {
    pub name: String,
    pub credential_type: String,
    pub repository_provider_id: Option<i64>,
    pub pat: Option<String>,
    pub private_key: Option<String>,
    pub host_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGitCredentialRequest {
    pub name: Option<String>,
    pub pat: Option<String>,
    pub private_key: Option<String>,
    pub host_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDefaultGitCredentialRequest {
    pub credential_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredentialListResponse {
    #[serde(default)]
    pub credentials: Vec<GitCredential>,
    #[serde(default)]
    pub runner_local: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCredentialProfile {
    pub id: i64,
    pub agent_slug: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub is_runner_host: Option<bool>,
    pub is_default: Option<bool>,
    pub credentials: Option<serde_json::Value>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentCredentialProfileRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_runner_host: Option<bool>,
    pub credentials: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentCredentialProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_runner_host: Option<bool>,
    pub credentials: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCredentialProfileListResponse {
    #[serde(alias = "items", default)]
    pub profiles: Vec<AgentCredentialProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryProvider {
    pub id: i64,
    pub provider_type: String,
    pub name: String,
    pub base_url: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryProviderRequest {
    pub provider_type: String,
    pub name: String,
    pub base_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub bot_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepositoryProviderRequest {
    pub name: Option<String>,
    pub base_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub bot_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryProviderListResponse {
    pub providers: Vec<RepositoryProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRepository {
    pub id: Option<String>,
    pub name: String,
    pub full_name: Option<String>,
    pub clone_url: Option<String>,
    pub ssh_url: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRepositoryListResponse {
    pub repositories: Vec<ProviderRepository>,
    pub total: Option<i64>,
}
