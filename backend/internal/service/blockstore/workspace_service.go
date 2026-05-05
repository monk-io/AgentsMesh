package blockstoreservice

import (
	"context"
	"errors"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// WorkspaceView is the API-facing shape; it folds in the root block summary
// so clients can render a workspace list with a single request.
type WorkspaceView struct {
	ID             uuid.UUID  `json:"id"`
	OrganizationID int64      `json:"organization_id"`
	Slug           string     `json:"slug"`
	Name           string     `json:"name"`
	RootBlockID    *uuid.UUID `json:"root_block_id,omitempty"`
	CreatedAt      time.Time  `json:"created_at"`
}

func workspaceView(ws *blockstore.BlockWorkspace) WorkspaceView {
	return WorkspaceView{
		ID: ws.ID, OrganizationID: ws.OrganizationID, Slug: ws.Slug,
		Name: ws.Name, RootBlockID: ws.RootBlockID, CreatedAt: ws.CreatedAt,
	}
}

func (s *Service) ListWorkspaces(ctx context.Context, actor ActorContext) ([]WorkspaceView, error) {
	list, err := s.repo.ListWorkspaces(ctx, actor.OrgID)
	if err != nil {
		return nil, err
	}
	out := make([]WorkspaceView, 0, len(list))
	for _, ws := range list {
		out = append(out, workspaceView(ws))
	}
	return out, nil
}

// EnsureDefaultWorkspace returns the caller org's default workspace, creating
// it (and a root page block) atomically on first access.
//
// Race handling: two concurrent first-time calls can both see NotFound and
// race to create. Postgres UNIQUE(org_id, slug) serialises them — the loser
// gets a unique-violation. Instead of propagating that as a 500, we re-read
// the workspace the winner just committed and return it, making the call
// idempotent from the caller's perspective.
func (s *Service) EnsureDefaultWorkspace(ctx context.Context, actor ActorContext) (WorkspaceView, error) {
	existing, err := s.repo.GetWorkspaceBySlug(ctx, actor.OrgID, blockstore.DefaultWorkspaceSlug)
	if err == nil {
		return workspaceView(existing), nil
	}
	if !errors.Is(err, blockstore.ErrWorkspaceNotFound) {
		return WorkspaceView{}, err
	}
	return s.createWorkspaceWithRoot(ctx, actor, blockstore.DefaultWorkspaceSlug, "Default Workspace")
}

// CreateWorkspace provisions a new workspace with the given slug + name inside
// the caller's org. Fails fast with ErrWorkspaceAlreadyExists if the slug is
// taken — callers that need idempotency (see EnsureDefaultWorkspace) must
// check + recover themselves. Primary use today: E2E tests asking for an
// isolated workspace per test so prior-run detritus can't influence results.
func (s *Service) CreateWorkspace(
	ctx context.Context,
	actor ActorContext,
	slug, name string,
) (WorkspaceView, error) {
	if slug == "" {
		return WorkspaceView{}, errors.New("slug is required")
	}
	if name == "" {
		name = slug
	}
	return s.createWorkspaceWithRoot(ctx, actor, slug, name)
}

// createWorkspaceWithRoot is the shared creation path: insert the workspace
// row, then inside a workspace-scoped tx create the root page block + its
// createBlock op, then stamp root_block_id back on the workspace.
func (s *Service) createWorkspaceWithRoot(
	ctx context.Context,
	actor ActorContext,
	slug, name string,
) (WorkspaceView, error) {
	ws := &blockstore.BlockWorkspace{
		ID:             uuid.New(),
		OrganizationID: actor.OrgID,
		Slug:           slug,
		Name:           name,
		CreatedBy:      actor.UserID,
		CreatedAt:      timeNowUTC(),
		UpdatedAt:      timeNowUTC(),
	}
	if err := s.repo.CreateWorkspace(ctx, ws); err != nil {
		// UNIQUE(org_id, slug) collision: the Ensure path recovers by
		// re-reading; direct CreateWorkspace callers get the sentinel.
		if errors.Is(err, blockstore.ErrWorkspaceAlreadyExists) && slug == blockstore.DefaultWorkspaceSlug {
			winner, err2 := s.repo.GetWorkspaceBySlug(ctx, actor.OrgID, blockstore.DefaultWorkspaceSlug)
			if err2 == nil {
				return workspaceView(winner), nil
			}
		}
		return WorkspaceView{}, err
	}

	// Create root page + record a system createBlock op so the subscription
	// stream reflects the entire workspace history from op_id=1.
	rootID := uuid.New()
	err := s.repo.WithinWorkspaceTx(ctx, ws.ID, func(tx blockstore.TxWriter) error {
		now := timeNowUTC()
		block := &blockstore.Block{
			ID:          rootID,
			WorkspaceID: ws.ID,
			Type:        blockstore.BlockTypePage,
			Data:        blockstore.JSONMap{"title": "Home"},
			Meta:        blockstore.JSONMap{},
			CreatedBy:   actor.UserID,
			CreatedAt:   now,
			UpdatedAt:   now,
		}
		if err := tx.InsertBlock(ctx, block); err != nil {
			return err
		}
		op := &blockstore.BlockOp{
			WorkspaceID: ws.ID,
			ActorType:   blockstore.ActorSystem,
			ActorID:     actor.UserID,
			Op:          blockstore.OpCreateBlock,
			TargetBlock: &rootID,
			Payload:     blockstore.JSONMap{"id": rootID, "type": block.Type, "data": block.Data},
			Forward:     blockstore.JSONMap{"id": rootID, "type": block.Type, "data": block.Data, "meta": block.Meta},
			Inverse:     blockstore.JSONMap{"id": rootID},
			AppliedAt:   now,
		}
		_, err := tx.InsertOp(ctx, op)
		return err
	})
	if err != nil {
		return WorkspaceView{}, err
	}
	if err := s.repo.UpdateWorkspaceRootBlock(ctx, ws.ID, rootID); err != nil {
		return WorkspaceView{}, err
	}
	ws.RootBlockID = &rootID
	return workspaceView(ws), nil
}

// DeleteWorkspace hard-deletes a workspace and every row it owns (blocks,
// refs, ops, embeddings). Guards:
//   - The default workspace cannot be deleted — it's the fallback every
//     member lands in, so removing it would break the UI for the whole org.
//   - The workspace must belong to the actor's org; otherwise return
//     ErrWorkspaceNotFound (don't leak existence across tenants).
//
// Primary caller: E2E fixture teardown. Destructive on purpose — this is
// the "reclaim DB space after a test run" endpoint, not a UI-level retire.
func (s *Service) DeleteWorkspace(ctx context.Context, actor ActorContext, wsID uuid.UUID) error {
	ws, err := s.repo.GetWorkspace(ctx, wsID)
	if err != nil {
		return err
	}
	if ws.OrganizationID != actor.OrgID {
		return blockstore.ErrWorkspaceNotFound
	}
	if ws.Slug == blockstore.DefaultWorkspaceSlug {
		return errors.New("cannot delete the default workspace")
	}
	return s.repo.DeleteWorkspaceCascade(ctx, wsID)
}
