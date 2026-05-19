use serde::{Deserialize, Serialize};

/// Flat view of an organization member, matching the web frontend's data shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMemberView {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: Option<String>,
}
