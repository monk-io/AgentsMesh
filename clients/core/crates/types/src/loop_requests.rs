use serde::{Deserialize, Serialize};

use crate::{LoopData, LoopRunData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoopRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub agent_slug: Option<String>,
    pub custom_agent_slug: Option<String>,
    pub permission_mode: Option<String>,
    pub prompt_template: Option<String>,
    pub prompt_variables: Option<serde_json::Value>,
    pub repository_id: Option<i64>,
    pub runner_id: Option<i64>,
    pub branch_name: Option<String>,
    pub ticket_id: Option<String>,
    pub credential_profile_id: Option<i64>,
    pub config_overrides: Option<serde_json::Value>,
    pub execution_mode: Option<String>,
    pub cron_expression: Option<String>,
    pub autopilot_config: Option<serde_json::Value>,
    pub callback_url: Option<String>,
    pub sandbox_strategy: Option<String>,
    pub session_persistence: Option<String>,
    pub concurrency_policy: Option<String>,
    pub max_concurrent_runs: Option<i64>,
    pub max_retained_runs: Option<i64>,
    pub timeout_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLoopRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub agent_slug: Option<String>,
    pub prompt_template: Option<String>,
    pub prompt_variables: Option<serde_json::Value>,
    pub repository_id: Option<i64>,
    pub runner_id: Option<i64>,
    pub branch_name: Option<String>,
    pub cron_expression: Option<String>,
    pub autopilot_config: Option<serde_json::Value>,
    pub sandbox_strategy: Option<String>,
    pub session_persistence: Option<String>,
    pub concurrency_policy: Option<String>,
    pub max_concurrent_runs: Option<i64>,
    pub max_retained_runs: Option<i64>,
    pub timeout_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopListResponse {
    pub loops: Vec<LoopData>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopRunListResponse {
    pub runs: Vec<LoopRunData>,
    pub total: Option<i64>,
}
