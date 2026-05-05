package blockstoreservice

import (
	"context"
	"encoding/json"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// dispatchTriggers runs after an ApplyOps commit. It walks every applied op,
// filters to the ones that could match a trigger (create/update/delete on a
// block), loads the workspace's enabled trigger_defs once, evaluates
// predicates against the target block's data, and fires matching actions.
//
// Execution is best-effort and fully async — triggers never block the write
// path, and a failing webhook logs + retries once, no more. Trigger
// reliability semantics land in a future "delivery queue" phase if the
// volume justifies the complexity.
func (s *Service) dispatchTriggers(ctx context.Context, wsID uuid.UUID, ops []*blockstore.BlockOp) {
	triggers, err := s.loadTriggers(ctx, wsID)
	if err != nil {
		s.logger.Warn("blockstore.trigger.load_failed", "err", err.Error())
		return
	}
	if len(triggers) == 0 {
		return
	}
	for _, op := range ops {
		if op.TargetBlock == nil {
			continue
		}
		event := opToTriggerEvent(op.Op)
		if event == "" {
			continue
		}
		target, err := s.repo.GetBlock(ctx, *op.TargetBlock)
		if err != nil {
			continue
		}
		for _, t := range triggers {
			if !t.Enabled || t.On != event || t.TargetType != target.Type {
				continue
			}
			if t.Predicate != "" && !evalTriggerPredicate(t.Predicate, target.Data) {
				continue
			}
			go s.fireTrigger(context.Background(), t, target, op)
		}
	}
}

// fireTrigger routes to the per-action-kind implementation. Webhook makes an
// outbound HTTP POST; agent writes an agent_event block the target agent
// consumes via memory.retrieve / subtree query. The agent_event block path
// is a polling model — not true push — but it keeps trigger output durable,
// auditable, and uniform with every other Block Store write.
func (s *Service) fireTrigger(ctx context.Context, t TriggerDef, target *blockstore.Block, op *blockstore.BlockOp) {
	switch t.Action.Kind {
	case "webhook":
		s.fireWebhook(ctx, t, target, op)
	case "agent":
		s.fireAgentAction(ctx, t, target, op)
	default:
		s.logger.Warn("blockstore.trigger.unknown_action_kind",
			"kind", t.Action.Kind, "trigger", t.Name)
	}
}

func opToTriggerEvent(op string) string {
	switch op {
	case blockstore.OpCreateBlock:
		return "create"
	case blockstore.OpUpdateBlock:
		return "update"
	case blockstore.OpDeleteBlock:
		return "delete"
	default:
		return ""
	}
}

// loadTriggers pulls every trigger_def in the workspace and decodes its
// data. Invalid rows are skipped — the logger surfaces malformed entries
// so operators can fix them without breaking live workflow.
func (s *Service) loadTriggers(ctx context.Context, wsID uuid.UUID) ([]TriggerDef, error) {
	def := blockstore.BlockTypeTriggerDef
	blocks, _, err := s.repo.ListBlocks(ctx, blockstore.BlockFilter{
		WorkspaceID: wsID,
		Type:        &def,
	})
	if err != nil {
		return nil, err
	}
	out := make([]TriggerDef, 0, len(blocks))
	for _, b := range blocks {
		t, ok := decodeTrigger(b.Data)
		if !ok {
			continue
		}
		// Stamp CreatedBy from the owning block so downstream side-effects
		// can attribute writes to a real user (权限跟着人走).
		t.CreatedBy = b.CreatedBy
		out = append(out, t)
	}
	return out, nil
}

func decodeTrigger(data blockstore.JSONMap) (TriggerDef, bool) {
	raw, err := json.Marshal(data)
	if err != nil {
		return TriggerDef{}, false
	}
	var t TriggerDef
	if err := json.Unmarshal(raw, &t); err != nil {
		return TriggerDef{}, false
	}
	if t.Name == "" || t.TargetType == "" || t.On == "" || t.Action.Kind == "" {
		return TriggerDef{}, false
	}
	return t, true
}
