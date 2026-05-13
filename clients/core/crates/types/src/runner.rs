use serde::{Deserialize, Serialize};

use crate::RunnerStatus;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Runner {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: RunnerStatus,
    #[serde(default, rename = "runner_version")]
    pub version: Option<String>,
    #[serde(default)]
    pub max_concurrent_pods: i32,
    #[serde(default, rename = "current_pods")]
    pub active_pod_count: i32,
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(default)]
    pub host_info: Option<serde_json::Value>,
    #[serde(default)]
    pub last_heartbeat: Option<String>,
    #[serde(default)]
    pub available_agents: Option<Vec<String>>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// Legacy request payloads: still consumed by services::RunnerService when it
// receives JSON requests from the web/desktop UI (the wire shape on the
// JS/NAPI boundary stays stable while the internal call routes through
// Connect-RPC). proto_runner_api_v1 owns the wire-level request types.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRunnerRequest {
    pub description: Option<String>,
    pub max_concurrent_pods: Option<i32>,
    pub is_enabled: Option<bool>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunnerTokenRequest {
    pub name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub max_uses: Option<i32>,
    pub expires_in_days: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxQueryRequest {
    pub pod_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRunnerRequest {
    pub target_version: Option<String>,
    pub force: Option<bool>,
}

// REST carve-outs without proto coverage (Tailscale-style runner
// registration). Stays serde so the kept ApiClient REST methods can drive
// them.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeRunnerRequest {
    pub auth_key: String,
    pub node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerAuthStatus {
    pub status: String,
    pub runner_id: Option<i64>,
    pub organization_slug: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn runner_roundtrip() {
        let runner = Runner {
            id: 1,
            name: "my-runner".into(),
            status: RunnerStatus::Online,
            version: Some("0.5.0".into()),
            max_concurrent_pods: 4,
            active_pod_count: 2,
            is_enabled: true,
            host_info: Some(serde_json::json!({"os": "linux"})),
            created_at: Some("2026-01-01T00:00:00Z".into()),
            updated_at: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&runner).unwrap();
        let decoded: Runner = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.name, "my-runner");
        assert_eq!(decoded.max_concurrent_pods, 4);
        assert!(decoded.is_enabled);
    }

    #[test]
    fn runner_minimal_json() {
        let json = r#"{
            "id":1,"name":"r","status":"offline",
            "max_concurrent_pods":2,"active_pod_count":0,"is_enabled":false
        }"#;
        let runner: Runner = serde_json::from_str(json).unwrap();
        assert_eq!(runner.id, 1);
        assert!(!runner.is_enabled);
        assert_eq!(runner.status, RunnerStatus::Offline);
        assert!(runner.version.is_none());
        assert!(runner.host_info.is_none());
    }

    #[test]
    fn runner_host_info_json_value() {
        let runner = Runner {
            id: 1,
            name: "r".into(),
            status: RunnerStatus::Online,
            version: None,
            max_concurrent_pods: 1,
            active_pod_count: 0,
            is_enabled: true,
            host_info: Some(serde_json::json!({"arch": "arm64", "cores": 8})),
            created_at: None,
            updated_at: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&runner).unwrap();
        let decoded: Runner = serde_json::from_str(&json).unwrap();
        let info = decoded.host_info.unwrap();
        assert_eq!(info["arch"], "arm64");
        assert_eq!(info["cores"], 8);
    }
}
