import { invoke } from "./invoke";
import type { ILoopService } from "@agentsmesh/service-interface";
import { fromBinary } from "@bufbuild/protobuf";
import {
  ReplaceCachedLoopsRequestSchema,
  SetCurrentLoopRequestSchema,
  ClearCurrentLoopRequestSchema,
  PatchLoopFromActionRequestSchema,
  InsertLoopRunRequestSchema,
  ReplaceCachedRunsRequestSchema,
  AppendCachedRunsRequestSchema,
  PatchLoopRunStatusRequestSchema,
  ClearLoopRunsRequestSchema,
} from "@agentsmesh/proto/loop_state/v1/loop_state_pb";

// Proto -> JS-cache shape converter. Mirrors the legacy JSON shape readers
// (loops_json / current_loop_json / runs_json) consumed by selectors.
interface ProtoLoop {
  id: bigint; slug: string; name: string; description?: string;
  podKey?: string; agentSlug?: string; intervalSeconds?: number;
  prompt?: string; status?: string; enabled?: boolean;
  lastRunAt?: string; nextRunAt?: string;
  createdAt?: string; updatedAt?: string;
}
interface ProtoLoopRun {
  id: bigint; loopSlug?: string; podKey?: string; status?: string;
  startedAt?: string; finishedAt?: string;
}

function loopToCache(l: ProtoLoop): Record<string, unknown> {
  const out: Record<string, unknown> = {
    id: Number(l.id), slug: l.slug, name: l.name,
  };
  if (l.description !== undefined) out.description = l.description;
  if (l.podKey !== undefined) out.pod_key = l.podKey;
  if (l.agentSlug !== undefined) out.agent_slug = l.agentSlug;
  if (l.intervalSeconds !== undefined) out.interval_seconds = l.intervalSeconds;
  if (l.prompt !== undefined) out.prompt = l.prompt;
  if (l.status !== undefined) out.status = l.status;
  if (l.enabled !== undefined) out.enabled = l.enabled;
  if (l.lastRunAt !== undefined) out.last_run_at = l.lastRunAt;
  if (l.nextRunAt !== undefined) out.next_run_at = l.nextRunAt;
  if (l.createdAt !== undefined) out.created_at = l.createdAt;
  if (l.updatedAt !== undefined) out.updated_at = l.updatedAt;
  return out;
}

function runToCache(r: ProtoLoopRun): Record<string, unknown> {
  const out: Record<string, unknown> = { id: Number(r.id) };
  if (r.loopSlug !== undefined) out.loop_slug = r.loopSlug;
  if (r.podKey !== undefined) out.pod_key = r.podKey;
  if (r.status !== undefined) out.status = r.status;
  if (r.startedAt !== undefined) out.started_at = r.startedAt;
  if (r.finishedAt !== undefined) out.finished_at = r.finishedAt;
  return out;
}

export class ElectronLoopService implements ILoopService {
  private _loopsCache = "[]";
  private _runsCache = "[]";
  private _currentLoopCache: string | null = null;

  loops_json(): string { return this._loopsCache; }
  runs_json(): string { return this._runsCache; }
  current_loop_json(): unknown { return this._currentLoopCache; }

  get_loop_by_slug_json(slug: string): unknown {
    const loops = JSON.parse(this._loopsCache) as { slug: string }[];
    const l = loops.find(x => x.slug === slug);
    return l ? JSON.stringify(l) : null;
  }

  // Proto-bytes mutators (mirror WasmLoopService). Decode locally to keep
  // synchronous read selectors warm, then fan out to NAPI fire-and-forget so
  // the main-process Rust state stays in sync.

  replace_cached_loops(reqBytes: Uint8Array): void {
    const req = fromBinary(ReplaceCachedLoopsRequestSchema, reqBytes);
    this._loopsCache = JSON.stringify(req.loops.map(loopToCache));
    void invoke<void>("loopSvcReplaceCachedLoops", Array.from(reqBytes)).catch(() => undefined);
  }

  set_current_loop(reqBytes: Uint8Array): void {
    const req = fromBinary(SetCurrentLoopRequestSchema, reqBytes);
    this._currentLoopCache = req.loop ? JSON.stringify(loopToCache(req.loop)) : null;
    void invoke<void>("loopSvcSetCurrentLoop", Array.from(reqBytes)).catch(() => undefined);
  }

  clear_current_loop(reqBytes: Uint8Array): void {
    fromBinary(ClearCurrentLoopRequestSchema, reqBytes);
    this._currentLoopCache = null;
    void invoke<void>("loopSvcClearCurrentLoop", Array.from(reqBytes)).catch(() => undefined);
  }

  patch_loop_from_action(reqBytes: Uint8Array): void {
    const req = fromBinary(PatchLoopFromActionRequestSchema, reqBytes);
    if (req.loop) {
      const patch = loopToCache(req.loop);
      const list = JSON.parse(this._loopsCache) as Array<{ slug: string }>;
      const idx = list.findIndex(x => x.slug === req.slug);
      if (idx >= 0) list[idx] = { ...list[idx], ...patch } as { slug: string };
      this._loopsCache = JSON.stringify(list);
    }
    void invoke<void>("loopSvcPatchLoopFromAction", Array.from(reqBytes)).catch(() => undefined);
  }

  insert_loop_run(reqBytes: Uint8Array): void {
    const req = fromBinary(InsertLoopRunRequestSchema, reqBytes);
    if (req.run) {
      const runs = JSON.parse(this._runsCache) as unknown[];
      runs.push(runToCache(req.run));
      this._runsCache = JSON.stringify(runs);
    }
    void invoke<void>("loopSvcInsertLoopRun", Array.from(reqBytes)).catch(() => undefined);
  }

  replace_cached_runs(reqBytes: Uint8Array): void {
    const req = fromBinary(ReplaceCachedRunsRequestSchema, reqBytes);
    this._runsCache = JSON.stringify(req.runs.map(runToCache));
    void invoke<void>("loopSvcReplaceCachedRuns", Array.from(reqBytes)).catch(() => undefined);
  }

  append_cached_runs(reqBytes: Uint8Array): void {
    const req = fromBinary(AppendCachedRunsRequestSchema, reqBytes);
    const existing = JSON.parse(this._runsCache) as unknown[];
    const newer = req.runs.map(runToCache);
    this._runsCache = JSON.stringify([...existing, ...newer]);
    void invoke<void>("loopSvcAppendCachedRuns", Array.from(reqBytes)).catch(() => undefined);
  }

  patch_loop_run_status(reqBytes: Uint8Array): void {
    const req = fromBinary(PatchLoopRunStatusRequestSchema, reqBytes);
    const runs = JSON.parse(this._runsCache) as Array<{ id: number; status?: string }>;
    const r = runs.find(x => x.id === Number(req.runId));
    if (r) r.status = req.status;
    this._runsCache = JSON.stringify(runs);
    void invoke<void>("loopSvcPatchLoopRunStatus", Array.from(reqBytes)).catch(() => undefined);
  }

  clear_loop_runs(reqBytes: Uint8Array): void {
    fromBinary(ClearLoopRunsRequestSchema, reqBytes);
    this._runsCache = "[]";
    void invoke<void>("loopSvcClearLoopRuns", Array.from(reqBytes)).catch(() => undefined);
  }
}
