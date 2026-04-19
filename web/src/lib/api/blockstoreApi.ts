import { orgPath, request } from "@/lib/api/base";
import type {
  ApplyOpsRequest,
  ApplyOpsResult,
  Block,
  BlockOp,
  BlockRef,
  ChildrenResult,
  SearchHit,
  Workspace,
} from "@/lib/api/blockstoreTypes";

// Thin client over /api/v1/orgs/:slug/blocks/*.
// All methods stay in a single object so callers can import once and discover
// the surface through IDE autocomplete.
export const blockstoreApi = {
  applyOps(req: ApplyOpsRequest): Promise<ApplyOpsResult> {
    return request<ApplyOpsResult>(orgPath(`/blocks/ops`), {
      method: "POST",
      body: req,
    });
  },

  listWorkspaces(): Promise<{ workspaces: Workspace[] }> {
    return request(orgPath(`/blocks/workspaces`));
  },

  ensureDefaultWorkspace(): Promise<Workspace> {
    return request<Workspace>(orgPath(`/blocks/workspaces/default`), {
      method: "POST",
    });
  },

  getBlock(id: string): Promise<Block> {
    return request<Block>(orgPath(`/blocks/${encodeURIComponent(id)}`));
  },

  listChildren(id: string, rel = "nest"): Promise<ChildrenResult> {
    return request<ChildrenResult>(
      orgPath(`/blocks/${encodeURIComponent(id)}/children?rel=${encodeURIComponent(rel)}`),
    );
  },

  listBacklinks(id: string): Promise<{ refs: BlockRef[] }> {
    return request(orgPath(`/blocks/${encodeURIComponent(id)}/backlinks`));
  },

  getSubtree(wsID: string, rootID: string, maxDepth = 64): Promise<ChildrenResult> {
    const q = new URLSearchParams({ root: rootID, max_depth: String(maxDepth) });
    return request<ChildrenResult>(
      orgPath(`/blocks/workspaces/${encodeURIComponent(wsID)}/subtree?${q.toString()}`),
    );
  },

  catchupOps(wsID: string, after = 0, limit = 200): Promise<{ ops: BlockOp[] }> {
    const q = new URLSearchParams({ after: String(after), limit: String(limit) });
    return request(
      orgPath(`/blocks/workspaces/${encodeURIComponent(wsID)}/ops?${q.toString()}`),
    );
  },

  // Tier 1: type_def blocks live outside the nest tree so subtree fetches
  // don't surface them. This endpoint returns them flat so the store can
  // populate them and `useBlockTypeSpecs` builds the live indicator registry.
  listTypeDefs(wsID: string): Promise<{ blocks: Block[] }> {
    return request(
      orgPath(`/blocks/workspaces/${encodeURIComponent(wsID)}/type-defs`),
    );
  },

  semanticSearch(
    wsID: string,
    query: string,
    opts: { topK?: number; minScore?: number; type?: string } = {},
  ): Promise<{ hits: SearchHit[] }> {
    return request(
      orgPath(`/blocks/workspaces/${encodeURIComponent(wsID)}/search`),
      {
        method: "POST",
        body: {
          query,
          top_k: opts.topK,
          min_score: opts.minScore,
          type: opts.type,
        },
      },
    );
  },
};
