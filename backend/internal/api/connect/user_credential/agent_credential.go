package usercredentialconnect

import (
	"context"
	"errors"

	"connectrpc.com/connect"

	agentservice "github.com/anthropics/agentsmesh/backend/internal/service/agent"
	ucv1 "github.com/anthropics/agentsmesh/proto/gen/go/user_credential/v1"
)

func (s *Server) ListAgentCredentialProfiles(
	ctx context.Context, _ *connect.Request[ucv1.ListAgentCredentialProfilesRequest],
) (*connect.Response[ucv1.ListAgentCredentialProfilesResponse], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	groups, err := s.credentialSvc.ListCredentialProfiles(ctx, userID)
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	items := make([]*ucv1.CredentialProfilesByAgent, 0, len(groups))
	for _, g := range groups {
		profiles := make([]*ucv1.AgentCredentialProfile, 0, len(g.Profiles))
		for _, p := range g.Profiles {
			profiles = append(profiles, toProtoAgentCredentialProfile(p))
		}
		items = append(items, &ucv1.CredentialProfilesByAgent{
			AgentSlug: g.AgentSlug,
			AgentName: g.AgentName,
			Profiles:  profiles,
		})
	}
	total := int64(len(items))
	return connect.NewResponse(&ucv1.ListAgentCredentialProfilesResponse{
		Items:  items,
		Total:  total,
		Limit:  int32(total),
		Offset: 0,
	}), nil
}

func (s *Server) ListAgentCredentialProfilesForAgent(
	ctx context.Context, req *connect.Request[ucv1.ListAgentCredentialProfilesForAgentRequest],
) (*connect.Response[ucv1.ListAgentCredentialProfilesForAgentResponse], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	profiles, err := s.credentialSvc.ListCredentialProfilesForAgent(ctx, userID, req.Msg.GetAgentSlug())
	if err != nil {
		return nil, connect.NewError(connect.CodeInternal, err)
	}
	items := make([]*ucv1.AgentCredentialProfile, 0, len(profiles))
	for _, p := range profiles {
		items = append(items, toProtoAgentCredentialProfile(s.credentialSvc.ProfileToResponse(p)))
	}
	total := int64(len(items))
	return connect.NewResponse(&ucv1.ListAgentCredentialProfilesForAgentResponse{
		Items:  items,
		Total:  total,
		Limit:  int32(total),
		Offset: 0,
		RunnerHost: &ucv1.RunnerHostInfo{
			Available:   true,
			Description: "Use Runner machine's local environment configuration",
		},
	}), nil
}

func (s *Server) GetAgentCredentialProfile(
	ctx context.Context, req *connect.Request[ucv1.GetAgentCredentialProfileRequest],
) (*connect.Response[ucv1.AgentCredentialProfile], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	p, err := s.credentialSvc.GetCredentialProfile(ctx, userID, req.Msg.GetId())
	if err != nil {
		return nil, mapAgentCredentialError(err)
	}
	return connect.NewResponse(toProtoAgentCredentialProfile(s.credentialSvc.ProfileToResponse(p))), nil
}

func (s *Server) CreateAgentCredentialProfile(
	ctx context.Context, req *connect.Request[ucv1.CreateAgentCredentialProfileRequest],
) (*connect.Response[ucv1.AgentCredentialProfile], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	params := &agentservice.CreateCredentialProfileParams{
		AgentSlug:    req.Msg.GetAgentSlug(),
		Name:         req.Msg.GetName(),
		Description:  req.Msg.Description,
		IsRunnerHost: req.Msg.GetIsRunnerHost(),
		Credentials:  req.Msg.GetCredentials(),
		IsDefault:    req.Msg.GetIsDefault(),
	}
	p, err := s.credentialSvc.CreateCredentialProfile(ctx, userID, params)
	if err != nil {
		return nil, mapAgentCredentialError(err)
	}
	return connect.NewResponse(toProtoAgentCredentialProfile(s.credentialSvc.ProfileToResponse(p))), nil
}

func (s *Server) UpdateAgentCredentialProfile(
	ctx context.Context, req *connect.Request[ucv1.UpdateAgentCredentialProfileRequest],
) (*connect.Response[ucv1.AgentCredentialProfile], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	params := &agentservice.UpdateCredentialProfileParams{
		Name:         req.Msg.Name,
		Description:  req.Msg.Description,
		IsRunnerHost: req.Msg.IsRunnerHost,
		IsDefault:    req.Msg.IsDefault,
		IsActive:     req.Msg.IsActive,
	}
	if creds := req.Msg.GetCredentials(); len(creds) > 0 {
		params.Credentials = creds
	}
	p, err := s.credentialSvc.UpdateCredentialProfile(ctx, userID, req.Msg.GetId(), params)
	if err != nil {
		return nil, mapAgentCredentialError(err)
	}
	return connect.NewResponse(toProtoAgentCredentialProfile(s.credentialSvc.ProfileToResponse(p))), nil
}

func (s *Server) DeleteAgentCredentialProfile(
	ctx context.Context, req *connect.Request[ucv1.DeleteAgentCredentialProfileRequest],
) (*connect.Response[ucv1.DeleteAgentCredentialProfileResponse], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	if err := s.credentialSvc.DeleteCredentialProfile(ctx, userID, req.Msg.GetId()); err != nil {
		return nil, mapAgentCredentialError(err)
	}
	return connect.NewResponse(&ucv1.DeleteAgentCredentialProfileResponse{}), nil
}

func (s *Server) SetDefaultAgentCredentialProfile(
	ctx context.Context, req *connect.Request[ucv1.SetDefaultAgentCredentialProfileRequest],
) (*connect.Response[ucv1.AgentCredentialProfile], error) {
	userID, err := requireUserID(ctx)
	if err != nil {
		return nil, err
	}
	p, err := s.credentialSvc.SetDefaultCredentialProfile(ctx, userID, req.Msg.GetId())
	if err != nil {
		return nil, mapAgentCredentialError(err)
	}
	return connect.NewResponse(toProtoAgentCredentialProfile(s.credentialSvc.ProfileToResponse(p))), nil
}

func mapAgentCredentialError(err error) error {
	switch {
	case errors.Is(err, agentservice.ErrCredentialProfileNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	case errors.Is(err, agentservice.ErrCredentialProfileExists):
		return connect.NewError(connect.CodeAlreadyExists, err)
	case errors.Is(err, agentservice.ErrAgentNotFound):
		return connect.NewError(connect.CodeNotFound, err)
	default:
		return connect.NewError(connect.CodeInternal, err)
	}
}
