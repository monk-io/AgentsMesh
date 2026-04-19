import type { RunnerData } from "@/lib/api";
import { getRunnerState } from "@/lib/wasm-core";

export type WasmRunnerRaw = {
  id: number;
  name: string;
  status: string;
  version?: string;
  max_concurrent_pods: number;
  active_pod_count: number;
  is_enabled: boolean;
  host_info?: Record<string, unknown>;
};

function runnerToWasm(r: RunnerData): object {
  return {
    id: r.id,
    name: r.node_id || "",
    status: r.status,
    version: r.runner_version,
    max_concurrent_pods: r.max_concurrent_pods,
    active_pod_count: r.current_pods,
    is_enabled: r.is_enabled,
    host_info: r.host_info,
    created_at: r.created_at,
    updated_at: r.updated_at,
  };
}

export function runnersToWasmJson(runners: RunnerData[]): string {
  return JSON.stringify(runners.map(runnerToWasm));
}

export function runnerToWasmJson(runner: RunnerData): string {
  return JSON.stringify(runnerToWasm(runner));
}

const runnerCache = new Map<number, RunnerData>();

export function cacheRunners(runners: RunnerData[]) {
  for (const r of runners) runnerCache.set(r.id, r);
}

export function cacheRunner(runner: RunnerData) {
  runnerCache.set(runner.id, runner);
}

export function enrichWasmRunner(wr: WasmRunnerRaw): RunnerData {
  const cached = runnerCache.get(wr.id);
  if (cached) {
    return {
      ...cached,
      status: wr.status as RunnerData["status"],
      current_pods: wr.active_pod_count,
      max_concurrent_pods: wr.max_concurrent_pods,
      is_enabled: wr.is_enabled,
    };
  }
  return {
    id: wr.id, node_id: wr.name, status: wr.status as RunnerData["status"],
    current_pods: wr.active_pod_count, max_concurrent_pods: wr.max_concurrent_pods,
    runner_version: wr.version, is_enabled: wr.is_enabled,
    created_at: "", updated_at: "",
  } as RunnerData;
}

export function readWasmRunners(): RunnerData[] {
  const raw: WasmRunnerRaw[] = JSON.parse(getRunnerState().runners_json());
  return raw.map(enrichWasmRunner);
}

export function readWasmAvailableRunners(): RunnerData[] {
  const raw: WasmRunnerRaw[] = JSON.parse(getRunnerState().available_runners_json());
  return raw.map(enrichWasmRunner);
}

export function readWasmCurrentRunner(): RunnerData | null {
  const json = getRunnerState().current_runner_json();
  if (!json) return null;
  return enrichWasmRunner(JSON.parse(json) as WasmRunnerRaw);
}
