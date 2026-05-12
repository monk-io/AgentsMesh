package grantconnect

import (
	"time"

	grantdom "github.com/anthropics/agentsmesh/backend/internal/domain/grant"
	grantv1 "github.com/anthropics/agentsmesh/proto/gen/go/grant/v1"
)

// toProtoGrant converts the GORM-backed grant + eager-loaded user assocs
// to wire shape. Returns nil for nil input so handlers can pass through
// service-layer results unchanged.
func toProtoGrant(g *grantdom.ResourceGrant) *grantv1.ResourceGrant {
	if g == nil {
		return nil
	}
	out := &grantv1.ResourceGrant{
		Id:           g.ID,
		ResourceType: g.ResourceType,
		ResourceId:   g.ResourceID,
		UserId:       g.UserID,
		GrantedBy:    g.GrantedBy,
		CreatedAt:    g.CreatedAt.UTC().Format(time.RFC3339),
	}
	if g.User != nil {
		out.User = &grantv1.ResourceGrantUser{
			Id:       g.User.ID,
			Email:    g.User.Email,
			Username: g.User.Username,
		}
		if g.User.Name != nil {
			out.User.Name = g.User.Name
		}
	}
	if g.GrantedByUser != nil {
		out.GrantedByUser = &grantv1.ResourceGrantUser{
			Id:       g.GrantedByUser.ID,
			Email:    g.GrantedByUser.Email,
			Username: g.GrantedByUser.Username,
		}
		if g.GrantedByUser.Name != nil {
			out.GrantedByUser.Name = g.GrantedByUser.Name
		}
	}
	return out
}
