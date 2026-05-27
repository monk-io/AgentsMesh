import { useAutopilotStore } from "@/stores/autopilot";
import { useLoopStore } from "@/stores/loop";
import { getLoopService, parseWasmAny } from "@/lib/wasm-core";
import type { DebounceRef } from "./realtimeEventHandlers";
import {
  type RealtimeEvent,
  decodeEventData,
  AutopilotStatusChangedEventDataSchema,
  AutopilotIterationEventDataSchema,
  AutopilotTerminatedEventDataSchema,
  AutopilotThinkingEventDataSchema,
  LoopRunEventDataSchema,
  LoopRunWarningEventDataSchema,
} from "@/lib/realtime";

type AutopilotDecisionType =
  | "continue" | "completed" | "need_help" | "give_up"
  | "CONTINUE" | "TASK_COMPLETED" | "NEED_HUMAN_HELP" | "GIVE_UP";

type AutopilotActionType = "observe" | "send_input" | "wait" | "none";

export function handleAutopilotEvent(event: RealtimeEvent) {
  const store = useAutopilotStore.getState();
  switch (event.type) {
    case "autopilot:status_changed": {
      const data = decodeEventData(AutopilotStatusChangedEventDataSchema, event.data);
      store.updateAutopilotControllerStatus(
        data.autopilotControllerKey,
        data.phase,
        data.currentIteration,
        data.maxIterations,
        data.circuitBreakerState,
        data.circuitBreakerReason,
      );
      break;
    }
    case "autopilot:iteration": {
      const data = decodeEventData(AutopilotIterationEventDataSchema, event.data);
      store.addIteration(data.autopilotControllerKey, {
        id: 0,
        autopilot_controller_id: 0,
        iteration: data.iteration,
        phase: data.phase,
        summary: data.summary,
        files_changed: data.filesChanged,
        duration_ms: Number(data.durationMs),
        created_at: new Date().toISOString(),
      });
      break;
    }
    case "autopilot:created": {
      store.fetchAutopilotControllers?.();
      break;
    }
    case "autopilot:terminated": {
      const data = decodeEventData(AutopilotTerminatedEventDataSchema, event.data);
      store.removeAutopilotController(data.autopilotControllerKey);
      break;
    }
    case "autopilot:thinking": {
      const data = decodeEventData(AutopilotThinkingEventDataSchema, event.data);
      store.updateThinking(data.autopilotControllerKey, {
        autopilot_controller_key: data.autopilotControllerKey,
        iteration: data.iteration,
        decision_type: data.decisionType as AutopilotDecisionType,
        reasoning: data.reasoning,
        confidence: data.confidence,
        ...(data.action ? {
          action: {
            type: data.action.type as AutopilotActionType,
            content: data.action.content,
            reason: data.action.reason,
          },
        } : {}),
        ...(data.progress ? {
          progress: {
            summary: data.progress.summary,
            completed_steps: data.progress.completedSteps,
            remaining_steps: data.progress.remainingSteps,
            percent: data.progress.percent,
          },
        } : {}),
        ...(data.helpRequest ? {
          help_request: {
            reason: data.helpRequest.reason,
            context: data.helpRequest.context,
            terminal_excerpt: data.helpRequest.terminalExcerpt,
            suggestions: data.helpRequest.suggestions.map((s) => ({ action: s.action, label: s.label })),
          },
        } : {}),
      });
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
        const currentLoop = parseWasmAny<{ id: number; slug: string }>(getLoopService().current_loop_json());
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
