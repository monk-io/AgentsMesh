import { create } from "zustand";
import { useMemo } from "react";
import type { LoopData, LoopRunData, RunStatus, CreateLoopRequest, UpdateLoopRequest } from "@/lib/api/loopTypes";
import { getLoopService } from "@/lib/wasm-core";
import { reconnectRegistry } from "@/lib/realtime";
import { getErrorMessage } from "@/lib/utils";

export type { LoopData, LoopRunData, RunStatus };

const svc = () => getLoopService();
const bump = () => useLoopStore.setState((s) => ({ _tick: s._tick + 1 }));

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
      const raw = await svc().fetch_loops(filters?.status ?? undefined, 500, undefined);
      const res: { total: number } = JSON.parse(raw);
      set({ totalCount: res.total, loading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), loading: false }); }
  },

  fetchLoop: async (slug) => {
    const curJson = svc().current_loop_json();
    const curSlug = curJson ? (typeof curJson === "string" ? JSON.parse(curJson) : curJson)?.slug : null;
    if (curSlug !== slug) {
      svc().clear_runs();
      set({ runsTotalCount: 0, _tick: get()._tick + 1 });
    }
    set({ loopLoading: true, error: null });
    try {
      await svc().fetch_loop(slug);
      set({ loopLoading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), loopLoading: false }); }
  },

  createLoop: async (data) => {
    const raw = await svc().create_loop(JSON.stringify(data));
    const loop: LoopData = JSON.parse(raw);
    get().fetchLoops();
    return { loop };
  },

  updateLoop: async (slug, data) => {
    const raw = await svc().update_loop(slug, JSON.stringify(data));
    const loop: LoopData = JSON.parse(raw);
    bump();
    get().fetchLoops();
    return loop;
  },

  deleteLoop: async (slug) => {
    await svc().delete_loop(slug);
    bump();
    get().fetchLoops();
  },

  enableLoop: async (slug) => { await svc().enable_loop(slug); bump(); },
  disableLoop: async (slug) => { await svc().disable_loop(slug); bump(); },

  triggerLoop: async (slug) => {
    try {
      const raw = await svc().trigger_loop(slug);
      const run: LoopRunData = JSON.parse(raw);
      bump();
      get().fetchRuns(slug, { limit: 20, offset: 0 });
      get().fetchLoop(slug);
      return { run };
    } catch { return { skipped: true, reason: "trigger skipped or failed" }; }
  },

  fetchRuns: async (slug, filters) => {
    set({ runsLoading: true });
    try {
      const raw = await svc().fetch_runs(slug, filters?.status ?? undefined, filters?.limit ?? undefined, filters?.offset ?? undefined);
      const res: { total: number } = JSON.parse(raw);
      set({ runsTotalCount: res.total, runsLoading: false, _tick: get()._tick + 1 });
    } catch (err) { set({ error: getErrorMessage(err, "An error occurred"), runsLoading: false }); }
  },

  loadMoreRuns: async (slug) => {
    if (get().runsLoading) return;
    const runs: LoopRunData[] = JSON.parse(svc().runs_json());
    await get().fetchRuns(slug, { limit: 20, offset: runs.length });
  },

  cancelRun: async (slug, runId) => {
    await svc().cancel_run(slug, BigInt(runId));
    bump();
    get().fetchRuns(slug, { limit: 20, offset: 0 });
    get().fetchLoop(slug);
  },

  setCurrentLoop: (loop) => {
    svc().set_current_loop(loop ? JSON.stringify(loop) : "");
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
