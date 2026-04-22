use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub id: i64,
    pub source_pod: Option<String>,
    pub target_pod: Option<String>,
    pub status: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub policy: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBindingRequest {
    pub target_pod: String,
    pub scopes: Option<Vec<String>>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptBindingRequest {
    pub binding_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectBindingRequest {
    pub binding_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestScopesBody {
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveScopesBody {
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindRequest {
    pub target_pod: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingListResponse {
    pub bindings: Vec<Binding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundPodsResponse {
    pub pods: Vec<String>,
}
