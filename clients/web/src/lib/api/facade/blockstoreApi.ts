import { create as protoCreate, toBinary } from "@bufbuild/protobuf";
import { getBlockstoreService, parseWasmAny } from "@/lib/wasm-core";
import { readCurrentOrg } from "@/stores/auth";
import {
  applyOps as applyOpsConnect,
  listWorkspaces as listWorkspacesConnect,
  ensureDefaultWorkspace as ensureDefaultWorkspaceConnect,
  getSubtree as getSubtreeConnect,
  streamOps as streamOpsConnect,
  listTypeDefs as listTypeDefsConnect,
  semanticSearch as semanticSearchConnect,
} from "@/lib/api/connect/blockstoreConnect";
import {
  ApplyRemoteOpRequestSchema,
  ReplaceWorkspacesRequestSchema,
  UpsertWorkspaceRequestSchema,
  UpsertBlocksRequestSchema,
  UpsertRefsRequestSchema,
  ProjectLocalOpsRequestSchema,
} from "@proto/blockstore_state/v1/blockstore_state_pb";
import type {
  ApplyOpsRequest,
  ApplyOpsResult,
  Block,
  BlockOp,
  BlockRef,
  ChildrenResult,
  SearchHit,
  Workspace,
} from "@/lib/viewModels/blockstore";

const svc = () => getBlockstoreService();

function orgSlug(): string {
  return readCurrentOrg()?.slug ?? "";
}

function applyRemoteOpProto(op: BlockOp): void {
  const req = protoCreate(ApplyRemoteOpRequestSchema, { opJson: JSON.stringify(op) });
  svc().apply_remote_op(toBinary(ApplyRemoteOpRequestSchema, req));
}

function replaceWorkspacesProto(workspaces: Workspace[]): void {
  const req = protoCreate(ReplaceWorkspacesRequestSchema, {
    workspacesJson: JSON.stringify(workspaces),
  });
  void svc().replace_workspaces(toBinary(ReplaceWorkspacesRequestSchema, req));
}

function upsertWorkspaceProto(ws: Workspace): void {
  const req = protoCreate(UpsertWorkspaceRequestSchema, {
    workspaceJson: JSON.stringify(ws),
  });
  void svc().upsert_workspace(toBinary(UpsertWorkspaceRequestSchema, req));
}

function upsertBlocksProto(blocks: Block[]): void {
  const req = protoCreate(UpsertBlocksRequestSchema, {
    blocksJson: JSON.stringify(blocks),
  });
  void svc().upsert_blocks(toBinary(UpsertBlocksRequestSchema, req));
}

function upsertRefsProto(refs: BlockRef[]): void {
  const req = protoCreate(UpsertRefsRequestSchema, {
    refsJson: JSON.stringify(refs),
  });
  void svc().upsert_refs(toBinary(UpsertRefsRequestSchema, req));
}

function projectLocalOpsProto(req: ApplyOpsRequest, res: ApplyOpsResult): void {
  // Some test mocks omit project_local_ops; tolerate by guarding.
  const s = svc() as unknown as { project_local_ops?: (b: Uint8Array) => unknown };
  if (typeof s.project_local_ops !== "function") return;
  const envelope = protoCreate(ProjectLocalOpsRequestSchema, {
    requestJson: JSON.stringify(req),
    resultJson: JSON.stringify(res),
  });
  void s.project_local_ops(toBinary(ProjectLocalOpsRequestSchema, envelope));
}

export const blockstoreApi = {
  async applyOps(req: ApplyOpsRequest): Promise<ApplyOpsResult> {
    const res = await applyOpsConnect(orgSlug(), req);
    // Project locally so the SSOT cache reflects the optimistic mutation
    // before the WS broadcast / catchup arrives.
    projectLocalOpsProto(req, res);
    return res;
  },

  async listWorkspaces(): Promise<{ workspaces: Workspace[] }> {
    const { workspaces } = await listWorkspacesConnect(orgSlug());
    replaceWorkspacesProto(workspaces);
    return { workspaces };
  },

  async ensureDefaultWorkspace(): Promise<Workspace> {
    const ws = await ensureDefaultWorkspaceConnect(orgSlug());
    upsertWorkspaceProto(ws);
    return ws;
  },

  async getBlock(id: string): Promise<Block> {
    const raw = parseWasmAny<Block>(svc().get_block_json(id));
    if (!raw) throw new Error(`block not found: ${id}`);
    return raw;
  },

  async listChildren(id: string, _rel = "nest"): Promise<ChildrenResult> {
    const raw = svc().list_children_json(id);
    return (JSON.parse(raw) as ChildrenResult) ?? { blocks: [], refs: [] };
  },

  async listBacklinks(id: string): Promise<{ refs: BlockRef[] }> {
    const raw = svc().list_backlinks_json(id);
    return (JSON.parse(raw) as { refs: BlockRef[] }) ?? { refs: [] };
  },

  async getSubtree(wsID: string, rootID: string, maxDepth = 64): Promise<ChildrenResult> {
    const { blocks, refs } = await getSubtreeConnect(orgSlug(), wsID, rootID, maxDepth);
    upsertBlocksProto(blocks);
    upsertRefsProto(refs);
    if (svc().last_op_id(wsID) === BigInt(0)) {
      // Seed watermark so the WS filter recognises this workspace, mirroring
      // the legacy Rust load_subtree path. `set_last_op_id` is wasm-bindgen
      // i64 → JS BigInt — passing a Number throws "Cannot convert 0 to a
      // BigInt" and wedges DocumentView in the Loading state.
      svc().set_last_op_id(wsID, BigInt(0));
    }
    const result: ChildrenResult = (() => {
      try {
        return JSON.parse(svc().list_children_json(rootID)) as ChildrenResult;
      } catch {
        return { blocks: [], refs: [] };
      }
    })();
    // The root block itself isn't in list_children — splice it in for callers.
    try {
      const rootJson = svc().get_block_json(rootID);
      const root = parseWasmAny<Block>(rootJson);
      if (root && !result.blocks.some((b) => b.id === root.id)) {
        result.blocks = [root, ...result.blocks];
      }
    } catch {
      // Root not present yet — tolerate.
    }
    return result;
  },

  async catchupOps(wsID: string, _after = 0, limit = 500): Promise<{ ops: BlockOp[] }> {
    const after = svc().last_op_id(wsID);
    const { ops } = await streamOpsConnect(orgSlug(), wsID, Number(after), limit);
    // Apply each authoritative op into the SSOT cache so subsequent reads
    // reflect the converged state. apply_remote_op also bumps last_op_id.
    for (const op of ops) {
      applyRemoteOpProto(op);
    }
    // Callers historically iterated the returned ops to feed applyRemoteOp;
    // with this adapter the cache is already converged, so return empty.
    return { ops: [] };
  },

  async listTypeDefs(wsID: string): Promise<{ blocks: Block[] }> {
    const { blocks } = await listTypeDefsConnect(orgSlug(), wsID);
    upsertBlocksProto(blocks);
    return { blocks };
  },

  async semanticSearch(
    wsID: string,
    query: string,
    opts: { topK?: number; minScore?: number; type?: string } = {},
  ): Promise<{ hits: SearchHit[] }> {
    return await semanticSearchConnect(orgSlug(), wsID, query, opts);
  },
};
