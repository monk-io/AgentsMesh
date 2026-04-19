use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    pub id: i64,
    pub email: String,
    pub role: String,
    pub status: Option<String>,
    pub token: Option<String>,
    pub organization_name: Option<String>,
    pub organization_slug: Option<String>,
    pub inviter_name: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvitationRequest {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationListResponse {
    pub invitations: Vec<Invitation>,
}
