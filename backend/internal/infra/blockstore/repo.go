package blockstoreinfra

import (
	"context"
	"errors"
	"fmt"
	"hash/fnv"
	"strings"
	"sync"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
	"gorm.io/gorm"
)

// Compile-time interface compliance check.
var _ blockstore.Repository = (*Repository)(nil)

// Repository implements blockstore.Repository using GORM + Postgres.
// Writes flow through WithinWorkspaceTx which obtains a transaction-scoped
// advisory lock keyed on the workspace, serialising concurrent ApplyOps calls
// against the same workspace and guaranteeing monotonic op_id.
type Repository struct {
	db *gorm.DB

	// pgvector detection is per-Repository so tests that rebuild the DB for
	// each case get a fresh probe. Shared state across repositories would
	// lead to cross-contamination when SQLite and Postgres are both used
	// in the same process (rare today, but a correctness footgun).
	pgvectorOnce  sync.Once
	pgvectorReady bool
	pgvectorDims  int
}

func NewRepository(db *gorm.DB) *Repository {
	return &Repository{db: db}
}

// WithinWorkspaceTx wraps fn in a transaction that first grabs
// pg_advisory_xact_lock(key) where key = fnv1a(workspaceID). Lock is released
// on commit/rollback automatically.
func (r *Repository) WithinWorkspaceTx(ctx context.Context, workspaceID uuid.UUID, fn func(blockstore.TxWriter) error) error {
	return r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		if err := acquireWorkspaceLock(tx, workspaceID); err != nil {
			return err
		}
		return fn(&txWriter{tx: tx, workspaceID: workspaceID})
	})
}

// acquireWorkspaceLock takes a transaction-scoped advisory lock by hashing the
// workspace UUID to a signed 64-bit int. No-op on non-Postgres dialects (e.g.
// the SQLite test DB) where the service layer's sequential op apply inside a
// single test process is already serial.
func acquireWorkspaceLock(tx *gorm.DB, workspaceID uuid.UUID) error {
	if tx.Dialector.Name() != "postgres" {
		return nil
	}
	key := workspaceLockKey(workspaceID)
	if err := tx.Exec("SELECT pg_advisory_xact_lock(?)", key).Error; err != nil {
		return fmt.Errorf("acquire workspace lock: %w", err)
	}
	return nil
}

// workspaceLockKey returns a stable int64 derived from the workspace UUID.
// FNV-1a 64-bit over the raw 16 bytes; sign-cast is fine because
// pg_advisory_xact_lock accepts a bigint key regardless of sign.
func workspaceLockKey(workspaceID uuid.UUID) int64 {
	h := fnv.New64a()
	_, _ = h.Write(workspaceID[:])
	return int64(h.Sum64())
}

// --- Workspaces ---

// isUniqueViolation inspects a GORM-wrapped Postgres error for SQLSTATE 23505
// (unique_violation). We avoid importing pgx just for the error type by
// matching on the substring in the driver-formatted message, which gorm's
// pgx v5 driver includes verbatim. Belt-and-braces alternative "duplicate
// key" string is also checked for older drivers / lib/pq.
func isUniqueViolation(err error) bool {
	if err == nil {
		return false
	}
	msg := err.Error()
	return strings.Contains(msg, "SQLSTATE 23505") || strings.Contains(msg, "duplicate key value")
}

func (r *Repository) CreateWorkspace(ctx context.Context, ws *blockstore.BlockWorkspace) error {
	err := r.db.WithContext(ctx).Create(ws).Error
	if err == nil {
		return nil
	}
	// Translate Postgres UNIQUE(org_id, slug) collision to a domain sentinel
	// so EnsureDefaultWorkspace can react to races idempotently. Any other
	// error surfaces as-is.
	if isUniqueViolation(err) {
		return blockstore.ErrWorkspaceAlreadyExists
	}
	return err
}

func (r *Repository) GetWorkspace(ctx context.Context, id uuid.UUID) (*blockstore.BlockWorkspace, error) {
	var ws blockstore.BlockWorkspace
	err := r.db.WithContext(ctx).Where("id = ?", id).First(&ws).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, blockstore.ErrWorkspaceNotFound
		}
		return nil, err
	}
	return &ws, nil
}

func (r *Repository) GetWorkspaceBySlug(ctx context.Context, orgID int64, slug string) (*blockstore.BlockWorkspace, error) {
	var ws blockstore.BlockWorkspace
	err := r.db.WithContext(ctx).
		Where("organization_id = ? AND slug = ?", orgID, slug).
		First(&ws).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, blockstore.ErrWorkspaceNotFound
		}
		return nil, err
	}
	return &ws, nil
}

func (r *Repository) ListWorkspaces(ctx context.Context, orgID int64) ([]*blockstore.BlockWorkspace, error) {
	var out []*blockstore.BlockWorkspace
	err := r.db.WithContext(ctx).
		Where("organization_id = ?", orgID).
		Order("created_at ASC").
		Find(&out).Error
	return out, err
}

func (r *Repository) UpdateWorkspaceRootBlock(ctx context.Context, wsID, rootID uuid.UUID) error {
	return r.db.WithContext(ctx).
		Model(&blockstore.BlockWorkspace{}).
		Where("id = ?", wsID).
		Updates(map[string]any{"root_block_id": rootID, "updated_at": gorm.Expr("CURRENT_TIMESTAMP")}).Error
}

// DeleteWorkspaceCascade hard-deletes a workspace together with every
// downstream row (embeddings → ops → refs → blocks → workspace row) inside
// one transaction. Order follows the natural dependency graph so even a
// partial failure leaves the DB consistent. No FKs exist, so we must be
// explicit — a stray child row after a workspace row gone would be an
// orphan that leaks storage forever.
func (r *Repository) DeleteWorkspaceCascade(ctx context.Context, wsID uuid.UUID) error {
	return r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		// Embeddings are keyed by block_id; no workspace column, so scope via join.
		if err := tx.Exec(`
			DELETE FROM block_embeddings
			 WHERE block_id IN (SELECT id FROM blocks WHERE workspace_id = ?)
		`, wsID).Error; err != nil {
			return err
		}
		if err := tx.Where("workspace_id = ?", wsID).Delete(&blockstore.BlockOp{}).Error; err != nil {
			return err
		}
		if err := tx.Where("workspace_id = ?", wsID).Delete(&blockstore.BlockRef{}).Error; err != nil {
			return err
		}
		if err := tx.Unscoped().Where("workspace_id = ?", wsID).Delete(&blockstore.Block{}).Error; err != nil {
			return err
		}
		res := tx.Where("id = ?", wsID).Delete(&blockstore.BlockWorkspace{})
		if res.Error != nil {
			return res.Error
		}
		if res.RowsAffected == 0 {
			return blockstore.ErrWorkspaceNotFound
		}
		return nil
	})
}
