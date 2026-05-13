// Package meshconnect hosts Connect-RPC handlers for the mesh domain.
// Mirrors backend/internal/api/rest/v1/mesh.go but exposes the data plane
// via Connect (binary protobuf wire, see conventions §2.5).
//
// Note: ticket→pod lookup (GetTicketPods + BatchGetTicketPods + CreatePodForTicket)
// belongs to MeshService — it stayed on REST throughout the ticket migration
// for that reason. This PR brings it onto Connect alongside the topology RPC.
//
// REST handler stays mounted in parallel until consumers flip lanes.
package meshconnect

import (
	"errors"
	"net/http"

	"connectrpc.com/connect"

	"github.com/anthropics/agentsmesh/backend/internal/middleware"
	meshservice "github.com/anthropics/agentsmesh/backend/internal/service/mesh"
	ticketservice "github.com/anthropics/agentsmesh/backend/internal/service/ticket"
)

const ServiceName = "proto.mesh.v1.MeshService"

const (
	GetMeshTopologyProcedure     = "/" + ServiceName + "/GetMeshTopology"
	GetTicketPodsProcedure       = "/" + ServiceName + "/GetTicketPods"
	BatchGetTicketPodsProcedure  = "/" + ServiceName + "/BatchGetTicketPods"
	CreatePodForTicketProcedure  = "/" + ServiceName + "/CreatePodForTicket"
)

// Server implements the MeshService contract. Mirrors REST's MeshHandler
// dependencies (mesh.go:17) — the mesh + ticket services in tandem because
// the ticket→pod lookups resolve the ticket by slug before delegating to
// the mesh service.
type Server struct {
	meshSvc   *meshservice.Service
	ticketSvc *ticketservice.Service
	orgSvc    middleware.OrganizationService
}

func NewServer(
	meshSvc *meshservice.Service,
	ticketSvc *ticketservice.Service,
	orgSvc middleware.OrganizationService,
) *Server {
	return &Server{meshSvc: meshSvc, ticketSvc: ticketSvc, orgSvc: orgSvc}
}

// mapServiceError mirrors REST's apierr translations (mesh.go). Translates
// mesh-domain sentinels to Connect codes per conventions §10. The mesh
// service mostly delegates to repo / pod / channel / binding services, so
// sentinels are imported from the package level (ErrTicketNotFound /
// ErrRunnerNotFound) rather than re-exported per-RPC.
func mapServiceError(err error) error {
	switch {
	case errors.Is(err, meshservice.ErrTicketNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	case errors.Is(err, meshservice.ErrRunnerNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	default:
		return connect.NewError(connect.CodeInternal, err)
	}
}

// Mount registers all MeshService procedures on mux behind the auth
// interceptor supplied via opts (cmd/server/connect_init.go).
func Mount(mux *http.ServeMux, srv *Server, opts ...connect.HandlerOption) {
	mux.Handle(GetMeshTopologyProcedure, connect.NewUnaryHandler(
		GetMeshTopologyProcedure, srv.GetMeshTopology, opts...,
	))
	mux.Handle(GetTicketPodsProcedure, connect.NewUnaryHandler(
		GetTicketPodsProcedure, srv.GetTicketPods, opts...,
	))
	mux.Handle(BatchGetTicketPodsProcedure, connect.NewUnaryHandler(
		BatchGetTicketPodsProcedure, srv.BatchGetTicketPods, opts...,
	))
	mux.Handle(CreatePodForTicketProcedure, connect.NewUnaryHandler(
		CreatePodForTicketProcedure, srv.CreatePodForTicket, opts...,
	))
}
