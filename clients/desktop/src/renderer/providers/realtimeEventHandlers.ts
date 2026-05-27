import { usePodStore } from "@/stores/pod";
import { useRunnerStore } from "@/stores/runner";
import { useTicketStore } from "@/stores/ticket";
import { useChannelMessageStore } from "@/stores/channel";
import { useAutopilotStore } from "@/stores/autopilot";
import { useLoopStore } from "@/stores/loop";
import {
  type RealtimeEvent,
  decodeEventData,
  RunnerStatusEventDataSchema,
  TicketStatusChangedEventDataSchema,
  ChannelMessageEventDataSchema,
  ChannelMessageEditedEventDataSchema,
  ChannelMessageDeletedEventDataSchema,
  AutopilotStatusChangedEventDataSchema,
  AutopilotIterationEventDataSchema,
  AutopilotTerminatedEventDataSchema,
  AutopilotThinkingEventDataSchema,
  MrEventDataSchema,
  PipelineEventDataSchema,
  LoopRunEventDataSchema,
  LoopRunWarningEventDataSchema,
} from "@/lib/realtime";
import type { MessageContent, MessageMentions } from "@/lib/viewModels/channelMessage";

export { handlePodEvent } from "./realtimePodHandlers";

type AutopilotDecisionType =
  | "continue" | "completed" | "need_help" | "give_up"
  | "CONTINUE" | "TASK_COMPLETED" | "NEED_HUMAN_HELP" | "GIVE_UP";

type AutopilotActionType = "observe" | "send_input" | "wait" | "none";

function parseMaybe<T>(json: string | undefined): T | undefined {
  if (!json) return undefined;
  try { return JSON.parse(json) as T; } catch { return undefined; }
}

export function handleChannelEvent(event: RealtimeEvent) {
  const msgState = useChannelMessageStore.getState();
  switch (event.type) {
    case "channel:message": {
      const data = decodeEventData(ChannelMessageEventDataSchema, event.data);
      const channelId = Number(data.channelId);
      const senderUserId = data.senderUserId != null ? Number(data.senderUserId) : undefined;
      msgState.addMessage(channelId, {
        id: Number(data.id),
        channel_id: channelId,
        sender_pod: data.senderPod,
        sender_user_id: senderUserId,
        message_type: data.messageType as "text" | "system" | "code" | "command",
        content: parseMaybe<MessageContent>(data.contentJson),
        mentions: parseMaybe<MessageMentions>(data.mentionsJson),
        created_at: data.createdAt,
        ...(senderUserId != null && data.senderName ? {
          sender_user: { id: senderUserId, username: data.senderName, name: data.senderName },
        } : {}),
      } as never);
      msgState.incrementUnread(channelId);
      break;
    }
    case "channel:message_edited": {
      const data = decodeEventData(ChannelMessageEditedEventDataSchema, event.data);
      msgState.updateMessage(Number(data.channelId), {
        id: Number(data.id),
        body: data.body,
        content: parseMaybe<MessageContent>(data.contentJson),
        mentions: parseMaybe<MessageMentions>(data.mentionsJson),
        edited_at: data.editedAt,
      } as never);
      break;
    }
    case "channel:message_deleted": {
      const data = decodeEventData(ChannelMessageDeletedEventDataSchema, event.data);
      msgState.removeMessage(Number(data.channelId), Number(data.id));
      break;
    }
  }
}

export function handleInfraEvent(event: RealtimeEvent) {
  switch (event.type) {
    case "runner:online":
    case "runner:offline":
    case "runner:updated": {
      const data = decodeEventData(RunnerStatusEventDataSchema, event.data);
      useRunnerStore.getState().updateRunnerStatus(Number(data.runnerId), data.status as "online" | "offline" | "maintenance" | "busy");
      break;
    }
    case "ticket:created":
    case "ticket:updated":
    case "ticket:status_changed":
    case "ticket:moved":
    case "ticket:deleted": {
      const data = decodeEventData(TicketStatusChangedEventDataSchema, event.data);
      const ticketState = useTicketStore.getState();
      ticketState.fetchTickets?.();
      if (event.type !== "ticket:deleted" && data.slug && ticketState.currentTicket?.slug === data.slug) {
        ticketState.fetchTicket?.(data.slug);
      }
      break;
    }
    case "mr:created":
    case "mr:updated":
    case "mr:merged":
    case "mr:closed": {
      const data = decodeEventData(MrEventDataSchema, event.data);
      if (data.ticketSlug || data.ticketId != null) useTicketStore.getState().fetchTickets?.();
      if (data.podId != null) usePodStore.getState().fetchPods?.();
      break;
    }
    case "pipeline:updated": {
      const data = decodeEventData(PipelineEventDataSchema, event.data);
      if (data.ticketSlug || data.ticketId != null) useTicketStore.getState().fetchTickets?.();
      if (data.podId != null) usePodStore.getState().fetchPods?.();
      break;
    }
  }
}

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
  debounceRef: React.MutableRefObject<ReturnType<typeof setTimeout> | null>,
  t: (key: string, params?: Record<string, string | number>) => string,
  showWarning: (title: string, description: string) => void
) {
  switch (event.type) {
    case "loop_run:started":
    case "loop_run:completed":
    case "loop_run:failed": {
      if (debounceRef.current) clearTimeout(debounceRef.current);
      debounceRef.current = setTimeout(() => {
        debounceRef.current = null;
        const s = useLoopStore.getState();
        s.fetchLoops?.();
        const data = decodeEventData(LoopRunEventDataSchema, event.data);
        if (s.currentLoop?.id === Number(data.loopId)) {
          s.fetchLoop?.(s.currentLoop.slug);
          s.fetchRuns?.(s.currentLoop.slug, { limit: 20, offset: 0 });
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
