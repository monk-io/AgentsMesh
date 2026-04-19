import type { Block, BlockOp, BlockRef, JSONMap } from "@/lib/api/blockstoreTypes";
import type { BlockstoreState } from "./blockstoreTypes";

// Translate one server-applied op into a store mutation.
// Actions call upsert* / remove* helpers already defined on the state.
export function applyRemoteOp(state: BlockstoreState, op: BlockOp) {
  switch (op.op) {
    case "createBlock": {
      const forward = op.forward as Partial<Block> & { id: string; type: string };
      const now = op.applied_at;
      const block: Block = {
        id: forward.id,
        workspace_id: op.workspace_id,
        type: forward.type,
        data: (forward.data as JSONMap) ?? {},
        text: (forward.text as string | null | undefined) ?? null,
        meta: (forward.meta as JSONMap) ?? {},
        created_by: op.actor_id,
        created_at: now,
        updated_at: now,
      };
      state.actions.upsertBlock(block);
      break;
    }
    case "updateBlock": {
      const forward = op.forward as JSONMap;
      const id = forward.id as string;
      const patch: Partial<Block> = {};
      if ("data" in forward) patch.data = forward.data as JSONMap;
      if ("text" in forward) patch.text = forward.text as string | null;
      if ("meta" in forward) patch.meta = forward.meta as JSONMap;
      patch.updated_at = op.applied_at;
      state.actions.updateBlockFields(id, patch);
      break;
    }
    case "deleteBlock": {
      const id = (op.forward as JSONMap).id as string;
      state.actions.removeBlock(id);
      break;
    }
    case "addRef": {
      const forward = op.forward as JSONMap;
      const ref: BlockRef = {
        id: forward.id as number,
        workspace_id: op.workspace_id,
        from_id: forward.from as string,
        to_id: forward.to as string,
        rel: forward.rel as string,
        order_key: (forward.order_key as string | null | undefined) ?? null,
        anchor: (forward.anchor as string | null | undefined) ?? null,
        meta: (forward.meta as JSONMap | undefined) ?? {},
        created_by: op.actor_id,
        created_at: op.applied_at,
        updated_at: op.applied_at,
      };
      state.actions.upsertRef(ref);
      break;
    }
    case "removeRef": {
      const refID = (op.forward as JSONMap).ref_id as number;
      state.actions.removeRef(refID);
      break;
    }
    case "updateRef": {
      const forward = op.forward as JSONMap;
      const refID = forward.ref_id as number;
      const patch: Partial<BlockRef> = {};
      if ("from" in forward) patch.from_id = forward.from as string;
      if ("order_key" in forward) patch.order_key = forward.order_key as string | null;
      if ("anchor" in forward) patch.anchor = forward.anchor as string | null;
      if ("meta" in forward) patch.meta = (forward.meta as JSONMap) ?? {};
      patch.updated_at = op.applied_at;
      state.actions.updateRefFields(refID, patch);
      break;
    }
  }
}
