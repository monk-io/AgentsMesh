use serde::{Deserialize, Serialize};

// JSON-bridge request payloads consumed by services::RunnerService when it
// receives JSON strings from the web / desktop UI. The wire shape on the
// JS/NAPI boundary stays stable while the internal call routes through
// Connect-RPC (proto_runner_api_v1 owns the wire-level request types).

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

// REST carve-outs without proto coverage (Tailscale-style runner registration).
// Stays serde so the kept ApiClient REST methods can drive them.

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
