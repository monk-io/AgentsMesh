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
    /// Ordered list of EnvBundle names to attach to every run. Each name
    /// is emitted as a `USE_ENV_BUNDLE "<name>"` line in the generated
    /// AgentFile (in array order; later entries override earlier ones on
    /// conflicting env keys). Empty / None = no bundles.
    pub used_env_bundles: Option<Vec<String>>,
    pub config_overrides: Option<serde_json::Value>,
    pub execution_mode: Option<String>,
    pub cron_expression: Option<String>,
    pub autopilot_config: Option<serde_json::Value>,
    pub callback_url: Option<String>,
    pub sandbox_strategy: Option<String>,
    pub session_persistence: Option<bool>,
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
    /// None leaves the bundle list unchanged; Some(empty) clears it;
    /// Some(non-empty) replaces it with the supplied ordered list.
    pub used_env_bundles: Option<Vec<String>>,
    pub cron_expression: Option<String>,
    pub autopilot_config: Option<serde_json::Value>,
    pub sandbox_strategy: Option<String>,
    pub session_persistence: Option<bool>,
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
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

#[cfg(test)]
mod pagination_tests {
    use super::*;

    #[test]
    fn run_list_relay_preserves_pagination() {
        let backend = r#"{"runs":[],"total":120,"limit":25,"offset":50}"#;
        let typed: LoopRunListResponse = serde_json::from_str(backend).unwrap();
        let relayed = serde_json::to_string(&typed).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&relayed).unwrap();
        assert_eq!(parsed["total"], serde_json::json!(120));
        assert_eq!(parsed["limit"], serde_json::json!(25));
        assert_eq!(parsed["offset"], serde_json::json!(50));
    }

    #[test]
    fn create_loop_request_accepts_boolean_session_persistence() {
        // Frontend serializes session_persistence as JSON boolean (not string),
        // matching the backend's *bool field. Regression guard against a prior
        // Option<String> typing that silently rejected every dialog submit.
        let body = r#"{
            "name": "demo",
            "agent_slug": "claude-code",
            "prompt_template": "go",
            "session_persistence": true,
            "max_concurrent_runs": 1,
            "max_retained_runs": 0,
            "timeout_minutes": 60
        }"#;
        let req: CreateLoopRequest = serde_json::from_str(body).unwrap();
        assert_eq!(req.session_persistence, Some(true));
    }

    #[test]
    fn update_loop_request_accepts_boolean_session_persistence() {
        let body = r#"{"session_persistence": false}"#;
        let req: UpdateLoopRequest = serde_json::from_str(body).unwrap();
        assert_eq!(req.session_persistence, Some(false));
    }
}
