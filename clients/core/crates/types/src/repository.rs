use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub provider_type: Option<String>,
    pub provider_base_url: Option<String>,
    pub http_clone_url: Option<String>,
    pub ssh_clone_url: Option<String>,
    pub external_id: Option<String>,
    pub default_branch: Option<String>,
    pub ticket_prefix: Option<String>,
    pub visibility: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub provider_type: Option<String>,
    pub provider_base_url: Option<String>,
    pub http_clone_url: Option<String>,
    pub ssh_clone_url: Option<String>,
    pub external_id: Option<String>,
    pub name: String,
    pub slug: Option<String>,
    pub default_branch: Option<String>,
    pub ticket_prefix: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepositoryRequest {
    pub name: Option<String>,
    pub default_branch: Option<String>,
    pub ticket_prefix: Option<String>,
    pub is_active: Option<bool>,
    pub http_clone_url: Option<String>,
    pub ssh_clone_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub is_default: Option<bool>,
    pub last_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBranchesRequest {
    pub access_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookStatus {
    pub is_configured: Option<bool>,
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSecret {
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMergeRequest {
    pub id: i64,
    pub title: Option<String>,
    pub state: Option<String>,
    pub source_branch: Option<String>,
    pub target_branch: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryListResponse {
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchListResponse {
    pub branches: Vec<Branch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequestListResponse {
    pub merge_requests: Vec<RepositoryMergeRequest>,
}
