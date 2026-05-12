use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_org_v1 as org_proto;

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/org/. Procedure paths derive from
// `proto.org.v1.OrgService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn list_my_orgs_connect(
        &self,
        req: &org_proto::ListMyOrgsRequest,
    ) -> Result<org_proto::ListMyOrgsResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/ListMyOrgs", req).await
    }

    pub async fn create_org_connect(
        &self,
        req: &org_proto::CreateOrgRequest,
    ) -> Result<org_proto::Organization, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/CreateOrg", req).await
    }

    pub async fn create_personal_org_connect(
        &self,
        req: &org_proto::CreatePersonalOrgRequest,
    ) -> Result<org_proto::Organization, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/CreatePersonalOrg", req).await
    }

    pub async fn get_org_connect(
        &self,
        req: &org_proto::GetOrgRequest,
    ) -> Result<org_proto::Organization, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/GetOrg", req).await
    }

    pub async fn update_org_connect(
        &self,
        req: &org_proto::UpdateOrgRequest,
    ) -> Result<org_proto::Organization, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/UpdateOrg", req).await
    }

    pub async fn delete_org_connect(
        &self,
        req: &org_proto::DeleteOrgRequest,
    ) -> Result<org_proto::DeleteOrgResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/DeleteOrg", req).await
    }

    pub async fn list_members_connect(
        &self,
        req: &org_proto::ListMembersRequest,
    ) -> Result<org_proto::ListMembersResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/ListMembers", req).await
    }

    pub async fn invite_member_connect(
        &self,
        req: &org_proto::InviteMemberRequest,
    ) -> Result<org_proto::InviteMemberResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/InviteMember", req).await
    }

    pub async fn remove_member_connect(
        &self,
        req: &org_proto::RemoveMemberRequest,
    ) -> Result<org_proto::RemoveMemberResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/RemoveMember", req).await
    }

    pub async fn update_member_role_connect(
        &self,
        req: &org_proto::UpdateMemberRoleRequest,
    ) -> Result<org_proto::UpdateMemberRoleResponse, ApiError> {
        connect_call(self, "/proto.org.v1.OrgService/UpdateMemberRole", req).await
    }
}

// =============================================================================
// Legacy REST methods — preserved for dual-track migration.
// =============================================================================

impl ApiClient {
    pub async fn list_organizations(&self) -> Result<OrganizationListResponse, ApiError> {
        self.get("/api/v1/orgs").await
    }

    pub async fn get_organization(&self, slug: &str) -> Result<Organization, ApiError> {
        self.get_resource(&format!("/api/v1/orgs/{slug}"), "organization").await
    }

    pub async fn create_organization(
        &self,
        data: &CreateOrganizationRequest,
    ) -> Result<Organization, ApiError> {
        self.post_resource("/api/v1/orgs", data, "organization").await
    }

    pub async fn update_organization(
        &self,
        slug: &str,
        data: &UpdateOrganizationRequest,
    ) -> Result<Organization, ApiError> {
        self.put_resource(&format!("/api/v1/orgs/{slug}"), data, "organization").await
    }

    pub async fn delete_organization(&self, slug: &str) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/orgs/{slug}")).await
    }

    pub async fn list_org_members(&self, slug: &str) -> Result<MemberListResponse, ApiError> {
        self.get(&format!("/api/v1/orgs/{slug}/members")).await
    }

    pub async fn invite_org_member(
        &self,
        slug: &str,
        data: &InviteMemberRequest,
    ) -> Result<OrgMember, ApiError> {
        self.post(&format!("/api/v1/orgs/{slug}/members"), data)
            .await
    }

    pub async fn remove_org_member(
        &self,
        slug: &str,
        user_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/orgs/{slug}/members/{user_id}"))
            .await
    }

    pub async fn update_org_member_role(
        &self,
        slug: &str,
        user_id: i64,
        data: &UpdateMemberRoleRequest,
    ) -> Result<OrgMember, ApiError> {
        self.put(
            &format!("/api/v1/orgs/{slug}/members/{user_id}"),
            data,
        )
        .await
    }
}
