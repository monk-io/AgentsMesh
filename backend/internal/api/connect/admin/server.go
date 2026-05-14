// Package adminconnect hosts Connect-RPC handlers for the platform-admin
// management plane (system administrators with `is_system_admin = true`).
//
// Migrated from REST handlers in backend/internal/api/rest/v1/admin/*. The
// REST surface gets deleted as call sites flip to Connect; the per-resource
// org-scoped Connect surfaces (org/runner/etc.) stay parallel.
//
// Auth model: every RPC calls interceptors.ResolveSystemAdmin to mirror
// REST's AdminMiddleware (is_system_admin + is_active checks). Per
// conventions §3.5 exception #2, admin requests do NOT carry `org_slug` —
// tenant is the whole platform.
//
// Split rationale (CLAUDE.md 200-line rule):
//   - server.go               — service scaffolding + Mount (this file)
//   - convert.go              — domain ↔ proto field translation
//   - audit.go                — Connect-context audit log helper
//   - handlers_users_query.go — ListUsers / GetUser / UpdateUser
//   - handlers_users_actions.go — Disable / Enable / GrantAdmin / RevokeAdmin /
//                                 VerifyUserEmail / UnverifyUserEmail
//   - handlers_orgs.go        — ListOrganizations / GetOrganization /
//                               GetOrganizationMembers / DeleteOrganization
package adminconnect

import (
	"errors"
	"net/http"

	"connectrpc.com/connect"

	"github.com/anthropics/agentsmesh/backend/internal/infra/database"
	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
)

const ServiceName = "proto.admin.v1.AdminService"

const (
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
)

// Server implements the user + organization slice of proto.admin.v1.AdminService.
// Dashboard / Runner / AuditLog RPCs live in the same proto service but stay
// on REST until follow-up PRs migrate them.
type Server struct {
	svc *adminservice.Service
	db  database.DB
}

func NewServer(svc *adminservice.Service, db database.DB) *Server {
	return &Server{svc: svc, db: db}
}

// Mount wires each implemented AdminService procedure onto mux. The auth
// interceptor in opts validates the JWT; per-handler ResolveSystemAdmin
// enforces is_system_admin (handler-level so the interceptor stays generic
// across user-scoped + admin-scoped services).
func Mount(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
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
}

// mapServiceError translates admin-service sentinels to Connect codes,
// mirroring apierr translation in REST handlers.
func mapServiceError(err error) error {
	switch {
	case errors.Is(err, adminservice.ErrUserNotFound),
		errors.Is(err, adminservice.ErrOrganizationNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	case errors.Is(err, adminservice.ErrUsernameAlreadyExists),
		errors.Is(err, adminservice.ErrEmailAlreadyExists),
		errors.Is(err, adminservice.ErrOrganizationHasActiveRunner):
		return connect.NewError(connect.CodeAlreadyExists, err)
	case errors.Is(err, adminservice.ErrCannotRevokeOwnAdmin),
		errors.Is(err, adminservice.ErrCannotDisableSelf):
		return connect.NewError(connect.CodeInvalidArgument, err)
	default:
		return connect.NewError(connect.CodeInternal, err)
	}
}
