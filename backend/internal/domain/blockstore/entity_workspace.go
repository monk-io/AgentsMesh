package blockstore

import (
	"time"

	"github.com/google/uuid"
)

// BlockWorkspace is a named namespace inside an organization.
// Holds an optional root_block_id that points to the conventional entry page
// for UI navigation. Blocks and their refs live under exactly one workspace.
type BlockWorkspace struct {
	ID             uuid.UUID  `gorm:"type:uuid;primaryKey" json:"id"`
	OrganizationID int64      `gorm:"not null;index" json:"organization_id"`
	Slug           string     `gorm:"size:64;not null" json:"slug"`
	Name           string     `gorm:"size:200;not null" json:"name"`
	RootBlockID    *uuid.UUID `gorm:"type:uuid" json:"root_block_id,omitempty"`
	CreatedBy      int64      `gorm:"not null" json:"created_by"`
	CreatedAt      time.Time  `gorm:"not null;default:current_timestamp" json:"created_at"`
	UpdatedAt      time.Time  `gorm:"not null;default:current_timestamp" json:"updated_at"`
}

func (BlockWorkspace) TableName() string {
	return "block_workspaces"
}

// DefaultWorkspaceSlug is used by the workspace bootstrapper for the auto-created
// first workspace of an organization.
const DefaultWorkspaceSlug = "default"
