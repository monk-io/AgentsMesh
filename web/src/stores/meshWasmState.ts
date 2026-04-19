import type {
  MeshTopologyData, MeshNodeData, MeshEdgeData, ChannelInfoData, RunnerInfoData,
} from "@/lib/api/meshTypes";
import { getMeshState } from "@/lib/wasm-core";

export { getMeshState as getMeshWasmState };

let cachedTopology: MeshTopologyData | null = null;
const nodeCache = new Map<string, MeshNodeData>();
const edgeCache = new Map<string, MeshEdgeData[]>();

export function cacheTopology(t: MeshTopologyData) {
  cachedTopology = t;
  nodeCache.clear();
  edgeCache.clear();
  for (const n of t.nodes) nodeCache.set(n.pod_key, n);
  for (const e of t.edges) {
    for (const key of [e.source, e.target]) {
      const arr = edgeCache.get(key) || [];
      arr.push(e);
      edgeCache.set(key, arr);
    }
  }
}

export function topologyToWasmJson(t: MeshTopologyData): string {
  return JSON.stringify({
    nodes: t.nodes.map((n) => ({
      pod_key: n.pod_key, alias: n.alias, status: n.status,
      agent_status: n.agent_status, agent_slug: "", runner_id: n.runner_id,
    })),
    edges: t.edges.map((e) => ({
      source: e.source, target: e.target, binding_status: e.status,
    })),
    channels: t.channels.map((c) => ({
      id: c.id, name: c.name, pod_keys: c.pod_keys,
    })),
    runners: (t.runners || []).map((r) => ({
      id: r.id, name: r.node_id, status: r.status, pod_keys: [] as string[],
    })),
  });
}

export function readTopology(): MeshTopologyData | null {
  return cachedTopology;
}

export function readSelectedNode(): string | null {
  const v = getMeshState().selected_node();
  return v ?? null;
}

export function getNodeByKey(podKey: string): MeshNodeData | undefined {
  return nodeCache.get(podKey);
}

export function getEdgesForNode(podKey: string): MeshEdgeData[] {
  return edgeCache.get(podKey) || [];
}

export function getChannelsForNode(podKey: string): ChannelInfoData[] {
  if (!cachedTopology) return [];
  return cachedTopology.channels.filter((c) => c.pod_keys.includes(podKey));
}

export function getActiveNodes(): MeshNodeData[] {
  if (!cachedTopology) return [];
  return cachedTopology.nodes.filter(
    (n) => n.status === "running" || n.status === "initializing",
  );
}

export function getNodesByRunner(runnerId: number): MeshNodeData[] {
  if (!cachedTopology) return [];
  return cachedTopology.nodes.filter((n) => n.runner_id === runnerId);
}

export function getRunnerInfo(runnerId: number): RunnerInfoData | undefined {
  return cachedTopology?.runners?.find((r) => r.id === runnerId);
}
