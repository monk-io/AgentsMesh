import { useAutopilotStore } from "@/stores/autopilot";
import { useLoopStore } from "@/stores/loop";
import { getLoopState, parseWasmAny } from "@/lib/wasm-core";
import type { DebounceRef } from "./realtimeEventHandlers";
import {
  type RealtimeEvent,
  decodeEventData,
  LoopRunEventDataSchema,
  LoopRunWarningEventDataSchema,
} from "@/lib/realtime";

export function handleAutopilotEvent(event: RealtimeEvent) {
  const store = useAutopilotStore.getState();
  switch (event.type) {
    case "autopilot:status_changed":
    case "autopilot:iteration":
    case "autopilot:thinking":
    case "autopilot:terminated": {
      // Rust event_dispatch owns the controller/iteration/thinking mutation in
      // runtime.state (update_controller / add_iteration / update_thinking /
      // remove_controller); bump triggers the React selectors to re-read. On
      // desktop the main-pushed autopilot snapshot mirrors the renderer caches.
      useAutopilotStore.setState((s) => ({ _tick: s._tick + 1 }));
      break;
    }
    case "autopilot:created": {
      // New controller needs its full payload from the server.
      store.fetchAutopilotControllers?.();
      break;
    }
  }
}

export function handleLoopEvent(
  event: RealtimeEvent,
  debounceRef: DebounceRef | undefined,
  t: (key: string, params?: Record<string, string | number>) => string,
  showWarning: (title: string, description: string) => void
) {
  switch (event.type) {
    case "loop_run:started":
    case "loop_run:completed":
    case "loop_run:failed": {
      if (!debounceRef) return;
      if (debounceRef.current) clearTimeout(debounceRef.current);
      debounceRef.current = setTimeout(() => {
        debounceRef.current = null;
        const s = useLoopStore.getState();
        s.fetchLoops?.();
        const currentLoop = parseWasmAny<{ id: number; slug: string }>(getLoopState().current_loop_json());
        const loopRunData = decodeEventData(LoopRunEventDataSchema, event.data);
        if (currentLoop?.id === Number(loopRunData.loopId)) {
          s.fetchLoop?.(currentLoop.slug);
          s.fetchRuns?.(currentLoop.slug, { limit: 20, offset: 0 });
        }
      }, 500);
      break;
    }
    case "loop_run:warning": {
      const data = decodeEventData(LoopRunWarningEventDataSchema, event.data);
      showWarning(t("loops.runWarningTitle", { runNumber: data.runNumber }), data.detail || data.warning);
      break;
    }
  }
}
