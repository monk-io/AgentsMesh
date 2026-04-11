package infra

import (
	"context"

	"github.com/anthropics/agentsmesh/backend/internal/domain/grant"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type grantRepo struct {
	db *gorm.DB
}

func NewGrantRepository(db *gorm.DB) grant.Repository {
	return &grantRepo{db: db}
}

func (r *grantRepo) Create(ctx context.Context, g *grant.ResourceGrant) error {
	return r.db.WithContext(ctx).Clauses(clause.OnConflict{
		Columns:   []clause.Column{{Name: "resource_type"}, {Name: "resource_id"}, {Name: "user_id"}},
		DoNothing: true,
	}).Create(g).Error
}

func (r *grantRepo) Delete(ctx context.Context, id int64) error {
	return r.db.WithContext(ctx).Delete(&grant.ResourceGrant{}, id).Error
}

func (r *grantRepo) ListByResource(ctx context.Context, resourceType, resourceID string) ([]*grant.ResourceGrant, error) {
	var grants []*grant.ResourceGrant
	err := r.db.WithContext(ctx).
		Where("resource_type = ? AND resource_id = ?", resourceType, resourceID).
		Preload("User").Preload("GrantedByUser").
		Order("created_at ASC").
		Find(&grants).Error
	return grants, err
}

func (r *grantRepo) GetGrantedUserIDs(ctx context.Context, resourceType, resourceID string) ([]int64, error) {
	var userIDs []int64
	err := r.db.WithContext(ctx).
		Model(&grant.ResourceGrant{}).
		Where("resource_type = ? AND resource_id = ?", resourceType, resourceID).
		Pluck("user_id", &userIDs).Error
	return userIDs, err
}

func (r *grantRepo) DeleteByResource(ctx context.Context, resourceType, resourceID string) error {
	return r.db.WithContext(ctx).
		Where("resource_type = ? AND resource_id = ?", resourceType, resourceID).
		Delete(&grant.ResourceGrant{}).Error
}
