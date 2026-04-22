use agentsmesh_types::{
    AuthorizeRunnerRequest, Branch, CreateRepositoryRequest, CreateRunnerTokenRequest,
    GRPCRegistrationToken, MergeRequestListResponse, Repository, RepositoryListResponse,
    RepositoryMergeRequest, Runner, RunnerAuthStatus, RunnerListResponse, RunnerLog,
    RunnerLogListResponse, RunnerStatus, RunnerTokenListResponse, UpdateRepositoryRequest,
    UpdateRunnerRequest, UpgradeRunnerRequest, WebhookSecret, WebhookStatus,
};

// ── Runner ────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, uniffi::Enum)]
pub enum RunnerStatusDto {
    Online,
    Offline,
    Maintenance,
    Unknown,
}

impl From<RunnerStatus> for RunnerStatusDto {
    fn from(s: RunnerStatus) -> Self {
        match s {
            RunnerStatus::Online => Self::Online,
            RunnerStatus::Offline => Self::Offline,
            RunnerStatus::Maintenance => Self::Maintenance,
            RunnerStatus::Unknown => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerDto {
    pub id: i64,
    pub name: String,
    pub node_id: Option<String>,
    pub description: Option<String>,
    pub status: RunnerStatusDto,
    pub version: Option<String>,
    pub max_concurrent_pods: i32,
    pub active_pod_count: i32,
    pub is_enabled: bool,
    /// Raw `serde_json::Value` serialized to JSON — Swift side decodes if it
    /// needs specific fields. Keeps DTO schema stable across runner versions.
    pub host_info_json: Option<String>,
    pub last_heartbeat: Option<String>,
    pub available_agents: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Runner> for RunnerDto {
    fn from(r: Runner) -> Self {
        Self {
            id: r.id,
            name: r.name,
            node_id: r.node_id,
            description: r.description,
            status: r.status.into(),
            version: r.version,
            max_concurrent_pods: r.max_concurrent_pods,
            active_pod_count: r.active_pod_count,
            is_enabled: r.is_enabled,
            host_info_json: r.host_info.and_then(|v| serde_json::to_string(&v).ok()),
            last_heartbeat: r.last_heartbeat,
            available_agents: r.available_agents,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerListResponseDto {
    pub runners: Vec<RunnerDto>,
    pub latest_runner_version: Option<String>,
}

impl From<RunnerListResponse> for RunnerListResponseDto {
    fn from(r: RunnerListResponse) -> Self {
        Self {
            runners: r.runners.into_iter().map(RunnerDto::from).collect(),
            latest_runner_version: r.latest_runner_version,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct GrpcRegistrationTokenDto {
    pub id: i64,
    pub name: Option<String>,
    pub token: Option<String>,
    pub max_uses: Option<i32>,
    pub used_count: Option<i32>,
    pub expires_at: Option<String>,
    pub created_at: Option<String>,
}

impl From<GRPCRegistrationToken> for GrpcRegistrationTokenDto {
    fn from(t: GRPCRegistrationToken) -> Self {
        Self {
            id: t.id,
            name: t.name,
            token: t.token,
            max_uses: t.max_uses,
            used_count: t.used_count,
            expires_at: t.expires_at,
            created_at: t.created_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerTokenListResponseDto {
    pub tokens: Vec<GrpcRegistrationTokenDto>,
}

impl From<RunnerTokenListResponse> for RunnerTokenListResponseDto {
    fn from(r: RunnerTokenListResponse) -> Self {
        Self {
            tokens: r
                .tokens
                .into_iter()
                .map(GrpcRegistrationTokenDto::from)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerAuthStatusDto {
    pub status: String,
    pub runner_id: Option<i64>,
    pub organization_slug: Option<String>,
}

impl From<RunnerAuthStatus> for RunnerAuthStatusDto {
    fn from(s: RunnerAuthStatus) -> Self {
        Self {
            status: s.status,
            runner_id: s.runner_id,
            organization_slug: s.organization_slug,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UpdateRunnerRequestDto {
    pub description: Option<String>,
    pub max_concurrent_pods: Option<i32>,
    pub is_enabled: Option<bool>,
    pub visibility: Option<String>,
}

impl From<UpdateRunnerRequestDto> for UpdateRunnerRequest {
    fn from(d: UpdateRunnerRequestDto) -> Self {
        Self {
            description: d.description,
            max_concurrent_pods: d.max_concurrent_pods,
            is_enabled: d.is_enabled,
            visibility: d.visibility,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct CreateRunnerTokenRequestDto {
    pub name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub max_uses: Option<i32>,
    pub expires_in_days: Option<i64>,
}

impl From<CreateRunnerTokenRequestDto> for CreateRunnerTokenRequest {
    fn from(d: CreateRunnerTokenRequestDto) -> Self {
        Self {
            name: d.name,
            labels: d.labels,
            max_uses: d.max_uses,
            expires_in_days: d.expires_in_days,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UpgradeRunnerRequestDto {
    pub target_version: Option<String>,
    pub force: Option<bool>,
}

impl From<UpgradeRunnerRequestDto> for UpgradeRunnerRequest {
    fn from(d: UpgradeRunnerRequestDto) -> Self {
        Self {
            target_version: d.target_version,
            force: d.force,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct AuthorizeRunnerRequestDto {
    pub auth_key: String,
    pub node_id: Option<String>,
}

impl From<AuthorizeRunnerRequestDto> for AuthorizeRunnerRequest {
    fn from(d: AuthorizeRunnerRequestDto) -> Self {
        Self {
            auth_key: d.auth_key,
            node_id: d.node_id,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerLogDto {
    pub id: i64,
    pub runner_id: i64,
    pub filename: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<String>,
}

impl From<RunnerLog> for RunnerLogDto {
    fn from(l: RunnerLog) -> Self {
        Self {
            id: l.id,
            runner_id: l.runner_id,
            filename: l.filename,
            url: l.url,
            created_at: l.created_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RunnerLogListResponseDto {
    pub logs: Vec<RunnerLogDto>,
}

impl From<RunnerLogListResponse> for RunnerLogListResponseDto {
    fn from(r: RunnerLogListResponse) -> Self {
        Self {
            logs: r.logs.into_iter().map(RunnerLogDto::from).collect(),
        }
    }
}

// ── Repository ────────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct RepositoryDto {
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

impl From<Repository> for RepositoryDto {
    fn from(r: Repository) -> Self {
        Self {
            id: r.id,
            name: r.name,
            slug: r.slug,
            provider_type: r.provider_type,
            provider_base_url: r.provider_base_url,
            http_clone_url: r.http_clone_url,
            ssh_clone_url: r.ssh_clone_url,
            external_id: r.external_id,
            default_branch: r.default_branch,
            ticket_prefix: r.ticket_prefix,
            visibility: r.visibility,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RepositoryListResponseDto {
    pub repositories: Vec<RepositoryDto>,
}

impl From<RepositoryListResponse> for RepositoryListResponseDto {
    fn from(r: RepositoryListResponse) -> Self {
        Self {
            repositories: r.repositories.into_iter().map(RepositoryDto::from).collect(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct CreateRepositoryRequestDto {
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

impl From<CreateRepositoryRequestDto> for CreateRepositoryRequest {
    fn from(d: CreateRepositoryRequestDto) -> Self {
        Self {
            provider_type: d.provider_type,
            provider_base_url: d.provider_base_url,
            http_clone_url: d.http_clone_url,
            ssh_clone_url: d.ssh_clone_url,
            external_id: d.external_id,
            name: d.name,
            slug: d.slug,
            default_branch: d.default_branch,
            ticket_prefix: d.ticket_prefix,
            visibility: d.visibility,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UpdateRepositoryRequestDto {
    pub name: Option<String>,
    pub default_branch: Option<String>,
    pub ticket_prefix: Option<String>,
    pub is_active: Option<bool>,
    pub http_clone_url: Option<String>,
    pub ssh_clone_url: Option<String>,
}

impl From<UpdateRepositoryRequestDto> for UpdateRepositoryRequest {
    fn from(d: UpdateRepositoryRequestDto) -> Self {
        Self {
            name: d.name,
            default_branch: d.default_branch,
            ticket_prefix: d.ticket_prefix,
            is_active: d.is_active,
            http_clone_url: d.http_clone_url,
            ssh_clone_url: d.ssh_clone_url,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct BranchDto {
    pub name: String,
    pub is_default: Option<bool>,
    pub last_commit: Option<String>,
}

impl From<Branch> for BranchDto {
    fn from(b: Branch) -> Self {
        Self {
            name: b.name,
            is_default: b.is_default,
            last_commit: b.last_commit,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct WebhookStatusDto {
    pub is_configured: Option<bool>,
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
}

impl From<WebhookStatus> for WebhookStatusDto {
    fn from(w: WebhookStatus) -> Self {
        Self {
            is_configured: w.is_configured,
            url: w.url,
            events: w.events,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct WebhookSecretDto {
    pub secret: String,
}

impl From<WebhookSecret> for WebhookSecretDto {
    fn from(w: WebhookSecret) -> Self {
        Self { secret: w.secret }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct RepositoryMergeRequestDto {
    pub id: i64,
    pub title: Option<String>,
    pub state: Option<String>,
    pub source_branch: Option<String>,
    pub target_branch: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<String>,
}

impl From<RepositoryMergeRequest> for RepositoryMergeRequestDto {
    fn from(m: RepositoryMergeRequest) -> Self {
        Self {
            id: m.id,
            title: m.title,
            state: m.state,
            source_branch: m.source_branch,
            target_branch: m.target_branch,
            author: m.author,
            url: m.url,
            created_at: m.created_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct MergeRequestListResponseDto {
    pub merge_requests: Vec<RepositoryMergeRequestDto>,
}

impl From<MergeRequestListResponse> for MergeRequestListResponseDto {
    fn from(r: MergeRequestListResponse) -> Self {
        Self {
            merge_requests: r
                .merge_requests
                .into_iter()
                .map(RepositoryMergeRequestDto::from)
                .collect(),
        }
    }
}
