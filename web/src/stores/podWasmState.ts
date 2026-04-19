import type { PodData } from "@/lib/api";
import { getPodState } from "@/lib/wasm-core";

export type WasmPodRaw = {
  key: string;
  status: string;
  agent_status?: string;
  title?: string;
  alias?: string;
  error_code?: string;
  error_message?: string;
};

export function podToWasmJson(pod: PodData): string {
  return JSON.stringify({
    key: pod.pod_key,
    status: pod.status,
    agent_status: pod.agent_status || undefined,
    agent_slug: pod.agent?.slug || "",
    runner_id: pod.runner?.id,
    title: pod.title,
    alias: pod.alias,
    user_id: pod.created_by?.id,
    ticket_slug: pod.ticket?.slug,
    error_code: pod.error_code,
    error_message: pod.error_message,
    created_at: pod.created_at,
  });
}

export function podsToWasmJson(pods: PodData[]): string {
  return JSON.stringify(pods.map((p) => ({
    key: p.pod_key,
    status: p.status,
    agent_status: p.agent_status || undefined,
    agent_slug: p.agent?.slug || "",
    runner_id: p.runner?.id,
    title: p.title,
    alias: p.alias,
    user_id: p.created_by?.id,
    ticket_slug: p.ticket?.slug,
    error_code: p.error_code,
    error_message: p.error_message,
    created_at: p.created_at,
  })));
}

const podCache = new Map<string, PodData>();

export function cachePods(pods: PodData[]) {
  for (const p of pods) podCache.set(p.pod_key, p);
}

export function cachePod(pod: PodData) {
  podCache.set(pod.pod_key, pod);
}

export function enrichWasmPod(wp: WasmPodRaw): PodData {
  const cached = podCache.get(wp.key);
  if (cached) {
    return {
      ...cached,
      status: wp.status as PodData["status"],
      agent_status: wp.agent_status ?? cached.agent_status,
      title: wp.title,
      alias: wp.alias,
      error_code: wp.error_code,
      error_message: wp.error_message,
    };
  }
  return {
    id: 0, pod_key: wp.key, status: wp.status as PodData["status"],
    agent_status: wp.agent_status || "", created_at: new Date().toISOString(),
  } as PodData;
}

export function readWasmPods(): PodData[] {
  const raw: WasmPodRaw[] = JSON.parse(getPodState().pods_json());
  return raw.map(enrichWasmPod);
}

export function readWasmCurrentPod(): PodData | null {
  const json = getPodState().current_pod_json();
  if (!json) return null;
  return enrichWasmPod(JSON.parse(json) as WasmPodRaw);
}
