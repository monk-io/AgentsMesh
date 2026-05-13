use serde::{Deserialize, Serialize};

use crate::{TicketPriority, TicketStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub content: Option<String>,
    pub priority: Option<TicketPriority>,
    pub severity: Option<String>,
    pub estimate: Option<f64>,
    pub repository_id: Option<i64>,
    pub assignee_ids: Option<Vec<i64>>,
    pub labels: Option<Vec<i64>>,
    pub parent_slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub priority: Option<TicketPriority>,
    pub severity: Option<String>,
    pub estimate: Option<f64>,
    pub repository_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketStatusRequest {
    pub status: TicketStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAssigneeRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTicketLabelRequest {
    pub label_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLabelRequest {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLabelRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketPodRequest {
    pub runner_id: Option<i64>,
    pub prompt: Option<String>,
    pub model: Option<String>,
    pub permission_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPodRequest {
    pub ticket_slugs: Vec<String>,
}
