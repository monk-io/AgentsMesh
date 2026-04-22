use agentsmesh_types::{
    CreateInvitationRequest, CreateResourceGrantRequest, Invitation, InvitationListResponse,
    PresignRequest, PresignResponse, ResourceGrant, ResourceGrantListResponse,
    ResourceGrantResponse, ResourceGrantUserBrief,
};

// ── Invitation ────────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct InvitationDto {
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

impl From<Invitation> for InvitationDto {
    fn from(i: Invitation) -> Self {
        Self {
            id: i.id,
            email: i.email,
            role: i.role,
            status: i.status,
            token: i.token,
            organization_name: i.organization_name,
            organization_slug: i.organization_slug,
            inviter_name: i.inviter_name,
            expires_at: i.expires_at,
            created_at: i.created_at,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct InvitationListResponseDto {
    pub invitations: Vec<InvitationDto>,
}

impl From<InvitationListResponse> for InvitationListResponseDto {
    fn from(r: InvitationListResponse) -> Self {
        Self {
            invitations: r.invitations.into_iter().map(InvitationDto::from).collect(),
        }
    }
}

pub(crate) fn create_invitation_req(email: String, role: String) -> CreateInvitationRequest {
    CreateInvitationRequest { email, role }
}

// ── File presign ──────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct PresignRequestDto {
    pub filename: String,
    pub content_type: String,
    pub size: i64,
}

impl From<PresignRequestDto> for PresignRequest {
    fn from(d: PresignRequestDto) -> Self {
        Self {
            filename: d.filename,
            content_type: d.content_type,
            size: d.size,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct PresignResponseDto {
    pub put_url: String,
    pub get_url: String,
}

impl From<PresignResponse> for PresignResponseDto {
    fn from(r: PresignResponse) -> Self {
        Self {
            put_url: r.put_url,
            get_url: r.get_url,
        }
    }
}

// ── Resource Grant ────────────────────────────────────────

#[derive(Clone, Debug, uniffi::Record)]
pub struct ResourceGrantUserBriefDto {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub name: Option<String>,
}

impl From<ResourceGrantUserBrief> for ResourceGrantUserBriefDto {
    fn from(u: ResourceGrantUserBrief) -> Self {
        Self {
            id: u.id,
            email: u.email,
            username: u.username,
            name: u.name,
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct ResourceGrantDto {
    pub id: i64,
    pub resource_type: String,
    pub resource_id: String,
    pub user_id: i64,
    pub granted_by: i64,
    pub created_at: String,
    pub user: Option<ResourceGrantUserBriefDto>,
    pub granted_by_user: Option<ResourceGrantUserBriefDto>,
}

impl From<ResourceGrant> for ResourceGrantDto {
    fn from(g: ResourceGrant) -> Self {
        Self {
            id: g.id,
            resource_type: g.resource_type,
            resource_id: g.resource_id,
            user_id: g.user_id,
            granted_by: g.granted_by,
            created_at: g.created_at,
            user: g.user.map(Into::into),
            granted_by_user: g.granted_by_user.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct ResourceGrantListResponseDto {
    pub grants: Vec<ResourceGrantDto>,
}

impl From<ResourceGrantListResponse> for ResourceGrantListResponseDto {
    fn from(r: ResourceGrantListResponse) -> Self {
        Self {
            grants: r.grants.into_iter().map(ResourceGrantDto::from).collect(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct ResourceGrantResponseDto {
    pub grant: ResourceGrantDto,
}

impl From<ResourceGrantResponse> for ResourceGrantResponseDto {
    fn from(r: ResourceGrantResponse) -> Self {
        Self {
            grant: r.grant.into(),
        }
    }
}

pub(crate) fn create_resource_grant_req(user_id: i64) -> CreateResourceGrantRequest {
    CreateResourceGrantRequest { user_id }
}
