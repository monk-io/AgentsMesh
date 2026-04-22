use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: i64,
    pub title: String,
    pub category: Option<String>,
    pub content: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicketMessage {
    pub id: i64,
    pub ticket_id: i64,
    pub content: Option<String>,
    pub sender_type: Option<String>,
    pub attachments: Option<Vec<String>>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicketListResponse {
    pub tickets: Vec<SupportTicket>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentUrlResponse {
    pub url: String,
}
