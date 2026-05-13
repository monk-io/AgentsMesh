use agentsmesh_types::{BoardColumn, BoardResponse, Label, LabelListResponse, Ticket, TicketListResponse, TicketPriority, TicketStatus};

// ── Enums ─────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, uniffi::Enum)]
pub enum TicketStatusDto {
    Backlog,
    Todo,
    InProgress,
    InReview,
    Done,
    Unknown,
}

impl From<TicketStatus> for TicketStatusDto {
    fn from(s: TicketStatus) -> Self {
        match s {
            TicketStatus::Backlog => Self::Backlog,
            TicketStatus::Todo => Self::Todo,
            TicketStatus::InProgress => Self::InProgress,
            TicketStatus::InReview => Self::InReview,
            TicketStatus::Done => Self::Done,
            TicketStatus::Unknown => Self::Unknown,
        }
    }
}

impl From<TicketStatusDto> for TicketStatus {
    fn from(s: TicketStatusDto) -> Self {
        match s {
            TicketStatusDto::Backlog => Self::Backlog,
            TicketStatusDto::Todo => Self::Todo,
            TicketStatusDto::InProgress => Self::InProgress,
            TicketStatusDto::InReview => Self::InReview,
            TicketStatusDto::Done => Self::Done,
            TicketStatusDto::Unknown => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, uniffi::Enum)]
pub enum TicketPriorityDto {
    None,
    Low,
    Medium,
    High,
    Urgent,
    Unknown,
}

impl From<TicketPriority> for TicketPriorityDto {
    fn from(p: TicketPriority) -> Self {
        match p {
            TicketPriority::None => Self::None,
            TicketPriority::Low => Self::Low,
            TicketPriority::Medium => Self::Medium,
            TicketPriority::High => Self::High,
            TicketPriority::Urgent => Self::Urgent,
            TicketPriority::Unknown => Self::Unknown,
        }
    }
}

impl From<TicketPriorityDto> for TicketPriority {
    fn from(p: TicketPriorityDto) -> Self {
        match p {
            TicketPriorityDto::None => Self::None,
            TicketPriorityDto::Low => Self::Low,
            TicketPriorityDto::Medium => Self::Medium,
            TicketPriorityDto::High => Self::High,
            TicketPriorityDto::Urgent => Self::Urgent,
            TicketPriorityDto::Unknown => Self::Unknown,
        }
    }
}

// ── Ticket ────────────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct TicketDto {
    pub slug: String,
    pub title: String,
    pub content: Option<String>,
    pub status: TicketStatusDto,
    pub priority: TicketPriorityDto,
    pub repository_id: Option<i64>,
    pub parent_slug: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Ticket> for TicketDto {
    fn from(t: Ticket) -> Self {
        Self {
            slug: t.slug,
            title: t.title,
            content: t.content,
            status: t.status.into(),
            priority: t.priority.into(),
            repository_id: t.repository_id,
            parent_slug: t.parent_slug,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct TicketListResponseDto {
    pub tickets: Vec<TicketDto>,
    pub total: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl From<TicketListResponse> for TicketListResponseDto {
    fn from(r: TicketListResponse) -> Self {
        Self {
            tickets: r.tickets.into_iter().map(TicketDto::from).collect(),
            total: r.total,
            limit: r.limit,
            offset: r.offset,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct BoardColumnDto {
    pub status: TicketStatusDto,
    pub tickets: Vec<TicketDto>,
    pub total_count: i64,
}

impl From<BoardColumn> for BoardColumnDto {
    fn from(c: BoardColumn) -> Self {
        Self {
            status: c.status.into(),
            tickets: c.tickets.into_iter().map(TicketDto::from).collect(),
            total_count: c.total_count,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct BoardResponseDto {
    pub columns: Vec<BoardColumnDto>,
    /// Opaque JSON of per-priority counts (server-side aggregated).
    pub priority_counts_json: Option<String>,
}

impl From<BoardResponse> for BoardResponseDto {
    fn from(r: BoardResponse) -> Self {
        Self {
            columns: r.columns.into_iter().map(BoardColumnDto::from).collect(),
            priority_counts_json: r
                .priority_counts
                .and_then(|v| serde_json::to_string(&v).ok()),
        }
    }
}

// ── Label ─────────────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct LabelDto {
    pub id: i64,
    pub name: String,
    pub color: String,
}

impl From<Label> for LabelDto {
    fn from(l: Label) -> Self {
        Self {
            id: l.id,
            name: l.name,
            color: l.color,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct LabelListResponseDto {
    pub labels: Vec<LabelDto>,
}

impl From<LabelListResponse> for LabelListResponseDto {
    fn from(r: LabelListResponse) -> Self {
        Self {
            labels: r.labels.into_iter().map(LabelDto::from).collect(),
        }
    }
}

// ── Request DTOs ──────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct CreateTicketRequestDto {
    pub title: String,
    pub content: Option<String>,
    pub priority: Option<TicketPriorityDto>,
    pub severity: Option<String>,
    pub estimate: Option<f64>,
    pub repository_id: Option<i64>,
    pub assignee_ids: Option<Vec<i64>>,
    pub labels: Option<Vec<i64>>,
    pub parent_slug: Option<String>,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UpdateTicketRequestDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub priority: Option<TicketPriorityDto>,
    pub severity: Option<String>,
    pub estimate: Option<f64>,
    pub repository_id: Option<i64>,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct CreateLabelRequestDto {
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UpdateLabelRequestDto {
    pub name: Option<String>,
    pub color: Option<String>,
}
