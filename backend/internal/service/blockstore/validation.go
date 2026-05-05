package blockstoreservice

import (
	"context"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// checkSameWorkspace asserts that both from and to blocks belong to wsID.
// Cross-workspace refs are reserved for Phase 2+ business rels.
func checkSameWorkspace(ctx context.Context, tx blockstore.TxWriter, wsID, from, to uuid.UUID) error {
	fromBlock, err := tx.FindBlockByID(ctx, from)
	if err != nil {
		return err
	}
	if fromBlock.WorkspaceID != wsID {
		return blockstore.ErrCrossWorkspaceRef
	}
	toBlock, err := tx.FindBlockByID(ctx, to)
	if err != nil {
		return err
	}
	if toBlock.WorkspaceID != wsID {
		return blockstore.ErrCrossWorkspaceRef
	}
	return nil
}

// ensureNoCycle walks up `from`'s ancestor chain (via rel='nest') and fails if
// `to` appears. A self-nest (from == to) is also a cycle.
func ensureNoCycle(ctx context.Context, tx blockstore.TxWriter, from, to uuid.UUID) error {
	if from == to {
		return blockstore.ErrNestCycle
	}
	ancestors, err := tx.FindAncestors(ctx, from, 64)
	if err != nil {
		return err
	}
	for _, a := range ancestors {
		if a == to {
			return blockstore.ErrNestCycle
		}
	}
	return nil
}

// ensureChildAllowed consults the workspace-aware type resolver to block
// incompatible nest relations (e.g. paragraph → page). Dynamic block_type_def
// rows take precedence over the bootstrap registry.
func (s *Service) ensureChildAllowed(
	ctx context.Context,
	tx blockstore.TxWriter,
	wsID uuid.UUID,
	parent, child uuid.UUID,
) error {
	parentBlock, err := tx.FindBlockByID(ctx, parent)
	if err != nil {
		return err
	}
	childBlock, err := tx.FindBlockByID(ctx, child)
	if err != nil {
		return err
	}
	spec, ok := s.resolveTypeSpecInTx(ctx, tx, parentBlock.Type)
	if !ok {
		return blockstore.ErrUnknownBlockType
	}
	if !spec.IsChildAllowed(childBlock.Type) {
		return blockstore.ErrChildNotAllowed
	}
	return nil
}
