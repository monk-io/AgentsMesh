import { usePodStore } from "@/stores/pod";
import { getPodState } from "@/lib/wasm-core";
import { useWorkspaceStore } from "@/stores/workspace";
import { useMeshStore } from "@/stores/mesh";
import type { DebounceRef } from "./realtimeEventHandlers";
import {
  type RealtimeEvent,
  decodeEventData,
  PodStatusChangedEventDataSchema,
  PodCreatedEventDataSchema,
  PodInitProgressEventDataSchema,
  PodRestartingEventDataSchema,
} from "@/lib/realtime";

function debouncedSidebarRefresh(ref?: DebounceRef) {
  if (!ref) {
    const { currentSidebarFilter, fetchSidebarPods } = usePodStore.getState();
    fetchSidebarPods?.(currentSidebarFilter);
    return;
  }
  if (ref.current) clearTimeout(ref.current);
  ref.current = setTimeout(() => {
    ref.current = null;
    const { currentSidebarFilter, fetchSidebarPods } = usePodStore.getState();
    fetchSidebarPods?.(currentSidebarFilter);
  }, 500);
}

let topologyTimer: ReturnType<typeof setTimeout> | null = null;
function debouncedFetchTopology() {
  if (topologyTimer) clearTimeout(topologyTimer);
  topologyTimer = setTimeout(() => {
    topologyTimer = null;
    useMeshStore.getState().fetchTopology?.();
  }, 500);
}

// Rust event_dispatch owns the pod state mutation (status/agent/title/alias/
// perpetual) in runtime.state; this bump triggers the React selectors to
// re-read it. On web getPodState() IS runtime.state; on desktop the main→
// renderer snapshot mirror (realtime-mirror.ts) has already upserted the
// renderer cache and bumped — this extra bump is a harmless no-op there.
const bumpPods = () => usePodStore.setState((s) => ({ _tick: s._tick + 1 }));

export function handlePodEvent(event: RealtimeEvent, sidebarDebounceRef?: DebounceRef) {
  switch (event.type) {
    case "pod:created": {
      const data = decodeEventData(PodCreatedEventDataSchema, event.data);
      usePodStore.getState().fetchPod?.(data.podKey);
      debouncedSidebarRefresh(sidebarDebounceRef);
      debouncedFetchTopology();
      break;
    }
    case "pod:status_changed": {
      const data = decodeEventData(PodStatusChangedEventDataSchema, event.data);
      // Rust dispatch patches a cached pod; a not-yet-cached pod needs the
      // full entity from the server, which dispatch can't synthesize.
      if (!getPodState().get_pod_json(data.podKey)) {
        usePodStore.getState().fetchPod?.(data.podKey);
      } else {
        bumpPods();
      }
      if (data.status === "terminated" || data.status === "failed" || data.status === "error") {
        useWorkspaceStore.getState().removePaneByPodKey(data.podKey);
      }
      debouncedSidebarRefresh(sidebarDebounceRef);
      debouncedFetchTopology();
      break;
    }
    case "pod:agent_status_changed": {
      bumpPods();
      break;
    }
    case "pod:terminated": {
      const data = decodeEventData(PodStatusChangedEventDataSchema, event.data);
      useWorkspaceStore.getState().removePaneByPodKey(data.podKey);
      bumpPods();
      debouncedSidebarRefresh(sidebarDebounceRef);
      debouncedFetchTopology();
      break;
    }
    case "pod:title_changed":
    case "pod:alias_changed":
    case "pod:perpetual_changed": {
      bumpPods();
      break;
    }
    case "pod:init_progress": {
      // Init progress lives in a transient side-map (creating-pod UI), not the
      // pod list — keep the explicit apply path.
      const data = decodeEventData(PodInitProgressEventDataSchema, event.data);
      usePodStore.getState().updatePodInitProgress(data.podKey, data.phase, data.progress, data.message);
      break;
    }
    case "pod:restarting": {
      const data = decodeEventData(PodRestartingEventDataSchema, event.data);
      usePodStore.getState().fetchPod?.(data.podKey);
      debouncedSidebarRefresh(sidebarDebounceRef);
      break;
    }
  }
}
