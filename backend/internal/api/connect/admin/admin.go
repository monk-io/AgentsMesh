// Package adminconnect hosts Connect-RPC handlers for the platform-admin
// management plane (system administrators with `is_system_admin = true`).
//
// Mirrors backend/internal/api/rest/v1/admin/* one-for-one for the batch
// 1 surface: dashboard, users, organizations, runners, audit logs. REST
// handlers ship in parallel during the migration window — drop after
// every web-admin call-site flips lane.
//
// Auth deviation vs REST: REST chains AuthMiddleware (cmd parses bearer)
// + AdminMiddleware (db lookup + is_system_admin gate). Connect wires
// the equivalent via NewAuthInterceptor + NewAdminInterceptor (see
// backend/internal/api/connect/interceptors/admin.go). The combined
// pipeline preserves all 3 REST rejection paths:
//   * no bearer / invalid JWT      → CodeUnauthenticated
//   * is_system_admin = false      → CodePermissionDenied
//   * is_active       = false      → CodePermissionDenied
// Audit-log writer reads admin user id from interceptors.AdminContext —
// no `gin.Context` dependency leaks into business logic.
package adminconnect

import (
	"net/http"

	"connectrpc.com/connect"

	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
)

const ServiceName = "proto.admin.v1.AdminService"

const (
	GetDashboardStatsProcedure = "/" + ServiceName + "/GetDashboardStats"

	ListUsersProcedure         = "/" + ServiceName + "/ListUsers"
	GetUserProcedure           = "/" + ServiceName + "/GetUser"
	UpdateUserProcedure        = "/" + ServiceName + "/UpdateUser"
	DisableUserProcedure       = "/" + ServiceName + "/DisableUser"
	EnableUserProcedure        = "/" + ServiceName + "/EnableUser"
	GrantAdminProcedure        = "/" + ServiceName + "/GrantAdmin"
	RevokeAdminProcedure       = "/" + ServiceName + "/RevokeAdmin"
	VerifyUserEmailProcedure   = "/" + ServiceName + "/VerifyUserEmail"
	UnverifyUserEmailProcedure = "/" + ServiceName + "/UnverifyUserEmail"

	ListOrganizationsProcedure      = "/" + ServiceName + "/ListOrganizations"
	GetOrganizationProcedure        = "/" + ServiceName + "/GetOrganization"
	GetOrganizationMembersProcedure = "/" + ServiceName + "/GetOrganizationMembers"
	DeleteOrganizationProcedure     = "/" + ServiceName + "/DeleteOrganization"

	ListRunnersProcedure   = "/" + ServiceName + "/ListRunners"
	GetRunnerProcedure     = "/" + ServiceName + "/GetRunner"
	DisableRunnerProcedure = "/" + ServiceName + "/DisableRunner"
	EnableRunnerProcedure  = "/" + ServiceName + "/EnableRunner"
	DeleteRunnerProcedure  = "/" + ServiceName + "/DeleteRunner"

	ListAuditLogsProcedure = "/" + ServiceName + "/ListAuditLogs"
)

// Server implements proto.admin.v1.AdminService.
type Server struct {
	adminSvc *adminservice.Service
}

func NewServer(adminSvc *adminservice.Service) *Server {
	return &Server{adminSvc: adminSvc}
}

// Mount registers every AdminService procedure on mux. Caller in
// cmd/server/connect_init.go threads the combined auth+admin interceptor
// via `opts`; this package never enforces auth itself.
func Mount(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mux.Handle(GetDashboardStatsProcedure, connect.NewUnaryHandler(
		GetDashboardStatsProcedure, srv.GetDashboardStats, opts...,
	))

	mux.Handle(ListUsersProcedure, connect.NewUnaryHandler(
		ListUsersProcedure, srv.ListUsers, opts...,
	))
	mux.Handle(GetUserProcedure, connect.NewUnaryHandler(
		GetUserProcedure, srv.GetUser, opts...,
	))
	mux.Handle(UpdateUserProcedure, connect.NewUnaryHandler(
		UpdateUserProcedure, srv.UpdateUser, opts...,
	))
	mux.Handle(DisableUserProcedure, connect.NewUnaryHandler(
		DisableUserProcedure, srv.DisableUser, opts...,
	))
	mux.Handle(EnableUserProcedure, connect.NewUnaryHandler(
		EnableUserProcedure, srv.EnableUser, opts...,
	))
	mux.Handle(GrantAdminProcedure, connect.NewUnaryHandler(
		GrantAdminProcedure, srv.GrantAdmin, opts...,
	))
	mux.Handle(RevokeAdminProcedure, connect.NewUnaryHandler(
		RevokeAdminProcedure, srv.RevokeAdmin, opts...,
	))
	mux.Handle(VerifyUserEmailProcedure, connect.NewUnaryHandler(
		VerifyUserEmailProcedure, srv.VerifyUserEmail, opts...,
	))
	mux.Handle(UnverifyUserEmailProcedure, connect.NewUnaryHandler(
		UnverifyUserEmailProcedure, srv.UnverifyUserEmail, opts...,
	))

	mux.Handle(ListOrganizationsProcedure, connect.NewUnaryHandler(
		ListOrganizationsProcedure, srv.ListOrganizations, opts...,
	))
	mux.Handle(GetOrganizationProcedure, connect.NewUnaryHandler(
		GetOrganizationProcedure, srv.GetOrganization, opts...,
	))
	mux.Handle(GetOrganizationMembersProcedure, connect.NewUnaryHandler(
		GetOrganizationMembersProcedure, srv.GetOrganizationMembers, opts...,
	))
	mux.Handle(DeleteOrganizationProcedure, connect.NewUnaryHandler(
		DeleteOrganizationProcedure, srv.DeleteOrganization, opts...,
	))

	mux.Handle(ListRunnersProcedure, connect.NewUnaryHandler(
		ListRunnersProcedure, srv.ListRunners, opts...,
	))
	mux.Handle(GetRunnerProcedure, connect.NewUnaryHandler(
		GetRunnerProcedure, srv.GetRunner, opts...,
	))
	mux.Handle(DisableRunnerProcedure, connect.NewUnaryHandler(
		DisableRunnerProcedure, srv.DisableRunner, opts...,
	))
	mux.Handle(EnableRunnerProcedure, connect.NewUnaryHandler(
		EnableRunnerProcedure, srv.EnableRunner, opts...,
	))
	mux.Handle(DeleteRunnerProcedure, connect.NewUnaryHandler(
		DeleteRunnerProcedure, srv.DeleteRunner, opts...,
	))

	mux.Handle(ListAuditLogsProcedure, connect.NewUnaryHandler(
		ListAuditLogsProcedure, srv.ListAuditLogs, opts...,
	))
}
