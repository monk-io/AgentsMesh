import { invoke } from "./invoke";
import { fromBinary, toBinary, create as protoCreate } from "@bufbuild/protobuf";
import {
  ApplyRemoteOpRequestSchema,
  ReplaceWorkspacesRequestSchema,
  UpsertWorkspaceRequestSchema,
  UpsertBlocksRequestSchema,
  UpsertRefsRequestSchema,
  ProjectLocalOpsRequestSchema,
} from "@agentsmesh/proto/blockstore_state/v1/blockstore_state_pb";
import type {
  Workspace as ProtoWorkspace,
  Block as ProtoBlock,
  BlockRef as ProtoBlockRef,
} from "@agentsmesh/proto/blockstore/v1/blockstore_pb";
import { applyOpToCache, safeJsonMap, type CacheState } from "./blockstore-apply";
import {
  hydrateSubtree, upsertBlocks, upsertRefs, upsertWorkspace, replaceWorkspaces,
  type WorkspacesRef,
} from "./blockstore-cache";

// Typed proto.blockstore.v1.* (camelCase + BigInt) → renderer cache shape
// (snake_case + number). Outer container fields are flat; only data/meta
// are inner JSON strings (per-blocktype variant schemas, kept opaque).
function protoWorkspaceToCache(w: ProtoWorkspace): Record<string, unknown> {
  return {
    id: w.id,
    organization_id: Number(w.organizationId),
    slug: w.slug, name: w.name,
    root_block_id: w.rootBlockId,
    created_at: w.createdAt,
  };
}
function protoBlockToCache(b: ProtoBlock): Record<string, unknown> {
  const data = b.dataJson ? safeParse(b.dataJson) : {};
  const meta = b.metaJson ? safeParse(b.metaJson) : {};
  return {
    id: b.id, workspace_id: b.workspaceId, type: b.type,
    data, text: b.text, meta,
    created_by: Number(b.createdBy),
    created_at: b.createdAt, updated_at: b.updatedAt,
    deleted_at: b.deletedAt,
  };
}
function protoBlockRefToCache(r: ProtoBlockRef): Record<string, unknown> {
  const meta = r.metaJson ? safeParse(r.metaJson) : {};
  return {
    id: Number(r.id), workspace_id: r.workspaceId,
    from_id: r.fromId, to_id: r.toId, rel: r.rel,
    order_key: r.orderKey, anchor: r.anchor,
    meta, created_by: Number(r.createdBy),
    created_at: r.createdAt, updated_at: r.updatedAt,
  };
}
function safeParse(s: string): unknown {
  try { return JSON.parse(s); } catch { return {}; }
}

/**
 * Desktop facade for the wasm-shaped BlockstoreService. The renderer cache is
 * the SSOT after the R6 Connect flip — main-process Rust state is kept warm
 * for legacy IPC consumers only. Synchronous mutators apply to `this.cache`
 * first so zustand selectors converge in the same React commit; IPC fires
 * fire-and-forget to mirror state in main.
 */
export class ElectronBlockstoreService {
  private blockCache = new Map<string, string>();
  private childrenCache = new Map<string, string>();
  private typeDefsCache = new Map<string, string>();
  private backlinksCache = new Map<string, string>();

  private cache: CacheState = {
    blocksJson: "{}", refsJson: "{}", nestChildrenJson: "{}",
    backlinksJsonFlat: "{}", lastOpIdsJson: "{}",
  };
  private workspaces: WorkspacesRef = { value: "{}" };

  private async refreshFlatCaches(): Promise<void> {
    const [blocks, refs, nestChildren, backlinks, lastOpIds, workspaces] = await Promise.all([
      invoke<string>("blockstoreBlocksJson"),
      invoke<string>("blockstoreRefsJson"),
      invoke<string>("blockstoreNestChildrenJson"),
      invoke<string>("blockstoreBacklinksJson"),
      invoke<string>("blockstoreLastOpIdsJson"),
      invoke<string>("blockstoreWorkspacesJson"),
    ]);
    this.cache.blocksJson = blocks || "{}";
    this.cache.refsJson = refs || "{}";
    this.cache.nestChildrenJson = nestChildren || "{}";
    this.cache.backlinksJsonFlat = backlinks || "{}";
    this.cache.lastOpIdsJson = lastOpIds || "{}";
    this.workspaces.value = workspaces || "{}";
  }

  async apply_ops(reqJson: string): Promise<string> {
    const result = await invoke<string>("blockstoreApplyOps", reqJson);
    await this.refreshFlatCaches();
    return result;
  }

  async list_workspaces(): Promise<string> {
    const result = await invoke<string>("blockstoreListWorkspaces");
    await this.refreshFlatCaches();
    return result;
  }

  async ensure_default_workspace(): Promise<string> {
    const result = await invoke<string>("blockstoreEnsureDefaultWorkspace");
    await this.refreshFlatCaches();
    return result;
  }

  async load_subtree(workspaceId: string, rootId: string): Promise<void> {
    await invoke("blockstoreLoadSubtree", workspaceId, rootId);
    await hydrateSubtree(rootId, this.blockCache, this.childrenCache);
    await this.refreshFlatCaches();
  }

  async load_type_defs(workspaceId: string): Promise<void> {
    await invoke("blockstoreLoadTypeDefs", workspaceId);
    const json = await invoke<string>("blockstoreTypeDefsJson", workspaceId);
    this.typeDefsCache.set(workspaceId, json);
  }

  async catchup(workspaceId: string): Promise<void> {
    await invoke("blockstoreCatchup", workspaceId);
  }

  async semantic_search(workspaceId: string, reqJson: string): Promise<string> {
    return invoke<string>("blockstoreSemanticSearch", workspaceId, reqJson);
  }

  apply_remote_op(reqBytes: Uint8Array): void {
    const req = fromBinary(ApplyRemoteOpRequestSchema, reqBytes);
    let normalized = req.opJson;
    let parsed: Record<string, unknown> | null = null;
    try {
      parsed = JSON.parse(req.opJson) as Record<string, unknown>;
      // Backend WS pushes applied_at as Unix ms; Rust BlockOp expects ISO string.
      if (typeof parsed.applied_at === "number") {
        parsed.applied_at = new Date(parsed.applied_at).toISOString();
        normalized = JSON.stringify(parsed);
      }
    } catch { /* fall through with original payload */ }
    if (parsed) {
      try { applyOpToCache(this.cache, parsed as unknown as Parameters<typeof applyOpToCache>[1]); }
      catch { /* tolerate malformed ops */ }
    }
    // Fire IPC to keep main-process mirror warm for legacy consumers. Renormalise
    // applied_at into the proto envelope before forwarding so main-side serde
    // decodes BlockOp.applied_at as string. We don't await — main is racing the
    // catchup loop and an in-flight refresh would overwrite the renderer cache
    // with a stale snapshot before later ops in the same loop apply.
    const outgoing = normalized === req.opJson
      ? reqBytes
      : toBinary(ApplyRemoteOpRequestSchema, protoCreate(ApplyRemoteOpRequestSchema, { opJson: normalized }));
    void invoke("blockstoreApplyRemoteOp", Array.from(outgoing));
  }

  // Mirrors services::blockstore::apply_local_ops — skips ref ops (server
  // assigns ref_id) and projects the rest using the server-returned op_ids.
  project_local_ops(reqBytes: Uint8Array): void {
    let envelope: { request?: { workspaceId?: string; ops?: Array<{ op: string; payloadJson?: string }> }; result?: { opIds?: bigint[]; wasReplay?: boolean } };
    try {
      envelope = fromBinary(ProjectLocalOpsRequestSchema, reqBytes);
    } catch { return; }
    const req = envelope.request;
    const res = envelope.result;
    if (!req || !res || res.wasReplay) {
      // still fan out so main-process cache mirrors the no-op
      void invoke("blockstoreProjectLocalOps", Array.from(reqBytes));
      return;
    }
    const wsId = req.workspaceId ?? "";
    const ops = req.ops ?? [];
    const opIds = res.opIds ?? [];
    for (let i = 0; i < ops.length; i++) {
      const env = ops[i];
      const opIdRaw = opIds[i];
      if (opIdRaw === undefined) continue;
      if (env.op === "addRef" || env.op === "removeRef" || env.op === "updateRef") continue;
      let forward: Record<string, unknown> = {};
      try { forward = env.payloadJson ? JSON.parse(env.payloadJson) as Record<string, unknown> : {}; }
      catch { /* tolerate malformed payload */ }
      applyOpToCache(this.cache, {
        id: Number(opIdRaw), workspace_id: wsId, op: env.op,
        forward, applied_at: "",
      });
    }
    void invoke("blockstoreProjectLocalOps", Array.from(reqBytes));
  }

  workspaces_json(): string { return this.workspaces.value; }
  blocks_json(): string { return this.cache.blocksJson; }
  refs_json(): string { return this.cache.refsJson; }
  nest_children_json(): string { return this.cache.nestChildrenJson; }
  backlinks_json(): string { return this.cache.backlinksJsonFlat; }
  last_op_ids_json(): string { return this.cache.lastOpIdsJson; }

  get_block_json(id: string): string | null { return this.blockCache.get(id) ?? null; }
  list_children_json(parentId: string): string {
    return this.childrenCache.get(parentId) ?? '{"blocks":[],"refs":[]}';
  }
  list_backlinks_json(targetId: string): string {
    return this.backlinksCache.get(targetId) ?? '{"refs":[]}';
  }
  type_defs_json(workspaceId: string): string {
    return this.typeDefsCache.get(workspaceId) ?? '{"blocks":[]}';
  }

  upsert_blocks(reqBytes: Uint8Array): void {
    try {
      const { blocks } = fromBinary(UpsertBlocksRequestSchema, reqBytes);
      // The upsertBlocks helper expects JSON-array shape; reconstruct it
      // from the typed proto Blocks (data_json/meta_json are already JSON
      // strings inside the proto, parse them back to objects per blockstore_types.Block).
      const json = JSON.stringify(blocks.map(protoBlockToCache));
      upsertBlocks(this.cache, this.blockCache, json);
      void invoke("blockstoreUpsertBlocks", Array.from(reqBytes));
    } catch { /* tolerate malformed bytes */ }
  }
  upsert_refs(reqBytes: Uint8Array): void {
    try {
      const { refs } = fromBinary(UpsertRefsRequestSchema, reqBytes);
      const json = JSON.stringify(refs.map(protoBlockRefToCache));
      upsertRefs(this.cache, json);
      void invoke("blockstoreUpsertRefs", Array.from(reqBytes));
    } catch { /* tolerate malformed bytes */ }
  }
  upsert_workspace(reqBytes: Uint8Array): void {
    try {
      const { workspace } = fromBinary(UpsertWorkspaceRequestSchema, reqBytes);
      if (!workspace) return;
      upsertWorkspace(this.workspaces, JSON.stringify(protoWorkspaceToCache(workspace)));
      void invoke("blockstoreUpsertWorkspace", Array.from(reqBytes));
    } catch { /* tolerate malformed bytes */ }
  }
  replace_workspaces(reqBytes: Uint8Array): void {
    try {
      const { workspaces } = fromBinary(ReplaceWorkspacesRequestSchema, reqBytes);
      replaceWorkspaces(this.workspaces, JSON.stringify(workspaces.map(protoWorkspaceToCache)));
      void invoke("blockstoreReplaceWorkspaces", Array.from(reqBytes));
    } catch { /* tolerate malformed bytes */ }
  }

  // Sync override: stores call set_last_op_id fire-and-forget. Cache the
  // value immediately so the next sync read sees it; IPC mirrors in background.
  set_last_op_id_sync(workspaceId: string, id: bigint): void {
    void invoke("blockstoreSetLastOpId", workspaceId, Number(id));
    const map = safeJsonMap(this.cache.lastOpIdsJson);
    map[workspaceId] = id.toString();
    this.cache.lastOpIdsJson = JSON.stringify(map);
  }

  // wasm exposes last_op_id as synchronous bigint; downstream stores read
  // without await. Resolve from the renderer cache to keep that contract.
  last_op_id(workspaceId: string): bigint {
    try {
      const map = safeJsonMap(this.cache.lastOpIdsJson);
      const v = map[workspaceId];
      if (typeof v === "number") return BigInt(v);
      if (typeof v === "string" && v) return BigInt(v);
    } catch { /* fall through */ }
    return 0n;
  }

  async set_last_op_id(workspaceId: string, id: bigint | number): Promise<void> {
    // napi-rs's i64 binding refuses incoming BigInt — widen at the IPC
    // boundary; op_id is a counter that fits in Number.
    const idNum = typeof id === "bigint" ? Number(id) : id;
    const map = safeJsonMap(this.cache.lastOpIdsJson);
    map[workspaceId] = idNum;
    this.cache.lastOpIdsJson = JSON.stringify(map);
    await invoke("blockstoreSetLastOpId", workspaceId, idNum);
  }
}
