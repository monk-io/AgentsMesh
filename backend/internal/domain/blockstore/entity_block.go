package blockstore

import (
	"time"

	"github.com/google/uuid"
)

// Block is the minimal addressable, extensible data unit.
// Block itself holds NO relationship fields — all relations (nest / mention /
// embed / depends_on / ...) live in block_refs, keyed by rel.
//
// Field responsibilities:
//   - Data  — structured, type-specific payload. Schema defined by BlockTypeSpec.
//   - Text  — plain-text summary used for full-text search (tsv) and semantic
//     embedding. **The writer maintains this field.** The service layer does
//     NOT derive Text from Data — doing so would silently couple every new
//     block type to a server-side extractor and break openness. Agents and UI
//     are responsible for supplying a relevant Text when they want the block
//     discoverable by search / memory retrieval.
//   - Meta  — ACL / tags / extension points that don't belong to the type
//     payload itself.
type Block struct {
	ID          uuid.UUID  `gorm:"type:uuid;primaryKey" json:"id"`
	WorkspaceID uuid.UUID  `gorm:"type:uuid;not null;index" json:"workspace_id"`
	Type        string     `gorm:"size:64;not null" json:"type"`
	Data        JSONMap    `gorm:"type:jsonb;not null;default:'{}'" json:"data"`
	Text        *string    `gorm:"type:text" json:"text,omitempty"`
	Meta        JSONMap    `gorm:"type:jsonb;not null;default:'{}'" json:"meta"`
	CreatedBy   int64      `gorm:"not null" json:"created_by"`
	CreatedAt   time.Time  `gorm:"not null;default:current_timestamp" json:"created_at"`
	UpdatedAt   time.Time  `gorm:"not null;default:current_timestamp" json:"updated_at"`
	DeletedAt   *time.Time `json:"deleted_at,omitempty"`
}

func (Block) TableName() string {
	return "blocks"
}
