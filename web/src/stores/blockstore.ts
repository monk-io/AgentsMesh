import { create } from "zustand";

import { blockstoreApi } from "@/lib/api/blockstoreApi";
import type { Block, BlockRef, BlockOp } from "@/lib/api/blockstoreTypes";
import { applyRemoteOp } from "./blockstoreOpApply";
import { compareOrderKey, type BlockstoreState } from "./blockstoreTypes";

// Zustand store for Block Store Phase 1.
// Keeps blocks/refs as flat maps plus two derived indexes (nestChildren / backlinks)
// to keep render paths O(1).
export const useBlockstoreStore = create<BlockstoreState>((set, get) => ({
  workspaces: {},
  blocks: {},
  refs: {},
  nestChildren: {},
  backlinks: {},
  lastOpId: {},
  pendingFocusBlockID: null,
  selectedBlockIDs: [],
  loading: false,
  error: null,

  actions: {
    async loadWorkspaces() {
      set({ loading: true, error: null });
      try {
        const { workspaces } = await blockstoreApi.listWorkspaces();
        const map: Record<string, (typeof workspaces)[number]> = {};
        workspaces.forEach((w) => (map[w.id] = w));
        set({ workspaces: map, loading: false });
      } catch (e) {
        set({ loading: false, error: (e as Error).message });
      }
    },

    async ensureDefaultWorkspace() {
      const ws = await blockstoreApi.ensureDefaultWorkspace();
      set((s) => ({ workspaces: { ...s.workspaces, [ws.id]: ws } }));
      return ws;
    },

    async loadSubtree(workspaceID, rootID) {
      const res = await blockstoreApi.getSubtree(workspaceID, rootID);
      set((s) => {
        const blocks = { ...s.blocks };
        res.blocks.forEach((b) => (blocks[b.id] = b));
        const refs = { ...s.refs };
        const nestChildren = { ...s.nestChildren };
        const backlinks = { ...s.backlinks };
        res.refs.forEach((r) => {
          refs[r.id] = r;
          indexRef(r, refs, nestChildren, backlinks);
        });
        // Seed lastOpId so this workspace is recognised as "subscribed" by
        // the WS filter (blockstoreSubscribe.ts). Without this a user would
        // miss their first ops until one happened to set the marker.
        const lastOpId =
          workspaceID in s.lastOpId ? s.lastOpId : { ...s.lastOpId, [workspaceID]: 0 };
        return { blocks, refs, nestChildren, backlinks, lastOpId };
      });
    },

    async loadTypeDefs(workspaceID) {
      // Type_def blocks live outside the nest tree, so getSubtree never
      // surfaces them. Call this on workspace load so `useBlockTypeSpecs`
      // can build the indicator registry for slash menu + RecordEditor.
      const res = await blockstoreApi.listTypeDefs(workspaceID);
      set((s) => {
        const blocks = { ...s.blocks };
        res.blocks.forEach((b) => (blocks[b.id] = b));
        return { blocks };
      });
    },

    async catchup(workspaceID) {
      const after = get().lastOpId[workspaceID] ?? 0;
      const { ops } = await blockstoreApi.catchupOps(workspaceID, after, 500);
      ops.forEach((op) => get().actions.applyRemoteOp(op));
      if (ops.length > 0) {
        get().actions.setLastOpId(workspaceID, ops[ops.length - 1].id);
      }
    },

    upsertBlock(b: Block) {
      set((s) => ({ blocks: { ...s.blocks, [b.id]: b } }));
    },

    upsertRef(r: BlockRef) {
      set((s) => {
        const refs = { ...s.refs, [r.id]: r };
        const nestChildren = { ...s.nestChildren };
        const backlinks = { ...s.backlinks };
        indexRef(r, refs, nestChildren, backlinks);
        return { refs, nestChildren, backlinks };
      });
    },

    removeBlock(id: string) {
      set((s) => {
        if (!s.blocks[id]) return s;
        // Cascade: drop every ref that touches this block on either side, and
        // clean the derived indexes (nestChildren / backlinks) so no ghost
        // entries linger when the same id is later reused by time-travel or
        // an undo op. Matches the server's soft-delete semantics — the ref
        // rows still exist in DB but the client treats them as invisible
        // once the target is gone.
        const refs = { ...s.refs };
        const nestChildren = { ...s.nestChildren };
        const backlinks = { ...s.backlinks };
        for (const ref of Object.values(s.refs)) {
          if (ref.from_id === id || ref.to_id === id) {
            delete refs[ref.id];
            unindexRef(ref, nestChildren, backlinks);
          }
        }
        // Also clear any dangling index entries keyed on this block id itself
        // (parent index or backlinks bucket where the block was the target).
        delete nestChildren[id];
        delete backlinks[id];
        const blocks = { ...s.blocks };
        delete blocks[id];
        return { blocks, refs, nestChildren, backlinks };
      });
    },

    removeRef(refID: number) {
      set((s) => {
        const existing = s.refs[refID];
        if (!existing) return s;
        const refs = { ...s.refs };
        delete refs[refID];
        const nestChildren = { ...s.nestChildren };
        const backlinks = { ...s.backlinks };
        unindexRef(existing, nestChildren, backlinks);
        return { refs, nestChildren, backlinks };
      });
    },

    updateBlockFields(id: string, fields: Partial<Block>) {
      set((s) => {
        const existing = s.blocks[id];
        if (!existing) return s;
        return { blocks: { ...s.blocks, [id]: { ...existing, ...fields } } };
      });
    },

    updateRefFields(refID: number, fields: Partial<BlockRef>) {
      set((s) => {
        const prev = s.refs[refID];
        if (!prev) return s;
        const next = { ...prev, ...fields };
        const refs = { ...s.refs, [refID]: next };
        const nestChildren = { ...s.nestChildren };
        const backlinks = { ...s.backlinks };
        unindexRef(prev, nestChildren, backlinks);
        indexRef(next, refs, nestChildren, backlinks);
        return { refs, nestChildren, backlinks };
      });
    },

    applyRemoteOp(op: BlockOp) {
      applyRemoteOp(get(), op);
      const current = get().lastOpId[op.workspace_id] ?? 0;
      if (op.id > current) {
        get().actions.setLastOpId(op.workspace_id, op.id);
      }
    },

    setLastOpId(workspaceID: string, id: number) {
      set((s) => ({ lastOpId: { ...s.lastOpId, [workspaceID]: id } }));
    },

    requestFocus(blockID: string) {
      set({ pendingFocusBlockID: blockID });
    },

    clearPendingFocus() {
      set((s) => (s.pendingFocusBlockID === null ? s : { pendingFocusBlockID: null }));
    },

    toggleSelection(blockID: string) {
      set((s) => {
        const has = s.selectedBlockIDs.includes(blockID);
        return {
          selectedBlockIDs: has
            ? s.selectedBlockIDs.filter((id) => id !== blockID)
            : [...s.selectedBlockIDs, blockID],
        };
      });
    },

    clearSelection() {
      set((s) => (s.selectedBlockIDs.length === 0 ? s : { selectedBlockIDs: [] }));
    },

    reset() {
      set({
        workspaces: {}, blocks: {}, refs: {},
        nestChildren: {}, backlinks: {}, lastOpId: {},
        pendingFocusBlockID: null, selectedBlockIDs: [],
        loading: false, error: null,
      });
    },
  },
}));

function indexRef(
  r: BlockRef,
  refs: Record<number, BlockRef>,
  nestChildren: Record<string, number[]>,
  backlinks: Record<string, number[]>,
) {
  if (r.rel === "nest") {
    const existing = nestChildren[r.from_id] ?? [];
    if (existing.includes(r.id)) return;
    const list = [...existing, r.id];
    list.sort((a, b) => compareOrderKey(refs[a], refs[b]));
    nestChildren[r.from_id] = list;
  } else {
    const existing = backlinks[r.to_id] ?? [];
    if (existing.includes(r.id)) return;
    backlinks[r.to_id] = [...existing, r.id];
  }
}

function unindexRef(
  r: BlockRef,
  nestChildren: Record<string, number[]>,
  backlinks: Record<string, number[]>,
) {
  if (r.rel === "nest") {
    const list = (nestChildren[r.from_id] ?? []).filter((id) => id !== r.id);
    if (list.length === 0) delete nestChildren[r.from_id];
    else nestChildren[r.from_id] = list;
  } else {
    const list = (backlinks[r.to_id] ?? []).filter((id) => id !== r.id);
    if (list.length === 0) delete backlinks[r.to_id];
    else backlinks[r.to_id] = list;
  }
}
