import type { Block, BlockOp, BlockRef, Workspace } from "@/lib/api/blockstoreTypes";

export interface BlockstoreState {
  workspaces: Record<string, Workspace>;
  blocks: Record<string, Block>;
  refs: Record<number, BlockRef>;
  /** parent block id → sorted list of nest-child ref ids (by order_key) */
  nestChildren: Record<string, number[]>;
  /** target block id → list of backlink ref ids (non-nest) */
  backlinks: Record<string, number[]>;
  /** per-workspace progress marker for WS subscription catch-up */
  lastOpId: Record<string, number>;
  /** Transient one-shot focus request. A renderer whose block id matches grabs
   *  the DOM focus on next render and clears the signal so it fires exactly once. */
  pendingFocusBlockID: string | null;
  /** Selected block ids for batch operations (delete / duplicate / move). */
  selectedBlockIDs: string[];
  loading: boolean;
  error: string | null;

  actions: BlockstoreActions;
}

export interface BlockstoreActions {
  loadWorkspaces(): Promise<void>;
  ensureDefaultWorkspace(): Promise<Workspace>;
  loadSubtree(workspaceID: string, rootID: string): Promise<void>;
  loadTypeDefs(workspaceID: string): Promise<void>;
  catchup(workspaceID: string): Promise<void>;

  // Upsert helpers used by both local-dispatch and remote-op code paths.
  upsertBlock(b: Block): void;
  upsertRef(r: BlockRef): void;
  removeBlock(id: string): void;
  removeRef(refID: number): void;
  updateBlockFields(id: string, fields: Partial<Block>): void;
  updateRefFields(refID: number, fields: Partial<BlockRef>): void;

  // Op application (remote stream): translates op.forward into store mutations.
  applyRemoteOp(op: BlockOp): void;

  setLastOpId(workspaceID: string, id: number): void;
  /** Mark a block as "should grab focus on next render". */
  requestFocus(blockID: string): void;
  /** Consumed by the renderer that took the focus; clears the signal. */
  clearPendingFocus(): void;
  /** Toggle a block id in the selection set (used by shift/ctrl/cmd+click). */
  toggleSelection(blockID: string): void;
  clearSelection(): void;
  reset(): void;
}

// sortedInsertByOrder maintains nestChildren[parent] sorted by order_key (nulls last).
export function compareOrderKey(a: BlockRef | undefined, b: BlockRef | undefined): number {
  const ak = a?.order_key ?? null;
  const bk = b?.order_key ?? null;
  if (ak === bk) return (a?.id ?? 0) - (b?.id ?? 0);
  if (ak === null) return 1;
  if (bk === null) return -1;
  if (ak < bk) return -1;
  if (ak > bk) return 1;
  return 0;
}
