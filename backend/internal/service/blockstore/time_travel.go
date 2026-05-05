package blockstoreservice

import (
	"context"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// BlockSnapshot is the shape returned by GetBlockAt. It keeps the wire format
// deliberately loose (JSONMap values everywhere) so callers can diff two
// snapshots without worrying about type-assertion ceremony.
type BlockSnapshot struct {
	ID       uuid.UUID          `json:"id"`
	Type     string             `json:"type"`
	Data     blockstore.JSONMap `json:"data"`
	Text     *string            `json:"text,omitempty"`
	Meta     blockstore.JSONMap `json:"meta"`
	Deleted  bool               `json:"deleted"`
	/** AtOpID echoes back the snapshot cutoff so clients can chain queries. */
	AtOpID int64 `json:"at_op_id"`
}

// GetBlockAt reconstructs `blockID`'s state at the point in time represented
// by op_id == upto (inclusive). Implementation is a linear fold over the op
// log: every createBlock / updateBlock / deleteBlock whose target_block
// matches is merged on top of the running state.
//
// Because every op carries its own forward diff in JSONB, the fold does not
// need to consult the live blocks row — which means the same method doubles
// as the primitive behind future "undo to point X" features.
func (s *Service) GetBlockAt(
	ctx context.Context,
	actor ActorContext,
	blockID uuid.UUID,
	uptoOpID int64,
) (*BlockSnapshot, error) {
	// Resolve the workspace via the live row so we can enforce org isolation
	// even when every previous revision is soft-deleted.
	live, err := s.repo.GetBlock(ctx, blockID)
	if err != nil {
		return nil, err
	}
	if err := s.assertSameOrg(ctx, actor, live.WorkspaceID); err != nil {
		return nil, err
	}
	if !extractACL(live.Meta).allows(actor.UserID, live.CreatedBy) {
		return nil, blockstore.ErrBlockForbidden
	}

	ops, err := s.collectBlockOps(ctx, live.WorkspaceID, blockID, uptoOpID)
	if err != nil {
		return nil, err
	}
	if len(ops) == 0 {
		return nil, blockstore.ErrBlockNotFound
	}

	snap := &BlockSnapshot{
		ID:     blockID,
		Data:   blockstore.JSONMap{},
		Meta:   blockstore.JSONMap{},
		AtOpID: uptoOpID,
	}
	for _, op := range ops {
		applyOpToSnapshot(snap, op)
	}
	return snap, nil
}

// collectBlockOps streams every op targeting `blockID` in this workspace up
// to and including `uptoOpID`. We page through StreamOps rather than
// introducing a dedicated query path; block histories are typically short.
func (s *Service) collectBlockOps(
	ctx context.Context,
	workspaceID, blockID uuid.UUID,
	uptoOpID int64,
) ([]*blockstore.BlockOp, error) {
	var out []*blockstore.BlockOp
	after := int64(0)
	for {
		page, err := s.repo.StreamOps(ctx, blockstore.OpStreamFilter{
			WorkspaceID: workspaceID, AfterID: after, Limit: 500,
		})
		if err != nil {
			return nil, err
		}
		if len(page) == 0 {
			break
		}
		for _, op := range page {
			if uptoOpID > 0 && op.ID > uptoOpID {
				return out, nil
			}
			if op.TargetBlock != nil && *op.TargetBlock == blockID {
				out = append(out, op)
			}
		}
		after = page[len(page)-1].ID
	}
	return out, nil
}

// applyOpToSnapshot folds a single op into the running snapshot by inspecting
// its forward diff. Fields absent from forward are preserved.
func applyOpToSnapshot(snap *BlockSnapshot, op *blockstore.BlockOp) {
	switch op.Op {
	case blockstore.OpCreateBlock:
		if t, ok := op.Forward["type"].(string); ok {
			snap.Type = t
		}
		if data, ok := op.Forward["data"].(map[string]any); ok {
			snap.Data = blockstore.JSONMap(data)
		}
		if meta, ok := op.Forward["meta"].(map[string]any); ok {
			snap.Meta = blockstore.JSONMap(meta)
		}
		if text, ok := op.Forward["text"].(string); ok {
			snap.Text = &text
		}
	case blockstore.OpUpdateBlock:
		if data, ok := op.Forward["data"].(map[string]any); ok {
			snap.Data = blockstore.JSONMap(data)
		}
		if meta, ok := op.Forward["meta"].(map[string]any); ok {
			snap.Meta = blockstore.JSONMap(meta)
		}
		if text, ok := op.Forward["text"].(string); ok {
			snap.Text = &text
		} else if _, set := op.Forward["text"]; set {
			snap.Text = nil
		}
	case blockstore.OpDeleteBlock:
		snap.Deleted = true
	}
}
