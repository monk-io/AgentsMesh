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
  PodTitleChangedEventDataSchema,
  PodAliasChangedEventDataSchema,
  PodInitProgressEventDataSchema,
  PodPerpetualChangedEventDataSchema,
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
      const podState = usePodStore.getState();
      const existingPodJson = getPodState().get_pod_json(data.podKey);
      if (!existingPodJson) {
        podState.fetchPod?.(data.podKey);
      } else {
        podState.updatePodStatus(data.podKey, data.status as "running" | "initializing" | "failed" | "paused" | "terminated" | "error", data.agentStatus, data.errorCode, data.errorMessage);
      }
      if (data.status === "terminated" || data.status === "failed" || data.status === "error") {
        useWorkspaceStore.getState().removePaneByPodKey(data.podKey);
      }
      debouncedSidebarRefresh(sidebarDebounceRef);
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
      debouncedSidebarRefresh(sidebarDebounceRef);
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
      debouncedSidebarRefresh(sidebarDebounceRef);
      break;
    }
    case "pod:perpetual_changed": {
      const data = decodeEventData(PodPerpetualChangedEventDataSchema, event.data);
      usePodStore.getState().updatePodPerpetualFromEvent(data.podKey, data.perpetual);
      break;
    }
  }
}
