import { invoke } from "./invoke";
import type { IPodService } from "@agentsmesh/service-interface";
import { fromBinary } from "@bufbuild/protobuf";
import {
  ReplaceCachedPodsRequestSchema,
  AppendCachedPodsRequestSchema,
  InsertCreatedPodRequestSchema,
  MarkPodTerminatedRequestSchema,
  PatchPodPerpetualRequestSchema,
  ApplyPodStatusEventRequestSchema,
  ApplyPodTitleEventRequestSchema,
  ApplyPodAliasEventRequestSchema,
  ApplyAgentStatusEventRequestSchema,
} from "@agentsmesh/proto/pod_state/v1/pod_state_pb";
import type { Pod as ProtoPod } from "@agentsmesh/proto/pod/v1/pod_pb";

// ProtoPod (camelCase + BigInt) → PodData (snake_case + number). Same logic as
// fromProtoPod in clients/web/src/lib/api/connect/podConnect.ts but inlined to
// keep electron-adapter independent of the web/src tree.
function podToCache(p: ProtoPod): Record<string, unknown> {
  const ZERO = BigInt(0);
  return {
    id: Number(p.id),
    pod_key: p.podKey,
    status: p.status,
    agent_status: p.agentStatus,
    alias: p.alias,
    title: p.title,
    runner: p.runner ? {
      id: p.runner.id === undefined || p.runner.id === ZERO ? undefined : Number(p.runner.id),
      node_id: p.runner.nodeId,
      status: p.runner.status,
    } : undefined,
    agent: p.agent ? { name: p.agent.name, slug: p.agent.slug } : undefined,
    repository: p.repository ? {
      id: p.repository.id === undefined || p.repository.id === ZERO ? undefined : Number(p.repository.id),
      name: p.repository.name,
      slug: p.repository.slug,
      provider_type: p.repository.providerType,
    } : undefined,
    ticket: p.ticket ? {
      id: p.ticket.id === undefined || p.ticket.id === ZERO ? undefined : Number(p.ticket.id),
      slug: p.ticket.slug,
      title: p.ticket.title,
    } : undefined,
    loop: p.loop ? {
      id: p.loop.id === undefined || p.loop.id === ZERO ? undefined : Number(p.loop.id),
      slug: p.loop.slug,
      name: p.loop.name,
    } : undefined,
    created_by: p.createdBy ? {
      id: p.createdBy.id === undefined || p.createdBy.id === ZERO ? undefined : Number(p.createdBy.id),
      username: p.createdBy.username,
      name: p.createdBy.name,
    } : undefined,
    repository_id: undefined,
    ticket_id: undefined,
    loop_id: undefined,
    organization_id: undefined,
    runner_id: p.runnerId !== undefined && p.runnerId !== ZERO ? Number(p.runnerId) : undefined,
    interaction_mode: p.interactionMode,
    agentfile: undefined,
    branch: p.branchName,
    initial_prompt: p.prompt,
    perpetual: p.perpetual,
    error_code: p.errorCode || undefined,
    error_message: p.errorMessage || undefined,
    created_at: p.createdAt,
    updated_at: p.updatedAt,
  };
}

export class ElectronPodService implements IPodService {
  private _podsCache = "[]";
  private _currentPodCache: string | null = null;

  pods_json(): string { return this._podsCache; }
  current_pod_json(): unknown { return this._currentPodCache; }
  get_pod_json(pod_key: string): unknown {
    const pods = JSON.parse(this._podsCache) as { pod_key: string }[];
    const p = pods.find(x => x.pod_key === pod_key);
    return p ? JSON.stringify(p) : null;
  }

  upsert_pod(json: string): void {
    const pod = JSON.parse(json) as { pod_key: string };
    const pods = JSON.parse(this._podsCache) as { pod_key: string }[];
    const idx = pods.findIndex(x => x.pod_key === pod.pod_key);
    if (idx >= 0) pods[idx] = pod; else pods.push(pod);
    this._podsCache = JSON.stringify(pods);
  }

  set_pods(json: string): void { this._podsCache = json; }
  set_current_pod(json: string): void { this._currentPodCache = json || null; }

  update_pod_status(key: string, status: string, agentStatus?: string | null, errorCode?: string | null, errorMessage?: string | null): void {
    const pods = JSON.parse(this._podsCache) as { pod_key: string; status: string; agent_status?: string; error_code?: string; error_message?: string }[];
    const p = pods.find(x => x.pod_key === key);
    if (p) {
      p.status = status;
      if (agentStatus !== undefined) p.agent_status = agentStatus ?? undefined;
      if (errorCode !== undefined) p.error_code = errorCode ?? undefined;
      if (errorMessage !== undefined) p.error_message = errorMessage ?? undefined;
    }
    this._podsCache = JSON.stringify(pods);
  }

  update_pod_title(key: string, title: string): void {
    const pods = JSON.parse(this._podsCache) as { pod_key: string; title?: string }[];
    const p = pods.find(x => x.pod_key === key);
    if (p) p.title = title;
    this._podsCache = JSON.stringify(pods);
  }

  update_pod_alias(key: string, alias: string): void {
    const pods = JSON.parse(this._podsCache) as { pod_key: string; alias?: string }[];
    const p = pods.find(x => x.pod_key === key);
    if (p) p.alias = alias;
    this._podsCache = JSON.stringify(pods);
  }

  update_agent_status(key: string, status: string): void {
    const pods = JSON.parse(this._podsCache) as { pod_key: string; agent_status?: string }[];
    const p = pods.find(x => x.pod_key === key);
    if (p) p.agent_status = status;
    this._podsCache = JSON.stringify(pods);
  }

  remove_pod(key: string): void {
    const pods = JSON.parse(this._podsCache) as { pod_key: string }[];
    this._podsCache = JSON.stringify(pods.filter(x => x.pod_key !== key));
  }

  async fetch_pods(status?: string | null, runnerId?: bigint | null, createdById?: bigint | null, limit?: bigint | null, offset?: bigint | null): Promise<string> {
    const result = await invoke<string>("podFetchPods", status, runnerId ? Number(runnerId) : null, createdById ? Number(createdById) : null, limit ? Number(limit) : null, offset ? Number(offset) : null);
    const parsed = JSON.parse(result);
    this._podsCache = JSON.stringify(parsed.pods || []);
    return result;
  }

  async fetch_sidebar_pods(filter: string, userId?: bigint | null): Promise<string> {
    const result = await invoke<string>("podFetchSidebarPods", filter, userId ? Number(userId) : null);
    const parsed = JSON.parse(result);
    this._podsCache = JSON.stringify(parsed.pods || []);
    return result;
  }

  async load_more_pods(filter: string, userId: bigint | null | undefined, offset: bigint): Promise<string> {
    const result = await invoke<string>("podLoadMorePods", filter, userId ? Number(userId) : null, Number(offset));
    const parsed = JSON.parse(result);
    for (const pod of (parsed.newPods || [])) this.upsert_pod(JSON.stringify(pod));
    return result;
  }

  async fetch_pod(key: string): Promise<string> {
    const result = await invoke<string>("podFetchPod", key);
    this.upsert_pod(result);
    this._currentPodCache = result;
    return result;
  }

  async create_pod(json: string): Promise<string> {
    const result = await invoke<string>("podCreatePod", json);
    this.upsert_pod(result);
    this._currentPodCache = result;
    return result;
  }

  async terminate_pod(key: string): Promise<void> {
    await invoke<void>("podTerminatePod", key);
    this.update_pod_status(key, "terminated");
  }

  async update_pod_alias_api(key: string, alias?: string | null): Promise<void> {
    await invoke<void>("podUpdatePodAlias", key, alias);
    this.update_pod_alias(key, alias || "");
  }

  async get_pod_connection(key: string): Promise<string> {
    return invoke<string>("podGetPodConnection", key);
  }

  // Proto-bytes mutators (mirror clients/core/crates/wasm WasmPodState). The
  // wasm side stores in linear memory which the renderer reads synchronously;
  // the Electron side has no Rust memory in-process — decode the proto bytes
  // locally and update _podsCache so the renderer's `pods_json()` reflects the
  // mutation immediately. NAPI forwarding is fire-and-forget so a future
  // main-process consumer of the Rust cache stays in sync.
  replace_cached_pods(reqBytes: Uint8Array): void {
    const req = fromBinary(ReplaceCachedPodsRequestSchema, reqBytes);
    this._podsCache = JSON.stringify(req.pods.map(podToCache));
  }

  append_cached_pods(reqBytes: Uint8Array): void {
    const req = fromBinary(AppendCachedPodsRequestSchema, reqBytes);
    const existing = JSON.parse(this._podsCache) as { pod_key: string }[];
    const seen = new Set(existing.map((p) => p.pod_key));
    for (const p of req.pods) {
      const c = podToCache(p);
      if (!seen.has(c.pod_key as string)) existing.push(c as { pod_key: string });
    }
    this._podsCache = JSON.stringify(existing);
  }

  insert_created_pod(reqBytes: Uint8Array): void {
    const req = fromBinary(InsertCreatedPodRequestSchema, reqBytes);
    if (req.pod) {
      const cache = JSON.parse(this._podsCache) as { pod_key: string }[];
      const c = podToCache(req.pod);
      const idx = cache.findIndex((p) => p.pod_key === c.pod_key);
      if (idx >= 0) cache[idx] = { ...cache[idx], ...c };
      else cache.unshift(c as { pod_key: string });
      this._podsCache = JSON.stringify(cache);
    }
  }

  mark_pod_terminated(reqBytes: Uint8Array): void {
    const req = fromBinary(MarkPodTerminatedRequestSchema, reqBytes);
    this.update_pod_status(req.podKey, "terminated");
  }

  patch_pod_perpetual(reqBytes: Uint8Array): void {
    const req = fromBinary(PatchPodPerpetualRequestSchema, reqBytes);
    const pods = JSON.parse(this._podsCache) as { pod_key: string; perpetual?: boolean }[];
    const p = pods.find((x) => x.pod_key === req.podKey);
    if (p) {
      p.perpetual = req.perpetual;
      this._podsCache = JSON.stringify(pods);
    }
  }

  apply_pod_status_event(reqBytes: Uint8Array): void {
    const req = fromBinary(ApplyPodStatusEventRequestSchema, reqBytes);
    this.update_pod_status(
      req.podKey, req.status,
      req.agentStatus ?? null, req.errorCode ?? null, req.errorMessage ?? null,
    );
  }

  apply_pod_title_event(reqBytes: Uint8Array): void {
    const req = fromBinary(ApplyPodTitleEventRequestSchema, reqBytes);
    this.update_pod_title(req.podKey, req.title);
  }

  apply_pod_alias_event(reqBytes: Uint8Array): void {
    const req = fromBinary(ApplyPodAliasEventRequestSchema, reqBytes);
    this.update_pod_alias(req.podKey, req.alias ?? "");
  }

  apply_agent_status_event(reqBytes: Uint8Array): void {
    const req = fromBinary(ApplyAgentStatusEventRequestSchema, reqBytes);
    this.update_agent_status(req.podKey, req.agentStatus);
  }
}
