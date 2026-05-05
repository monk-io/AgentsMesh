package blockstoreservice

import (
	"context"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/infra/otel"
	"github.com/google/uuid"
	otelmetric "go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/attribute"
)

// ApplyOps is the sole write entry point. It runs the entire batch inside a
// workspace-scoped advisory-locked transaction. Any single op failure rolls
// back the whole batch and returns the concrete domain error.
//
// Idempotency: if IdempotencyKey is non-empty and already recorded, the stored
// op ids are returned without re-executing. This makes the batch safe to retry
// on network failure.
func (s *Service) ApplyOps(ctx context.Context, actor ActorContext, in ApplyOpsInput) (*ApplyOpsResult, error) {
	// Fires once, lazily, on the first real write — so operators who forget
	// to call SetEmbedder in production still see a warning in their logs
	// rather than silently running semantic search on bag-of-words vectors.
	s.WarnIfDefaultEmbedder()
	start := time.Now()
	defer func() {
		otel.BlockstoreOpsDuration.Record(ctx, float64(time.Since(start).Milliseconds()),
			otelmetric.WithAttributes(attribute.String("actor_type", actor.ActorType)))
	}()
	if len(in.Ops) == 0 {
		return nil, blockstore.ErrApplyOpsEmpty
	}
	wsID, err := uuid.Parse(in.WorkspaceID)
	if err != nil {
		return nil, err
	}
	ws, err := s.repo.GetWorkspace(ctx, wsID)
	if err != nil {
		return nil, err
	}
	if ws.OrganizationID != actor.OrgID {
		return nil, blockstore.ErrOrgMismatch
	}

	var applied []*blockstore.BlockOp
	result := &ApplyOpsResult{ParentOpID: in.ParentOpID}

	err = s.repo.WithinWorkspaceTx(ctx, wsID, func(tx blockstore.TxWriter) error {
		// Idempotency short-circuit: if key already processed, find the
		// op chain parented by its first entry and return those ids.
		if in.IdempotencyKey != "" {
			existing, err := tx.FindOpByIdempotencyKey(ctx, in.IdempotencyKey)
			if err != nil {
				return err
			}
			if existing != nil {
				result.WasReplay = true
				result.OpIDs = append(result.OpIDs, existing.ID)
				// Fetch sibling ops (parent_op_id == existing.ID) so the
				// replay response mirrors the original batch exactly. Without
				// this, a client retrying a 2-op batch would see only the
				// first op_id and incorrectly assume the second never applied.
				siblings, err := tx.ListOpsByParent(ctx, existing.ID)
				if err != nil {
					return err
				}
				for _, sib := range siblings {
					result.OpIDs = append(result.OpIDs, sib.ID)
				}
				return nil
			}
		}

		var parentOp *int64
		if in.ParentOpID != nil {
			parentOp = in.ParentOpID
		}
		var firstOpID int64

		for idx, envelope := range in.Ops {
			if err := ensureValidOp(envelope.Op); err != nil {
				return err
			}
			op, err := s.applyOne(ctx, tx, actor, envelope, wsID)
			if err != nil {
				return err
			}
			// First op of the batch carries idempotency_key; siblings link via parent_op_id.
			if idx == 0 {
				if in.IdempotencyKey != "" {
					op.IdempotencyKey = &in.IdempotencyKey
				}
				op.ParentOpID = parentOp
			} else {
				op.ParentOpID = &firstOpID
			}
			id, err := tx.InsertOp(ctx, op)
			if err != nil {
				return err
			}
			op.ID = id
			if idx == 0 {
				firstOpID = id
			}
			applied = append(applied, op)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	for _, op := range applied {
		result.OpIDs = append(result.OpIDs, op.ID)
		otel.BlockstoreOpsApplied.Add(ctx, 1,
			otelmetric.WithAttributes(
				attribute.String("op_kind", op.Op),
				attribute.String("actor_type", actor.ActorType),
			))
	}

	// Publish after successful commit; best-effort, failure is logged only.
	if s.publisher != nil && !result.WasReplay {
		s.publisher.PublishBatch(ctx, actor.OrgID, applied)
	}
	// Embedding refresh runs off the write path entirely so a slow provider
	// (OpenAI API latency, cold network) doesn't stall ApplyOps.
	if !result.WasReplay {
		s.enqueueEmbeddings(applied)
	}
	// Tier 3: trigger dispatch. Matching runs synchronously so each op
	// evaluates against its own post-commit state (GetBlock sees the just-
	// applied diff, not a later op's). Action firing is dispatched to its
	// own goroutine inside dispatchTriggers so slow webhooks never stall
	// the write path. SuppressTriggers is set by internal callers
	// (fireAgentAction) to prevent cascading trigger→agent_event loops.
	if !result.WasReplay && !in.SuppressTriggers {
		s.dispatchTriggers(ctx, wsID, applied)
	}
	return result, nil
}

// applyOne dispatches a single envelope to its per-kind handler.
// Handlers return a partially-populated BlockOp (forward/inverse/payload/op)
// that ApplyOps then augments with idempotency/parent_op_id.
func (s *Service) applyOne(
	ctx context.Context,
	tx blockstore.TxWriter,
	actor ActorContext,
	env OpEnvelope,
	wsID uuid.UUID,
) (*blockstore.BlockOp, error) {
	switch env.Op {
	case blockstore.OpCreateBlock:
		return s.applyCreateBlock(ctx, tx, actor, env.Payload, wsID)
	case blockstore.OpUpdateBlock:
		return s.applyUpdateBlock(ctx, tx, actor, env.Payload, wsID)
	case blockstore.OpDeleteBlock:
		return s.applyDeleteBlock(ctx, tx, actor, env.Payload, wsID)
	case blockstore.OpAddRef:
		return s.applyAddRef(ctx, tx, actor, env.Payload, wsID)
	case blockstore.OpRemoveRef:
		return s.applyRemoveRef(ctx, tx, actor, env.Payload, wsID)
	case blockstore.OpUpdateRef:
		return s.applyUpdateRef(ctx, tx, actor, env.Payload, wsID)
	default:
		return nil, blockstore.ErrUnknownOpKind
	}
}
