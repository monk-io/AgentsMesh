use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessage {
    pub id: i64,
    pub sender_pod: Option<String>,
    pub receiver_pod: Option<String>,
    pub message_type: Option<String>,
    pub content: Option<String>,
    pub correlation_id: Option<String>,
    pub reply_to_id: Option<i64>,
    pub is_read: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDirectMessageRequest {
    pub receiver_pod: String,
    pub message_type: Option<String>,
    pub content: String,
    pub correlation_id: Option<String>,
    pub reply_to_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkMessagesReadRequest {
    pub message_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessageListResponse {
    pub messages: Vec<DirectMessage>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnreadCountResponse {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterEntry {
    pub id: i64,
    pub message: Option<DirectMessage>,
    pub error: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterListResponse {
    pub entries: Vec<DeadLetterEntry>,
}
