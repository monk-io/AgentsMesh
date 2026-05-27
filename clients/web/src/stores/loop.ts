import { create } from "zustand";
import { useMemo } from "react";
import { create as protoCreate, toBinary } from "@bufbuild/protobuf";
import type { LoopData, LoopRunData, RunStatus, CreateLoopRequest, UpdateLoopRequest } from "@/lib/viewModels/loop";
import { getLoopService } from "@/lib/wasm-core";
import { readCurrentOrg } from "@/stores/auth";
import { reconnectRegistry } from "@/lib/realtime";
import { getErrorMessage } from "@/lib/utils";
import {
  listLoops as listLoopsConnect,
  getLoop as getLoopConnect,
  createLoop as createLoopConnect,
  updateLoop as updateLoopConnect,
  deleteLoop as deleteLoopConnect,
  enableLoop as enableLoopConnect,
  disableLoop as disableLoopConnect,
  triggerLoop as triggerLoopConnect,
  listLoopRuns as listLoopRunsConnect,
  cancelLoopRun as cancelLoopRunConnect,
} from "@/lib/api/facade/loopConnect";
import {
  AppendCachedRunsRequestSchema, ClearCurrentLoopRequestSchema,
  ClearLoopRunsRequestSchema, InsertLoopRunRequestSchema,
  PatchLoopFromActionRequestSchema, PatchLoopRunStatusRequestSchema,
  ReplaceCachedLoopsRequestSchema, ReplaceCachedRunsRequestSchema,
  SetCurrentLoopRequestSchema,
} from "@proto/loop_state/v1/loop_state_pb";
import { loopToProtoLoop, loopRunToProtoLoopRun } from "@/lib/api/loopProtoMap";

export type { LoopData, LoopRunData, RunStatus };

const svc = () => getLoopService();
const bump = () => useLoopStore.setState((s) => ({ _tick: s._tick + 1 }));

function orgSlug(): string {
  return readCurrentOrg()?.slug ?? "";
}

export function useLoops(): LoopData[] {
  const tick = useLoopStore((s) => s._tick);
  return useMemo(() => JSON.parse(svc().loops_json()), [tick]);
}

export function useCurrentLoop(): LoopData | null {
  const tick = useLoopStore((s) => s._tick);
  return useMemo(() => {
    const v = svc().current_loop_json();
    return v ? (typeof v === "string" ? JSON.parse(v) : v) : null;
  }, [tick]);
}

export function useLoopRuns(): LoopRunData[] {
  const tick = useLoopStore((s) => s._tick);
  return useMemo(() => JSON.parse(svc().runs_json()), [tick]);
}

function replaceCachedLoops(items: LoopData[]): void {
  const req = protoCreate(ReplaceCachedLoopsRequestSchema, {
    loops: items.map(loopToProtoLoop),
  });
  svc().replace_cached_loops(toBinary(ReplaceCachedLoopsRequestSchema, req));
}

function setCurrentLoop(loop: LoopData): void {
  const req = protoCreate(SetCurrentLoopRequestSchema, { loop: loopToProtoLoop(loop) });
  svc().set_current_loop(toBinary(SetCurrentLoopRequestSchema, req));
}

function clearCurrentLoop(): void {
  const req = protoCreate(ClearCurrentLoopRequestSchema, {});
  svc().clear_current_loop(toBinary(ClearCurrentLoopRequestSchema, req));
}

function patchLoopFromAction(slug: string, loop: LoopData): void {
  const req = protoCreate(PatchLoopFromActionRequestSchema, {
    slug, loop: loopToProtoLoop(loop),
  });
  svc().patch_loop_from_action(toBinary(PatchLoopFromActionRequestSchema, req));
}

function insertLoopRun(run: LoopRunData): void {
  const req = protoCreate(InsertLoopRunRequestSchema, { run: loopRunToProtoLoopRun(run) });
  svc().insert_loop_run(toBinary(InsertLoopRunRequestSchema, req));
}

function replaceCachedRuns(items: LoopRunData[]): void {
  const req = protoCreate(ReplaceCachedRunsRequestSchema, {
    runs: items.map(loopRunToProtoLoopRun),
  });
  svc().replace_cached_runs(toBinary(ReplaceCachedRunsRequestSchema, req));
}

function appendCachedRuns(items: LoopRunData[]): void {
  const req = protoCreate(AppendCachedRunsRequestSchema, {
    runs: items.map(loopRunToProtoLoopRun),
  });
  svc().append_cached_runs(toBinary(AppendCachedRunsRequestSchema, req));
}

function patchLoopRunStatus(runId: number, status: string): void {
  const req = protoCreate(PatchLoopRunStatusRequestSchema, {
    runId: BigInt(runId), status,
  });
  svc().patch_loop_run_status(toBinary(PatchLoopRunStatusRequestSchema, req));
}

function clearLoopRuns(): void {
  const req = protoCreate(ClearLoopRunsRequestSchema, {});
  svc().clear_loop_runs(toBinary(ClearLoopRunsRequestSchema, req));
}

interface LoopStoreState {
  _tick: number;
  loading: boolean; loopLoading: boolean; runsLoading: boolean;
  error: string | null; totalCount: number; runsTotalCount: number;
  fetchLoops: (filters?: { query?: string; status?: string }) => Promise<void>;
  fetchLoop: (slug: string) => Promise<void>;
  createLoop: (data: CreateLoopRequest) => Promise<{ loop: LoopData }>;
  updateLoop: (slug: string, data: UpdateLoopRequest) => Promise<LoopData>;
  deleteLoop: (slug: string) => Promise<void>;
  enableLoop: (slug: string) => Promise<void>;
  disableLoop: (slug: string) => Promise<void>;
  triggerLoop: (slug: string) => Promise<{ run?: LoopRunData; skipped?: boolean; reason?: string }>;
  fetchRuns: (slug: string, filters?: { status?: string; limit?: number; offset?: number }) => Promise<void>;
  loadMoreRuns: (slug: string) => Promise<void>;
  cancelRun: (slug: string, runId: number) => Promise<void>;
  setCurrentLoop: (loop: LoopData | null) => void;
  getLoopBySlug: (slug: string) => LoopData | undefined;
  clearError: () => void;
}

export const useLoopStore = create<LoopStoreState>((set, get) => ({
  _tick: 0,
  loading: false, loopLoading: false, runsLoading: false,
  error: null, totalCount: 0, runsTotalCount: 0,

  fetchLoops: async (filters) => {
    set({ loading: true, error: null });
    try {
      const { items, total } = await listLoopsConnect(orgSlug(), {
        status: filters?.status,
        query: filters?.query,
        limit: 500,
      });
      replaceCachedLoops(items);
      set({ totalCount: total, loading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), loading: false }); }
  },

  fetchLoop: async (slug) => {
    const curJson = svc().current_loop_json();
    const curSlug = curJson ? (typeof curJson === "string" ? JSON.parse(curJson) : curJson)?.slug : null;
    if (curSlug !== slug) {
      clearLoopRuns();
      set({ runsTotalCount: 0, _tick: get()._tick + 1 });
    }
    set({ loopLoading: true, error: null });
    try {
      const loop = await getLoopConnect(orgSlug(), slug);
      setCurrentLoop(loop);
      set({ loopLoading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), loopLoading: false }); }
  },

  createLoop: async (data) => {
    const loop = await createLoopConnect(orgSlug(), data);
    get().fetchLoops();
    return { loop };
  },

  updateLoop: async (slug, data) => {
    const loop = await updateLoopConnect(orgSlug(), slug, data);
    bump();
    get().fetchLoops();
    return loop;
  },

  deleteLoop: async (slug) => {
    await deleteLoopConnect(orgSlug(), slug);
    clearCurrentLoop();
    bump();
    get().fetchLoops();
  },

  enableLoop: async (slug) => {
    const loop = await enableLoopConnect(orgSlug(), slug);
    patchLoopFromAction(slug, loop);
    bump();
  },
  disableLoop: async (slug) => {
    const loop = await disableLoopConnect(orgSlug(), slug);
    patchLoopFromAction(slug, loop);
    bump();
  },

  triggerLoop: async (slug) => {
    try {
      const result = await triggerLoopConnect(orgSlug(), slug);
      if (result.run) {
        insertLoopRun(result.run);
      }
      bump();
      get().fetchRuns(slug, { limit: 20, offset: 0 });
      get().fetchLoop(slug);
      return result;
    } catch { return { skipped: true, reason: "trigger skipped or failed" }; }
  },

  fetchRuns: async (slug, filters) => {
    set({ runsLoading: true });
    try {
      const { items, total } = await listLoopRunsConnect(orgSlug(), slug, {
        status: filters?.status,
        limit: filters?.limit,
        offset: filters?.offset,
      });
      if ((filters?.offset ?? 0) > 0) {
        appendCachedRuns(items);
      } else {
        replaceCachedRuns(items);
      }
      set({ runsTotalCount: total, runsLoading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), runsLoading: false }); }
  },

  loadMoreRuns: async (slug) => {
    if (get().runsLoading) return;
    const runs: LoopRunData[] = JSON.parse(svc().runs_json());
    await get().fetchRuns(slug, { limit: 20, offset: runs.length });
  },

  cancelRun: async (slug, runId) => {
    await cancelLoopRunConnect(orgSlug(), slug, runId);
    patchLoopRunStatus(runId, "cancelled");
    bump();
    get().fetchRuns(slug, { limit: 20, offset: 0 });
    get().fetchLoop(slug);
  },

  setCurrentLoop: (loop) => {
    if (loop) setCurrentLoop(loop);
    else clearCurrentLoop();
    bump();
  },

  getLoopBySlug: (slug) => {
    const val = svc().get_loop_by_slug_json(slug);
    return val ? (typeof val === "string" ? JSON.parse(val) : val) : undefined;
  },

  clearError: () => set({ error: null }),
}));

reconnectRegistry.register({
  name: "loop:list",
  fn: () => useLoopStore.getState().fetchLoops?.(),
  priority: "low",
});
