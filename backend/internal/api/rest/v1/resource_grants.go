package v1

import (
	"net/http"
	"strconv"

	"github.com/anthropics/agentsmesh/backend/internal/domain/grant"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	grantservice "github.com/anthropics/agentsmesh/backend/internal/service/grant"
	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/anthropics/agentsmesh/backend/pkg/policy"
	"github.com/gin-gonic/gin"
)

type grantAccessRequest struct {
	UserID int64 `json:"user_id" binding:"required"`
}

// ListPodGrants lists grants for a pod.
func (h *PodHandler) ListPodGrants(c *gin.Context) {
	podKey := c.Param("key")

	pod, err := h.podService.GetPod(c.Request.Context(), podKey)
	if err != nil {
		apierr.ResourceNotFound(c, "Pod not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.PodPolicy.AllowWrite(sub, policy.PodResource(pod.OrganizationID, pod.CreatedByID)) {
		apierr.ForbiddenAccess(c)
		return
	}

	grants, err := h.grantService.ListGrants(c.Request.Context(), grant.TypePod, podKey)
	if err != nil {
		apierr.InternalError(c, "Failed to list grants")
		return
	}
	c.JSON(http.StatusOK, gin.H{"grants": grants})
}

// GrantPodAccess grants access to a pod.
func (h *PodHandler) GrantPodAccess(c *gin.Context) {
	podKey := c.Param("key")
	var req grantAccessRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	pod, err := h.podService.GetPod(c.Request.Context(), podKey)
	if err != nil {
		apierr.ResourceNotFound(c, "Pod not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.PodPolicy.AllowWrite(sub, policy.PodResource(pod.OrganizationID, pod.CreatedByID)) {
		apierr.ForbiddenAccess(c)
		return
	}

	g, err := h.grantService.GrantAccess(c.Request.Context(),
		tenant.OrganizationID, grant.TypePod, podKey, req.UserID, tenant.UserID)
	if err != nil {
		handleGrantError(c, err)
		return
	}
	c.JSON(http.StatusOK, gin.H{"grant": g})
}

// RevokePodGrant revokes a pod grant.
func (h *PodHandler) RevokePodGrant(c *gin.Context) {
	podKey := c.Param("key")
	grantID, err := strconv.ParseInt(c.Param("grant_id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid grant ID")
		return
	}

	pod, err := h.podService.GetPod(c.Request.Context(), podKey)
	if err != nil {
		apierr.ResourceNotFound(c, "Pod not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.PodPolicy.AllowWrite(sub, policy.PodResource(pod.OrganizationID, pod.CreatedByID)) {
		apierr.ForbiddenAccess(c)
		return
	}

	if err := h.grantService.RevokeAccess(c.Request.Context(), grantID); err != nil {
		apierr.InternalError(c, "Failed to revoke grant")
		return
	}
	c.JSON(http.StatusOK, gin.H{"message": "Grant revoked"})
}

// ListRunnerGrants lists grants for a runner.
func (h *RunnerHandler) ListRunnerGrants(c *gin.Context) {
	runnerID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid runner ID")
		return
	}

	r, err := h.runnerService.GetRunner(c.Request.Context(), runnerID)
	if err != nil {
		apierr.ResourceNotFound(c, "Runner not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	grants, err := h.grantService.ListGrants(c.Request.Context(), grant.TypeRunner, strconv.FormatInt(runnerID, 10))
	if err != nil {
		apierr.InternalError(c, "Failed to list grants")
		return
	}
	c.JSON(http.StatusOK, gin.H{"grants": grants})
}

// GrantRunnerAccess grants access to a runner.
func (h *RunnerHandler) GrantRunnerAccess(c *gin.Context) {
	runnerID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid runner ID")
		return
	}

	var req grantAccessRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	r, err := h.runnerService.GetRunner(c.Request.Context(), runnerID)
	if err != nil {
		apierr.ResourceNotFound(c, "Runner not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		apierr.ForbiddenAdmin(c)
		return
	}
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	g, err := h.grantService.GrantAccess(c.Request.Context(),
		tenant.OrganizationID, grant.TypeRunner, strconv.FormatInt(runnerID, 10), req.UserID, tenant.UserID)
	if err != nil {
		handleGrantError(c, err)
		return
	}
	c.JSON(http.StatusOK, gin.H{"grant": g})
}

// RevokeRunnerGrant revokes a runner grant.
func (h *RunnerHandler) RevokeRunnerGrant(c *gin.Context) {
	runnerID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid runner ID")
		return
	}

	r, err := h.runnerService.GetRunner(c.Request.Context(), runnerID)
	if err != nil {
		apierr.ResourceNotFound(c, "Runner not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		apierr.ForbiddenAdmin(c)
		return
	}
	if !policy.RunnerPolicy.AllowWrite(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	grantID, err := strconv.ParseInt(c.Param("grant_id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid grant ID")
		return
	}

	if err := h.grantService.RevokeAccess(c.Request.Context(), grantID); err != nil {
		apierr.InternalError(c, "Failed to revoke grant")
		return
	}
	c.JSON(http.StatusOK, gin.H{"message": "Grant revoked"})
}

// ListRepositoryGrants lists grants for a repository.
func (h *RepositoryHandler) ListRepositoryGrants(c *gin.Context) {
	repoID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid repository ID")
		return
	}

	repo, err := h.repositoryService.GetByID(c.Request.Context(), repoID)
	if err != nil {
		apierr.ResourceNotFound(c, "Repository not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RepositoryPolicy.AllowRead(sub, policy.VisibleResource(
		repo.OrganizationID, repo.ImportedByUserID, repo.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	grants, err := h.grantService.ListGrants(c.Request.Context(), grant.TypeRepository, strconv.FormatInt(repoID, 10))
	if err != nil {
		apierr.InternalError(c, "Failed to list grants")
		return
	}
	c.JSON(http.StatusOK, gin.H{"grants": grants})
}

// GrantRepositoryAccess grants access to a repository.
func (h *RepositoryHandler) GrantRepositoryAccess(c *gin.Context) {
	repoID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid repository ID")
		return
	}

	var req grantAccessRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		apierr.ValidationError(c, err.Error())
		return
	}

	repo, err := h.repositoryService.GetByID(c.Request.Context(), repoID)
	if err != nil {
		apierr.ResourceNotFound(c, "Repository not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		apierr.ForbiddenAdmin(c)
		return
	}
	if !policy.RepositoryPolicy.AllowWrite(sub, policy.VisibleResource(
		repo.OrganizationID, repo.ImportedByUserID, repo.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	g, err := h.grantService.GrantAccess(c.Request.Context(),
		tenant.OrganizationID, grant.TypeRepository, strconv.FormatInt(repoID, 10), req.UserID, tenant.UserID)
	if err != nil {
		handleGrantError(c, err)
		return
	}
	c.JSON(http.StatusOK, gin.H{"grant": g})
}

// RevokeRepositoryGrant revokes a repository grant.
func (h *RepositoryHandler) RevokeRepositoryGrant(c *gin.Context) {
	repoID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid repository ID")
		return
	}

	repo, err := h.repositoryService.GetByID(c.Request.Context(), repoID)
	if err != nil {
		apierr.ResourceNotFound(c, "Repository not found")
		return
	}

	tenant := middleware.GetTenant(c)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		apierr.ForbiddenAdmin(c)
		return
	}
	if !policy.RepositoryPolicy.AllowWrite(sub, policy.VisibleResource(
		repo.OrganizationID, repo.ImportedByUserID, repo.Visibility,
	)) {
		apierr.ForbiddenAccess(c)
		return
	}

	grantID, err := strconv.ParseInt(c.Param("grant_id"), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid grant ID")
		return
	}

	if err := h.grantService.RevokeAccess(c.Request.Context(), grantID); err != nil {
		apierr.InternalError(c, "Failed to revoke grant")
		return
	}
	c.JSON(http.StatusOK, gin.H{"message": "Grant revoked"})
}

func handleGrantError(c *gin.Context, err error) {
	switch err {
	case grantservice.ErrSelfGrant:
		apierr.BadRequest(c, apierr.VALIDATION_FAILED, "Cannot grant access to yourself")
	case grantservice.ErrInvalidType:
		apierr.BadRequest(c, apierr.VALIDATION_FAILED, "Invalid resource type")
	default:
		apierr.InternalError(c, "Failed to grant access")
	}
}
