use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPodSettings {
    pub default_runner_id: Option<i64>,
    pub default_agent_slug: Option<String>,
    pub preferences: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub base_url: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAIProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAIProviderRequest {
    pub name: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderListResponse {
    pub providers: Vec<AIProvider>,
}
