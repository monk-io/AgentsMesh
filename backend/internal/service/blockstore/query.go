package blockstoreservice

import (
	"context"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// ListChildrenResult pairs blocks with the nest ref that carried them, so the
// UI can display order_key / anchor in its renderer.
type ListChildrenResult struct {
	Blocks []*blockstore.Block     `json:"blocks"`
	Refs   []*blockstore.BlockRef  `json:"refs"`
}

func (s *Service) GetBlock(ctx context.Context, actor ActorContext, id uuid.UUID) (*blockstore.Block, error) {
	b, err := s.repo.GetBlock(ctx, id)
	if err != nil {
		return nil, err
	}
	if err := s.assertSameOrg(ctx, actor, b.WorkspaceID); err != nil {
		return nil, err
	}
	if !extractACL(b.Meta).allows(actor.UserID, b.CreatedBy) {
		return nil, blockstore.ErrBlockForbidden
	}
	return b, nil
}

func (s *Service) ListChildren(ctx context.Context, actor ActorContext, parentID uuid.UUID, rel string) (*ListChildrenResult, error) {
	parent, err := s.repo.GetBlock(ctx, parentID)
	if err != nil {
		return nil, err
	}
	if err := s.assertSameOrg(ctx, actor, parent.WorkspaceID); err != nil {
		return nil, err
	}
	if !extractACL(parent.Meta).allows(actor.UserID, parent.CreatedBy) {
		return nil, blockstore.ErrBlockForbidden
	}
	if rel == "" {
		rel = blockstore.RelNest
	}
	blocks, refs, err := s.repo.ListChildren(ctx, parentID, rel)
	if err != nil {
		return nil, err
	}
	// Filter private children the caller cannot see.
	visibleBlocks, visibleRefs := filterByACL(blocks, refs, actor.UserID)
	return &ListChildrenResult{Blocks: visibleBlocks, Refs: visibleRefs}, nil
}

func (s *Service) ListBacklinks(ctx context.Context, actor ActorContext, targetID uuid.UUID) ([]*blockstore.BlockRef, error) {
	target, err := s.repo.GetBlock(ctx, targetID)
	if err != nil {
		return nil, err
	}
	if err := s.assertSameOrg(ctx, actor, target.WorkspaceID); err != nil {
		return nil, err
	}
	if !extractACL(target.Meta).allows(actor.UserID, target.CreatedBy) {
		return nil, blockstore.ErrBlockForbidden
	}
	return s.repo.ListBacklinks(ctx, targetID, true)
}

// ListSubtree returns the nest tree rooted at rootID, flat-packed with its
// connecting refs. Frontend uses this as the initial snapshot before the
// streaming subscription takes over.
func (s *Service) ListSubtree(ctx context.Context, actor ActorContext, wsID, rootID uuid.UUID, maxDepth int) (*ListChildrenResult, error) {
	if err := s.assertSameOrg(ctx, actor, wsID); err != nil {
		return nil, err
	}
	blocks, refs, err := s.repo.ListWorkspaceSubtree(ctx, wsID, rootID, maxDepth)
	if err != nil {
		return nil, err
	}
	visibleBlocks, visibleRefs := filterByACL(blocks, refs, actor.UserID)
	return &ListChildrenResult{Blocks: visibleBlocks, Refs: visibleRefs}, nil
}

// filterByACL drops blocks whose meta.acl forbids the actor, plus any ref that
// touches a dropped block. List views never reveal the existence of private
// blocks the caller cannot read.
func filterByACL(blocks []*blockstore.Block, refs []*blockstore.BlockRef, userID int64) ([]*blockstore.Block, []*blockstore.BlockRef) {
	visible := make(map[uuid.UUID]bool, len(blocks))
	keptBlocks := make([]*blockstore.Block, 0, len(blocks))
	for _, b := range blocks {
		if !extractACL(b.Meta).allows(userID, b.CreatedBy) {
			continue
		}
		visible[b.ID] = true
		keptBlocks = append(keptBlocks, b)
	}
	keptRefs := make([]*blockstore.BlockRef, 0, len(refs))
	for _, r := range refs {
		if visible[r.FromID] && visible[r.ToID] {
			keptRefs = append(keptRefs, r)
		}
	}
	return keptBlocks, keptRefs
}

// StreamOps returns ops strictly after afterID, bounded to limit rows.
// Used by clients reconnecting with a last_op_id checkpoint.
func (s *Service) StreamOps(ctx context.Context, actor ActorContext, wsID uuid.UUID, afterID int64, limit int) ([]*blockstore.BlockOp, error) {
	if err := s.assertSameOrg(ctx, actor, wsID); err != nil {
		return nil, err
	}
	if limit <= 0 || limit > 1000 {
		limit = 200
	}
	return s.repo.StreamOps(ctx, blockstore.OpStreamFilter{
		WorkspaceID: wsID, AfterID: afterID, Limit: limit,
	})
}

// ListRegisteredTypes returns every block type registered for the workspace,
// bootstrap + dynamic union. Exposed for MCP tool enumeration.
func (s *Service) ListRegisteredTypes(
	ctx context.Context,
	actor ActorContext,
	wsID uuid.UUID,
) ([]blockstore.BlockTypeSpec, error) {
	if err := s.assertSameOrg(ctx, actor, wsID); err != nil {
		return nil, err
	}
	return s.listAllTypes(ctx, wsID), nil
}

func (s *Service) assertSameOrg(ctx context.Context, actor ActorContext, wsID uuid.UUID) error {
	ws, err := s.repo.GetWorkspace(ctx, wsID)
	if err != nil {
		return err
	}
	if ws.OrganizationID != actor.OrgID {
		return blockstore.ErrOrgMismatch
	}
	return nil
}

// ListTypeDefBlocks returns every block_type_def in the workspace as raw
// Block rows. Used by the frontend's RecordEditor pipeline: type_def blocks
// live outside the nest hierarchy, so subtree fetches don't surface them on
// first load. The MCP ListRegisteredTypes endpoint returns hydrated specs;
// this returns the underlying blocks so the store can index them and keep
// the live `useBlockTypeSpecs` hook in sync with future op updates.
func (s *Service) ListTypeDefBlocks(
	ctx context.Context,
	actor ActorContext,
	wsID uuid.UUID,
) ([]*blockstore.Block, error) {
	if err := s.assertSameOrg(ctx, actor, wsID); err != nil {
		return nil, err
	}
	def := blockstore.BlockTypeTypeDef
	blocks, _, err := s.repo.ListBlocks(ctx, blockstore.BlockFilter{
		WorkspaceID: wsID,
		Type:        &def,
	})
	if err != nil {
		return nil, err
	}
	out := make([]*blockstore.Block, 0, len(blocks))
	for _, b := range blocks {
		if extractACL(b.Meta).allows(actor.UserID, b.CreatedBy) {
			out = append(out, b)
		}
	}
	return out, nil
}
