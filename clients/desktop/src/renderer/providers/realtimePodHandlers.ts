import { usePodStore } from "@/stores/pod";
import { useWorkspaceStore } from "@/stores/workspace";
import { useMeshStore } from "@/stores/mesh";
import {
  type RealtimeEvent,
  decodeEventData,
  PodStatusChangedEventDataSchema,
  PodCreatedEventDataSchema,
  PodTitleChangedEventDataSchema,
  PodAliasChangedEventDataSchema,
  PodInitProgressEventDataSchema,
  PodRestartingEventDataSchema,
} from "@/lib/realtime";

// Debounce burst-y refetches: terminate fires status_changed+terminated back-to-back
// and reconnect catchup can replay many events. Without this, setState storm links to React #185.
let sidebarTimer: ReturnType<typeof setTimeout> | null = null;
function debouncedSidebarRefresh() {
  if (sidebarTimer) clearTimeout(sidebarTimer);
  sidebarTimer = setTimeout(() => {
    sidebarTimer = null;
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

export function handlePodEvent(event: RealtimeEvent) {
  switch (event.type) {
    case "pod:created": {
      const data = decodeEventData(PodCreatedEventDataSchema, event.data);
      usePodStore.getState().fetchPod?.(data.podKey);
      debouncedSidebarRefresh();
      debouncedFetchTopology();
      break;
    }
    case "pod:status_changed": {
      const data = decodeEventData(PodStatusChangedEventDataSchema, event.data);
      const podState = usePodStore.getState();
      const existingPod = podState.pods.find(p => p.pod_key === data.podKey);
      if (!existingPod) {
        podState.fetchPod?.(data.podKey);
      } else if (podState.updatePodStatus) {
        podState.updatePodStatus(data.podKey, data.status as "running" | "initializing" | "failed" | "paused" | "terminated" | "error", data.agentStatus, data.errorCode, data.errorMessage);
      }
      if (data.status === "terminated" || data.status === "failed" || data.status === "error") {
        useWorkspaceStore.getState().removePaneByPodKey(data.podKey);
      }
      debouncedFetchTopology();
      break;
    }
    case "pod:agent_status_changed": {
      const data = decodeEventData(PodStatusChangedEventDataSchema, event.data);
      if (data.agentStatus) usePodStore.getState().updateAgentStatus(data.podKey, data.agentStatus);
      break;
    }
    case "pod:terminated": {
      const data = decodeEventData(PodStatusChangedEventDataSchema, event.data);
      usePodStore.getState().updatePodStatus?.(data.podKey, "terminated");
      useWorkspaceStore.getState().removePaneByPodKey(data.podKey);
      debouncedFetchTopology();
      break;
    }
    case "pod:title_changed": {
      const data = decodeEventData(PodTitleChangedEventDataSchema, event.data);
      usePodStore.getState().updatePodTitle(data.podKey, data.title);
      break;
    }
    case "pod:alias_changed": {
      const data = decodeEventData(PodAliasChangedEventDataSchema, event.data);
      usePodStore.getState().updatePodAliasFromEvent(data.podKey, data.alias ?? null);
      break;
    }
    case "pod:init_progress": {
      const data = decodeEventData(PodInitProgressEventDataSchema, event.data);
      usePodStore.getState().updatePodInitProgress(data.podKey, data.phase, data.progress, data.message);
      break;
    }
    case "pod:restarting": {
      const data = decodeEventData(PodRestartingEventDataSchema, event.data);
      usePodStore.getState().fetchPod?.(data.podKey);
      debouncedSidebarRefresh();
      break;
    }
  }
}
