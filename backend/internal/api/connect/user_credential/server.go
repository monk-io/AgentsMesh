// Package usercredentialconnect hosts Connect-RPC handlers for three user-scoped
// services that share the proto.user_credential.v1 package: Git credentials,
// Agent credential profiles, and Repository providers. They're grouped because
// they're tightly coupled at the data layer (GitCredential references
// RepositoryProvider) and share the same user-scope auth boundary.
//
// Per conventions §3.5 these are user-scoped — the auth interceptor populates
// TenantContext.UserID and that's the only scope each RPC enforces, no
// org_slug = 1 (would have been a no-op).
//
// SENSITIVE DATA: PAT / SSH private key / OAuth client_secret / bot_token
// never appear on response messages. The convert helpers omit them by
// construction (REST-side has `json:"-"` on the same fields).
package usercredentialconnect

import (
	"net/http"

	"connectrpc.com/connect"

	agentservice "github.com/anthropics/agentsmesh/backend/internal/service/agent"
	userservice "github.com/anthropics/agentsmesh/backend/internal/service/user"
)

const (
	GitServiceName        = "proto.user_credential.v1.UserGitCredentialService"
	AgentServiceName      = "proto.user_credential.v1.UserAgentCredentialService"
	ProviderServiceName   = "proto.user_credential.v1.UserRepositoryProviderService"
)

const (
	ListGitCredentialsProcedure        = "/" + GitServiceName + "/ListGitCredentials"
	GetGitCredentialProcedure          = "/" + GitServiceName + "/GetGitCredential"
	CreateGitCredentialProcedure       = "/" + GitServiceName + "/CreateGitCredential"
	UpdateGitCredentialProcedure       = "/" + GitServiceName + "/UpdateGitCredential"
	DeleteGitCredentialProcedure       = "/" + GitServiceName + "/DeleteGitCredential"
	GetDefaultGitCredentialProcedure   = "/" + GitServiceName + "/GetDefaultGitCredential"
	SetDefaultGitCredentialProcedure   = "/" + GitServiceName + "/SetDefaultGitCredential"
	ClearDefaultGitCredentialProcedure = "/" + GitServiceName + "/ClearDefaultGitCredential"
)

const (
	ListAgentCredentialProfilesProcedure         = "/" + AgentServiceName + "/ListAgentCredentialProfiles"
	ListAgentCredentialProfilesForAgentProcedure = "/" + AgentServiceName + "/ListAgentCredentialProfilesForAgent"
	GetAgentCredentialProfileProcedure           = "/" + AgentServiceName + "/GetAgentCredentialProfile"
	CreateAgentCredentialProfileProcedure        = "/" + AgentServiceName + "/CreateAgentCredentialProfile"
	UpdateAgentCredentialProfileProcedure        = "/" + AgentServiceName + "/UpdateAgentCredentialProfile"
	DeleteAgentCredentialProfileProcedure        = "/" + AgentServiceName + "/DeleteAgentCredentialProfile"
	SetDefaultAgentCredentialProfileProcedure    = "/" + AgentServiceName + "/SetDefaultAgentCredentialProfile"
)

const (
	ListRepositoryProvidersProcedure          = "/" + ProviderServiceName + "/ListRepositoryProviders"
	GetRepositoryProviderProcedure            = "/" + ProviderServiceName + "/GetRepositoryProvider"
	CreateRepositoryProviderProcedure         = "/" + ProviderServiceName + "/CreateRepositoryProvider"
	UpdateRepositoryProviderProcedure         = "/" + ProviderServiceName + "/UpdateRepositoryProvider"
	DeleteRepositoryProviderProcedure         = "/" + ProviderServiceName + "/DeleteRepositoryProvider"
	SetDefaultRepositoryProviderProcedure     = "/" + ProviderServiceName + "/SetDefaultRepositoryProvider"
	TestRepositoryProviderConnectionProcedure = "/" + ProviderServiceName + "/TestRepositoryProviderConnection"
	ListProviderRepositoriesProcedure         = "/" + ProviderServiceName + "/ListProviderRepositories"
)

// Server implements all three credential services. They share dependencies
// (userService is the home of both Git credentials and Repository providers),
// so one struct keeps the dep wiring simple.
type Server struct {
	userSvc       *userservice.Service
	credentialSvc *agentservice.CredentialProfileService
}

// NewServer constructs a Server with the three required service deps.
func NewServer(
	userSvc *userservice.Service,
	credentialSvc *agentservice.CredentialProfileService,
) *Server {
	return &Server{
		userSvc:       userSvc,
		credentialSvc: credentialSvc,
	}
}

// Mount registers all 23 procedures across the three services on mux behind
// the auth interceptor supplied via opts (cmd/server/connect_init.go).
func Mount(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mountGitCredential(mux, srv, opts...)
	mountAgentCredential(mux, srv, opts...)
	mountRepositoryProvider(mux, srv, opts...)
}

func mountGitCredential(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mux.Handle(ListGitCredentialsProcedure, connect.NewUnaryHandler(
		ListGitCredentialsProcedure, srv.ListGitCredentials, opts...,
	))
	mux.Handle(GetGitCredentialProcedure, connect.NewUnaryHandler(
		GetGitCredentialProcedure, srv.GetGitCredential, opts...,
	))
	mux.Handle(CreateGitCredentialProcedure, connect.NewUnaryHandler(
		CreateGitCredentialProcedure, srv.CreateGitCredential, opts...,
	))
	mux.Handle(UpdateGitCredentialProcedure, connect.NewUnaryHandler(
		UpdateGitCredentialProcedure, srv.UpdateGitCredential, opts...,
	))
	mux.Handle(DeleteGitCredentialProcedure, connect.NewUnaryHandler(
		DeleteGitCredentialProcedure, srv.DeleteGitCredential, opts...,
	))
	mux.Handle(GetDefaultGitCredentialProcedure, connect.NewUnaryHandler(
		GetDefaultGitCredentialProcedure, srv.GetDefaultGitCredential, opts...,
	))
	mux.Handle(SetDefaultGitCredentialProcedure, connect.NewUnaryHandler(
		SetDefaultGitCredentialProcedure, srv.SetDefaultGitCredential, opts...,
	))
	mux.Handle(ClearDefaultGitCredentialProcedure, connect.NewUnaryHandler(
		ClearDefaultGitCredentialProcedure, srv.ClearDefaultGitCredential, opts...,
	))
}

func mountAgentCredential(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mux.Handle(ListAgentCredentialProfilesProcedure, connect.NewUnaryHandler(
		ListAgentCredentialProfilesProcedure, srv.ListAgentCredentialProfiles, opts...,
	))
	mux.Handle(ListAgentCredentialProfilesForAgentProcedure, connect.NewUnaryHandler(
		ListAgentCredentialProfilesForAgentProcedure, srv.ListAgentCredentialProfilesForAgent, opts...,
	))
	mux.Handle(GetAgentCredentialProfileProcedure, connect.NewUnaryHandler(
		GetAgentCredentialProfileProcedure, srv.GetAgentCredentialProfile, opts...,
	))
	mux.Handle(CreateAgentCredentialProfileProcedure, connect.NewUnaryHandler(
		CreateAgentCredentialProfileProcedure, srv.CreateAgentCredentialProfile, opts...,
	))
	mux.Handle(UpdateAgentCredentialProfileProcedure, connect.NewUnaryHandler(
		UpdateAgentCredentialProfileProcedure, srv.UpdateAgentCredentialProfile, opts...,
	))
	mux.Handle(DeleteAgentCredentialProfileProcedure, connect.NewUnaryHandler(
		DeleteAgentCredentialProfileProcedure, srv.DeleteAgentCredentialProfile, opts...,
	))
	mux.Handle(SetDefaultAgentCredentialProfileProcedure, connect.NewUnaryHandler(
		SetDefaultAgentCredentialProfileProcedure, srv.SetDefaultAgentCredentialProfile, opts...,
	))
}

func mountRepositoryProvider(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mux.Handle(ListRepositoryProvidersProcedure, connect.NewUnaryHandler(
		ListRepositoryProvidersProcedure, srv.ListRepositoryProviders, opts...,
	))
	mux.Handle(GetRepositoryProviderProcedure, connect.NewUnaryHandler(
		GetRepositoryProviderProcedure, srv.GetRepositoryProvider, opts...,
	))
	mux.Handle(CreateRepositoryProviderProcedure, connect.NewUnaryHandler(
		CreateRepositoryProviderProcedure, srv.CreateRepositoryProvider, opts...,
	))
	mux.Handle(UpdateRepositoryProviderProcedure, connect.NewUnaryHandler(
		UpdateRepositoryProviderProcedure, srv.UpdateRepositoryProvider, opts...,
	))
	mux.Handle(DeleteRepositoryProviderProcedure, connect.NewUnaryHandler(
		DeleteRepositoryProviderProcedure, srv.DeleteRepositoryProvider, opts...,
	))
	mux.Handle(SetDefaultRepositoryProviderProcedure, connect.NewUnaryHandler(
		SetDefaultRepositoryProviderProcedure, srv.SetDefaultRepositoryProvider, opts...,
	))
	mux.Handle(TestRepositoryProviderConnectionProcedure, connect.NewUnaryHandler(
		TestRepositoryProviderConnectionProcedure, srv.TestRepositoryProviderConnection, opts...,
	))
	mux.Handle(ListProviderRepositoriesProcedure, connect.NewUnaryHandler(
		ListProviderRepositoriesProcedure, srv.ListProviderRepositories, opts...,
	))
}
