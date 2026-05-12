package runnerconnect

import (
	"context"
	"errors"
	"log/slog"
	"time"

	"connectrpc.com/connect"
	"github.com/google/uuid"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"github.com/anthropics/agentsmesh/backend/internal/api/connect/interceptors"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	runner "github.com/anthropics/agentsmesh/backend/internal/service/runner"
	"github.com/anthropics/agentsmesh/backend/pkg/policy"
	runnerapiv1 "github.com/anthropics/agentsmesh/proto/gen/go/runner_api/v1"
)

// ListRunners — REST analogue: GET /api/v1/organizations/:slug/runners.
// Filters by RunnerPolicy.ListFilter (private runners exclude non-owners).
// Returns the uniform list envelope plus the optional latest_runner_version
// envelope hint (DEVIATION from §8, kept because the value is global).
func (s *Server) ListRunners(
	ctx context.Context, req *connect.Request[runnerapiv1.ListRunnersRequest],
) (*connect.Response[runnerapiv1.ListRunnersResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	filter := listFilter(tenant)
	runners, err := s.runnerSvc.ListRunners(ctx, tenant.OrganizationID, filter.VisibilityUserID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	resp := &runnerapiv1.ListRunnersResponse{
		Items:  toProtoRunners(runners),
		Total:  int64(len(runners)),
		Limit:  req.Msg.GetLimit(),
		Offset: req.Msg.GetOffset(),
	}
	if s.versionChecker != nil {
		if v := s.versionChecker.GetLatestVersion(ctx); v != "" {
			resp.LatestRunnerVersion = &v
		}
	}
	return connect.NewResponse(resp), nil
}

func (s *Server) ListAvailableRunners(
	ctx context.Context, req *connect.Request[runnerapiv1.ListAvailableRunnersRequest],
) (*connect.Response[runnerapiv1.ListAvailableRunnersResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	filter := listFilter(tenant)
	runners, err := s.runnerSvc.ListAvailableRunners(ctx, tenant.OrganizationID, filter.VisibilityUserID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	return connect.NewResponse(&runnerapiv1.ListAvailableRunnersResponse{
		Items: toProtoRunners(runners),
		Total: int64(len(runners)),
	}), nil
}

func (s *Server) GetRunner(
	ctx context.Context, req *connect.Request[runnerapiv1.GetRunnerRequest],
) (*connect.Response[runnerapiv1.GetRunnerResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	runnerID := req.Msg.GetId()

	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}

	resp := &runnerapiv1.GetRunnerResponse{Runner: toProtoRunner(r)}
	if s.podCoordinator != nil {
		resp.RelayConnections = toProtoRelayConnections(s.podCoordinator.GetRelayConnections(runnerID))
	}
	if s.versionChecker != nil {
		if v := s.versionChecker.GetLatestVersion(ctx); v != "" {
			resp.LatestRunnerVersion = &v
		}
	}
	return connect.NewResponse(resp), nil
}

func (s *Server) UpdateRunner(
	ctx context.Context, req *connect.Request[runnerapiv1.UpdateRunnerRequest],
) (*connect.Response[runnerapiv1.Runner], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("organization admin role required"))
	}

	runnerID := req.Msg.GetId()
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	if !policy.RunnerPolicy.AllowWrite(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}

	input := runner.RunnerUpdateInput{}
	if req.Msg.Description != nil {
		v := req.Msg.GetDescription()
		input.Description = &v
	}
	if req.Msg.MaxConcurrentPods != nil {
		v := int(req.Msg.GetMaxConcurrentPods())
		input.MaxConcurrentPods = &v
	}
	if req.Msg.IsEnabled != nil {
		v := req.Msg.GetIsEnabled()
		input.IsEnabled = &v
	}
	if req.Msg.Visibility != nil {
		v := req.Msg.GetVisibility()
		input.Visibility = &v
	}
	// TagsUpdate semantics: presence of the message means "set to values",
	// absence means "leave unchanged". service-layer treats nil = no-op.
	if req.Msg.GetTags() != nil {
		input.Tags = req.Msg.GetTags().GetValues()
		if input.Tags == nil {
			// Distinguish "explicit empty" from "no update" — service uses
			// len(Tags) == 0 with non-nil slice to mean "clear".
			input.Tags = []string{}
		}
	}

	updated, err := s.runnerSvc.UpdateRunner(ctx, runnerID, input)
	if err != nil {
		return nil, mapServiceError(err)
	}
	return connect.NewResponse(toProtoRunner(updated)), nil
}

func (s *Server) DeleteRunner(
	ctx context.Context, req *connect.Request[runnerapiv1.DeleteRunnerRequest],
) (*connect.Response[runnerapiv1.DeleteRunnerResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.AllowAdmin(sub, tenant.OrganizationID) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("organization admin role required"))
	}

	runnerID := req.Msg.GetId()
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	if !policy.RunnerPolicy.AllowWrite(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}

	if err := s.runnerSvc.DeleteRunner(ctx, runnerID); err != nil {
		return nil, mapServiceError(err)
	}
	return connect.NewResponse(&runnerapiv1.DeleteRunnerResponse{}), nil
}

func (s *Server) UpgradeRunner(
	ctx context.Context, req *connect.Request[runnerapiv1.UpgradeRunnerRequest],
) (*connect.Response[runnerapiv1.UpgradeRunnerResponse], error) {
	if s.upgradeSender == nil {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("upgrade service not configured"))
	}
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	runnerID := req.Msg.GetId()
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowWrite(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}
	if req.Msg.GetForce() {
		slog.WarnContext(ctx, "Deprecated 'force' field received - ignored", "runner_id", runnerID, "user_id", tenant.UserID)
	}
	if !s.upgradeSender.IsConnected(runnerID) {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("runner is not connected"))
	}
	requestID := uuid.New().String()
	// force=true: Poddaemon keeps pods alive across restart.
	if err := s.upgradeSender.SendUpgradeRunner(runnerID, requestID, req.Msg.GetTargetVersion(), true); err != nil {
		if st, ok := status.FromError(err); ok && st.Code() == codes.NotFound {
			return nil, connect.NewError(connect.CodeUnavailable, errors.New("runner disconnected before command could be sent"))
		}
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	slog.InfoContext(ctx, "Runner upgrade initiated",
		"runner_id", runnerID,
		"request_id", requestID,
		"target_version", req.Msg.GetTargetVersion(),
		"active_pod_count", r.CurrentPods,
		"user_id", tenant.UserID,
		"org_id", tenant.OrganizationID,
	)
	return connect.NewResponse(&runnerapiv1.UpgradeRunnerResponse{
		RequestId: requestID,
		Message:   "Upgrade command sent to runner",
	}), nil
}

func (s *Server) RequestLogUpload(
	ctx context.Context, req *connect.Request[runnerapiv1.RequestLogUploadRequest],
) (*connect.Response[runnerapiv1.RequestLogUploadResponse], error) {
	if s.logUploadSender == nil || s.logUploadSvc == nil {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("log upload service not configured"))
	}
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	runnerID := req.Msg.GetId()
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}
	if !s.logUploadSender.IsConnected(runnerID) {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("runner is not connected"))
	}
	uploadReq, err := s.logUploadSvc.RequestUpload(ctx, tenant.OrganizationID, runnerID, tenant.UserID)
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	if err := s.logUploadSender.SendUploadLogs(runnerID, uploadReq.RequestID, uploadReq.PresignedURL, uploadReq.ExpiresAt); err != nil {
		s.logUploadSvc.MarkFailed(uploadReq.RequestID, "failed to send command to runner")
		if st, ok := status.FromError(err); ok && st.Code() == codes.NotFound {
			return nil, connect.NewError(connect.CodeUnavailable, errors.New("runner disconnected before command could be sent"))
		}
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	slog.InfoContext(ctx, "Log upload requested",
		"runner_id", runnerID,
		"request_id", uploadReq.RequestID,
		"user_id", tenant.UserID,
		"org_id", tenant.OrganizationID,
	)
	return connect.NewResponse(&runnerapiv1.RequestLogUploadResponse{
		RequestId: uploadReq.RequestID,
		Message:   "Log upload command sent to runner",
	}), nil
}

func (s *Server) ListRunnerLogs(
	ctx context.Context, req *connect.Request[runnerapiv1.ListRunnerLogsRequest],
) (*connect.Response[runnerapiv1.ListRunnerLogsResponse], error) {
	if s.logUploadSvc == nil {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("log upload service not configured"))
	}
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	runnerID := req.Msg.GetId()
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}
	limit := int(req.Msg.GetLimit())
	if limit <= 0 {
		limit = 20
	}
	offset := int(req.Msg.GetOffset())
	logs, err := s.logUploadSvc.ListByRunner(ctx, tenant.OrganizationID, runnerID, limit, offset)
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	items := make([]*runnerapiv1.RunnerLog, 0, len(logs))
	for _, l := range logs {
		items = append(items, toProtoLogEntry(l))
	}
	return connect.NewResponse(&runnerapiv1.ListRunnerLogsResponse{
		Items:  items,
		Total:  int64(len(items)),
		Limit:  int32(limit),
		Offset: int32(offset),
	}), nil
}

func (s *Server) QuerySandboxes(
	ctx context.Context, req *connect.Request[runnerapiv1.QuerySandboxesRequest],
) (*connect.Response[runnerapiv1.QuerySandboxesResponse], error) {
	if s.sandboxQuerySvc == nil {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("sandbox query service not configured"))
	}
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	runnerID := req.Msg.GetId()
	podKeys := req.Msg.GetPodKeys()
	if len(podKeys) == 0 || len(podKeys) > 100 {
		return nil, connect.NewError(connect.CodeInvalidArgument, errors.New("pod_keys must contain 1-100 entries"))
	}
	r, err := s.runnerSvc.GetRunner(ctx, runnerID)
	if err != nil {
		return nil, mapServiceError(err)
	}
	sub := policy.NewSubject(tenant.OrganizationID, tenant.UserID, tenant.UserRole)
	if !policy.RunnerPolicy.AllowRead(sub, policy.VisibleResource(
		r.OrganizationID, r.RegisteredByUserID, r.Visibility,
	)) {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("forbidden"))
	}
	if !s.sandboxQuerySvc.IsConnected(runnerID) {
		return nil, connect.NewError(connect.CodeUnavailable, errors.New("runner is not connected"))
	}
	result, err := s.sandboxQuerySvc.QuerySandboxes(ctx, runnerID, podKeys)
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	return connect.NewResponse(&runnerapiv1.QuerySandboxesResponse{
		Sandboxes: toProtoSandboxStatuses(result.Sandboxes),
		Error:     result.Error,
	}), nil
}

// ---- Tokens ----

func (s *Server) CreateRunnerToken(
	ctx context.Context, req *connect.Request[runnerapiv1.CreateRunnerTokenRequest],
) (*connect.Response[runnerapiv1.RunnerToken], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	if tenant.UserRole != "owner" && tenant.UserRole != "admin" {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("organization admin role required"))
	}

	// Convert labels from []string ("k=v") → map[string]string. The legacy
	// REST handler accepted both shapes (map JSON) so the wasm callers send
	// a slice; both are translated identically here.
	labels := labelsToMap(req.Msg.GetLabels())
	expiresInSeconds := 0
	if req.Msg.ExpiresInDays != nil {
		expiresInSeconds = int(req.Msg.GetExpiresInDays() * 86400)
	}
	maxUses := int(req.Msg.GetMaxUses())

	gen, err := s.runnerSvc.GenerateGRPCRegistrationToken(
		ctx,
		tenant.OrganizationID,
		tenant.UserID,
		&runner.GenerateGRPCRegistrationTokenRequest{
			Name:      req.Msg.GetName(),
			Labels:    labels,
			SingleUse: maxUses == 0 || maxUses == 1,
			MaxUses:   maxUses,
			ExpiresIn: expiresInSeconds,
		},
		s.baseURL,
	)
	if err != nil {
		return nil, mapServiceError(err)
	}

	token := gen.Token
	expiresAt := gen.ExpiresAt.UTC().Format(time.RFC3339)
	out := &runnerapiv1.RunnerToken{
		Id:        gen.ID,
		Token:     &token,
		ExpiresAt: &expiresAt,
	}
	return connect.NewResponse(out), nil
}

func (s *Server) ListRunnerTokens(
	ctx context.Context, req *connect.Request[runnerapiv1.ListRunnerTokensRequest],
) (*connect.Response[runnerapiv1.ListRunnerTokensResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	if tenant.UserRole != "owner" && tenant.UserRole != "admin" {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("organization admin role required"))
	}
	tokens, err := s.runnerSvc.ListGRPCRegistrationTokens(ctx, tenant.OrganizationID)
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	items := make([]*runnerapiv1.RunnerToken, 0, len(tokens))
	for _, t := range tokens {
		items = append(items, toProtoToken(t))
	}
	return connect.NewResponse(&runnerapiv1.ListRunnerTokensResponse{
		Items: items,
		Total: int64(len(items)),
	}), nil
}

func (s *Server) DeleteRunnerToken(
	ctx context.Context, req *connect.Request[runnerapiv1.DeleteRunnerTokenRequest],
) (*connect.Response[runnerapiv1.DeleteRunnerTokenResponse], error) {
	ctx, _, err := interceptors.ResolveOrgScope(ctx, req.Msg, s.orgSvc)
	if err != nil {
		return nil, err
	}
	tenant := middleware.GetTenant(ctx)
	if tenant.UserRole != "owner" && tenant.UserRole != "admin" {
		return nil, connect.NewError(connect.CodePermissionDenied, errors.New("organization admin role required"))
	}
	if err := s.runnerSvc.DeleteGRPCRegistrationToken(ctx, req.Msg.GetId(), tenant.OrganizationID); err != nil {
		return nil, mapServiceError(err)
	}
	return connect.NewResponse(&runnerapiv1.DeleteRunnerTokenResponse{}), nil
}

// labelsToMap converts a "k=v" slice into a map. Empty / malformed entries
// are silently dropped to match the REST handler's tolerant binding.
func labelsToMap(labels []string) map[string]string {
	if len(labels) == 0 {
		return nil
	}
	out := make(map[string]string, len(labels))
	for _, l := range labels {
		for i := 0; i < len(l); i++ {
			if l[i] == '=' {
				out[l[:i]] = l[i+1:]
				break
			}
		}
	}
	return out
}
