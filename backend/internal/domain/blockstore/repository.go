package blockstore

import (
	"context"

	"github.com/google/uuid"
)

// RefFilter narrows block_refs queries.
type RefFilter struct {
	WorkspaceID uuid.UUID
	FromID      *uuid.UUID
	ToID        *uuid.UUID
	Rel         *string
	Limit       int
	Offset      int
}

// BlockFilter narrows blocks queries.
type BlockFilter struct {
	WorkspaceID uuid.UUID
	Type        *string
	IncludeDeleted bool
	Limit       int
	Offset      int
}

// OpStreamFilter fetches ops strictly after a given id, for subscription catch-up.
type OpStreamFilter struct {
	WorkspaceID uuid.UUID
	AfterID     int64 // exclusive
	Limit       int
}

// TxWriter is the write surface exposed inside an ApplyOps transaction.
// Service layer composes multiple TxWriter calls into one atomic op batch.
type TxWriter interface {
	InsertBlock(ctx context.Context, b *Block) error
	UpdateBlockFields(ctx context.Context, id uuid.UUID, fields map[string]any) error
	SoftDeleteBlock(ctx context.Context, id uuid.UUID) error
	InsertRef(ctx context.Context, r *BlockRef) (int64, error)
	DeleteRefByID(ctx context.Context, id int64) error
	UpdateRefFields(ctx context.Context, id int64, fields map[string]any) error
	InsertOp(ctx context.Context, o *BlockOp) (int64, error)

	// Helpers for cycle/parent checks, scoped to current tx.
	FindNestParent(ctx context.Context, childID uuid.UUID) (*BlockRef, error)
	FindAncestors(ctx context.Context, blockID uuid.UUID, maxDepth int) ([]uuid.UUID, error)
	FindRefByID(ctx context.Context, id int64) (*BlockRef, error)
	FindBlockByID(ctx context.Context, id uuid.UUID) (*Block, error)
	FindOpByIdempotencyKey(ctx context.Context, key string) (*BlockOp, error)
	// ListOpsByParent returns every op whose parent_op_id matches the given id,
	// ordered by id ascending. Used by idempotent replay to return the full
	// op list, not just the batch head. Empty result is a non-error.
	ListOpsByParent(ctx context.Context, parentOpID int64) ([]*BlockOp, error)
	// ListTypeDefs returns all block_type_def blocks in the current workspace,
	// using the same transaction. Used by the service's dynamic type resolver
	// so newly-written type definitions become visible inside the same
	// ApplyOps batch that registered them.
	ListTypeDefs(ctx context.Context) ([]*Block, error)
}

// Repository is the full read/write surface of the block store.
// Write paths typically flow through WithinWorkspaceTx (advisory-locked,
// transactional), while read paths are plain queries.
type Repository interface {
	// --- Workspaces ---
	CreateWorkspace(ctx context.Context, ws *BlockWorkspace) error
	GetWorkspace(ctx context.Context, id uuid.UUID) (*BlockWorkspace, error)
	GetWorkspaceBySlug(ctx context.Context, orgID int64, slug string) (*BlockWorkspace, error)
	ListWorkspaces(ctx context.Context, orgID int64) ([]*BlockWorkspace, error)
	UpdateWorkspaceRootBlock(ctx context.Context, wsID, rootID uuid.UUID) error
	// DeleteWorkspaceCascade hard-deletes the workspace row and every row
	// keyed to it (blocks, block_refs, block_ops, block_embeddings). No
	// soft-delete — this endpoint exists to reclaim space after E2E runs or
	// when an org genuinely retires a workspace. Callers must enforce policy
	// (no deleting the default workspace; org-member check) upstream.
	DeleteWorkspaceCascade(ctx context.Context, workspaceID uuid.UUID) error

	// --- Transactional write path ---
	// WithinWorkspaceTx takes a per-workspace advisory lock, opens a transaction,
	// and hands the caller a TxWriter. If fn returns nil the transaction commits.
	WithinWorkspaceTx(ctx context.Context, workspaceID uuid.UUID, fn func(TxWriter) error) error

	// --- Reads ---
	GetBlock(ctx context.Context, id uuid.UUID) (*Block, error)
	ListBlocks(ctx context.Context, filter BlockFilter) ([]*Block, int64, error)
	ListChildren(ctx context.Context, parentID uuid.UUID, rel string) ([]*Block, []*BlockRef, error)
	ListBacklinks(ctx context.Context, targetID uuid.UUID, excludeNest bool) ([]*BlockRef, error)
	ListRefs(ctx context.Context, filter RefFilter) ([]*BlockRef, error)
	StreamOps(ctx context.Context, filter OpStreamFilter) ([]*BlockOp, error)

	// ListWorkspaceSubtree returns the full nest tree rooted at rootID.
	// The second return value is the flat list of nest refs connecting the blocks.
	ListWorkspaceSubtree(ctx context.Context, workspaceID, rootID uuid.UUID, maxDepth int) ([]*Block, []*BlockRef, error)

	// GetTypeDefByKey returns the highest-revision block_type_def block in a
	// workspace whose data.type_key matches the given key, or nil when none
	// exists. Used by the service's type resolver to avoid O(N) scans of all
	// type-def blocks. Postgres backends use a JSONB expression; SQLite test
	// backends fall back to LIKE on the serialized data.
	GetTypeDefByKey(ctx context.Context, workspaceID uuid.UUID, typeKey string) (*Block, error)

	// --- Embeddings (Phase 4) ---
	UpsertEmbedding(ctx context.Context, blockID uuid.UUID, model string, dims int, vector []float32, sourceHash string) error
	ListEmbeddings(ctx context.Context, workspaceID uuid.UUID, model string) ([]EmbeddingRow, error)
	DeleteEmbedding(ctx context.Context, blockID uuid.UUID) error
	// GetEmbeddingHash returns the current source_hash of a block's embedding,
	// or "" if none exists. Used by the service to skip re-embedding unchanged
	// text (no-op updateBlock, or update touching only non-text fields).
	GetEmbeddingHash(ctx context.Context, blockID uuid.UUID) (string, error)
	// SearchEmbeddings ranks and limits server-side when the Postgres vec
	// column + HNSW index are available; otherwise falls back to a projection
	// the service can sort in memory. Either way the caller gets at most topK
	// EmbeddingRows already narrowed to this workspace.
	SearchEmbeddings(ctx context.Context, workspaceID uuid.UUID, model string, queryVec []float32, topK int) ([]EmbeddingRow, error)
}

// EmbeddingRow is the join projection used by semantic search. Keeping it
// flat avoids shipping the block's full row for every candidate when we
// only need id + vector + text-for-snippet + ACL inputs.
type EmbeddingRow struct {
	BlockID   uuid.UUID
	Type      string
	Text      *string
	Vector    []float32
	CreatedBy int64
	Meta      JSONMap
	// Score is populated by pgvector-backed SearchEmbeddings; JSONB-fallback
	// callers ignore it and rank in memory.
	Score float32
}
