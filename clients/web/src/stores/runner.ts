import { create } from "zustand";
import { useMemo } from "react";
import { create as protoCreate, toBinary } from "@bufbuild/protobuf";
import type { RunnerData } from "@/lib/api";
import { reconnectRegistry } from "@/lib/realtime";
import { getErrorMessage } from "@/lib/utils";
import { getRunnerService } from "@/lib/wasm-core";
import { readCurrentOrg } from "@/stores/auth";
import {
  listRunners as listRunnersConnect,
  listAvailableRunners as listAvailableRunnersConnect,
  getRunner as getRunnerConnect,
  updateRunner as updateRunnerConnect,
  deleteRunner as deleteRunnerConnect,
  createRunnerToken as createRunnerTokenConnect,
} from "@/lib/api/facade/runnerConnect";
import {
  ApplyRunnerStatusEventRequestSchema,
  ReplaceCachedRunnersRequestSchema,
  ReplaceAvailableRunnersRequestSchema,
  SetCurrentRunnerRequestSchema,
  PatchCachedRunnerRequestSchema,
  RemoveCachedRunnerRequestSchema,
} from "@proto/runner_state/v1/runner_state_pb";
import { runnerToProtoRunner } from "@/lib/api/runnerProtoMap";

export type RunnerStatus = "online" | "offline" | "maintenance" | "busy";
export type Runner = RunnerData;

interface RunnerState {
  _tick: number; loading: boolean; fetched: boolean; error: string | null;
  fetchRunners: (status?: RunnerStatus) => Promise<void>;
  fetchAvailableRunners: () => Promise<void>;
  fetchRunner: (id: number) => Promise<void>;
  updateRunner: (id: number, data: { description?: string; max_concurrent_pods?: number; is_enabled?: boolean; tags?: string[] }) => Promise<Runner>;
  deleteRunner: (id: number) => Promise<void>;
  createToken: (data?: { name?: string; labels?: string[]; max_uses?: number; expires_in_days?: number }) => Promise<string>;
  setCurrentRunner: (runner: Runner | null) => void;
  updateRunnerStatus: (runnerId: number, status: RunnerStatus) => void;
  clearError: () => void;
}

const svc = () => getRunnerService();
const bump = () => useRunnerStore.setState((s) => ({ _tick: s._tick + 1 }));

function orgSlug(): string {
  return readCurrentOrg()?.slug ?? "";
}

function dispatchReplaceCachedRunners(items: Runner[]) {
  const req = protoCreate(ReplaceCachedRunnersRequestSchema, {
    runners: items.map(runnerToProtoRunner),
  });
  svc().replace_cached_runners(toBinary(ReplaceCachedRunnersRequestSchema, req));
}

function dispatchReplaceAvailableRunners(items: Runner[]) {
  const req = protoCreate(ReplaceAvailableRunnersRequestSchema, {
    runners: items.map(runnerToProtoRunner),
  });
  svc().replace_available_runners(toBinary(ReplaceAvailableRunnersRequestSchema, req));
}

function dispatchSetCurrentRunner(runner: Runner | null) {
  const req = protoCreate(SetCurrentRunnerRequestSchema, {
    runner: runner ? runnerToProtoRunner(runner) : undefined,
  });
  svc().set_current_runner_proto(toBinary(SetCurrentRunnerRequestSchema, req));
}

function dispatchPatchCachedRunner(runner: Runner) {
  const req = protoCreate(PatchCachedRunnerRequestSchema, {
    runner: runnerToProtoRunner(runner),
  });
  svc().patch_cached_runner(toBinary(PatchCachedRunnerRequestSchema, req));
}

function dispatchRemoveCachedRunner(id: number) {
  const req = protoCreate(RemoveCachedRunnerRequestSchema, {
    runnerId: BigInt(id),
  });
  svc().remove_cached_runner(toBinary(RemoveCachedRunnerRequestSchema, req));
}

export function useRunners(): Runner[] {
  const tick = useRunnerStore((s) => s._tick);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  return useMemo(() => JSON.parse(svc().runners_json()), [tick]);
}

export function useAvailableRunners(): Runner[] {
  const tick = useRunnerStore((s) => s._tick);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  return useMemo(() => JSON.parse(svc().available_runners_json()), [tick]);
}

export function useCurrentRunner(): Runner | null {
  const tick = useRunnerStore((s) => s._tick);
  return useMemo(() => {
    const raw = svc().current_runner_json();
    return raw ? (typeof raw === "string" ? JSON.parse(raw) : raw) : null;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [tick]);
}

export function applyRunnerStatusEvent(runnerId: number, status: string) {
  const req = protoCreate(ApplyRunnerStatusEventRequestSchema, {
    runnerId: BigInt(runnerId), status,
  });
  svc().apply_runner_status_event(toBinary(ApplyRunnerStatusEventRequestSchema, req));
}

export const useRunnerStore = create<RunnerState>((set, get) => ({
  _tick: 0, loading: false, fetched: false, error: null,

  fetchRunners: async (status) => {
    set({ loading: true, error: null });
    try {
      const { items } = await listRunnersConnect(orgSlug(), { status });
      dispatchReplaceCachedRunners(items);
      set({ loading: false, fetched: true, _tick: get()._tick + 1 });
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to fetch runners"), loading: false }); }
  },

  fetchAvailableRunners: async () => {
    try {
      const { items } = await listAvailableRunnersConnect(orgSlug());
      dispatchReplaceAvailableRunners(items);
      bump();
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to fetch available runners") }); }
  },

  fetchRunner: async (id) => {
    try {
      const { runner } = await getRunnerConnect(orgSlug(), id);
      if (runner) dispatchSetCurrentRunner(runner);
      bump();
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to fetch runner") }); }
  },

  updateRunner: async (id, data) => {
    try {
      const runner = await updateRunnerConnect(orgSlug(), id, data);
      dispatchPatchCachedRunner(runner);
      bump();
      return runner;
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to update runner") }); throw e; }
  },

  deleteRunner: async (id) => {
    try {
      await deleteRunnerConnect(orgSlug(), id);
      dispatchRemoveCachedRunner(id);
      bump();
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to delete runner") }); throw e; }
  },

  createToken: async (data) => {
    try {
      const resp = await createRunnerTokenConnect(orgSlug(), data ?? {});
      return resp.token ?? "";
    } catch (e: unknown) { set({ error: getErrorMessage(e, "Failed to create token") }); throw e; }
  },

  setCurrentRunner: (runner) => {
    dispatchSetCurrentRunner(runner);
    bump();
  },

  updateRunnerStatus: (runnerId, status) => {
    applyRunnerStatusEvent(runnerId, status);
    bump();
  },

  clearError: () => set({ error: null }),
}));

export const getRunnerStatusInfo = (status: RunnerStatus) => {
  const m: Record<RunnerStatus, { label: string; color: string; dotColor: string }> = {
    online: { label: "Online", color: "text-green-600 dark:text-green-400", dotColor: "bg-green-500" },
    offline: { label: "Offline", color: "text-gray-500 dark:text-gray-400", dotColor: "bg-gray-400" },
    maintenance: { label: "Maintenance", color: "text-yellow-600 dark:text-yellow-400", dotColor: "bg-yellow-500" },
    busy: { label: "Busy", color: "text-orange-600 dark:text-orange-400", dotColor: "bg-orange-500" },
  };
  return m[status];
};

export const canAcceptPods = (runner: Runner): boolean =>
  runner.status === "online" && runner.current_pods < runner.max_concurrent_pods;

export const formatHostInfo = (hostInfo?: Runner["host_info"]) => {
  if (!hostInfo) return "Unknown";
  const parts: string[] = [];
  if (hostInfo.os) parts.push(hostInfo.os);
  if (hostInfo.arch) parts.push(hostInfo.arch);
  if (hostInfo.cpu_cores) parts.push(`${hostInfo.cpu_cores} cores`);
  if (hostInfo.memory) parts.push(`${(hostInfo.memory / 1024 / 1024 / 1024).toFixed(1)}GB RAM`);
  return parts.length > 0 ? parts.join(" / ") : "Unknown";
};

reconnectRegistry.register({
  name: "runner:list",
  fn: () => useRunnerStore.getState().fetchRunners?.(),
  priority: "immediate",
});
