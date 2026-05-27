import { usePodStore } from "@/stores/pod";
import { useRunnerStore } from "@/stores/runner";
import { getTicketState, getPodState, parseWasmAny } from "@/lib/wasm-core";
import { useTicketStore } from "@/stores/ticket";
import { useChannelStore, useChannelMessageStore } from "@/stores/channel";
import { readCurrentUser } from "@/stores/auth";
import type { PodData } from "@/lib/api/facade/pod";
import {
  type RealtimeEvent,
  decodeEventData,
  RunnerStatusEventDataSchema,
  TicketStatusChangedEventDataSchema,
  ChannelMessageEventDataSchema,
  ChannelMessageEditedEventDataSchema,
  ChannelMessageDeletedEventDataSchema,
  ChannelMemberChangedEventDataSchema,
  MrEventDataSchema,
  PipelineEventDataSchema,
} from "@/lib/realtime";
import type { MessageContent, MessageMentions } from "@/lib/viewModels/channelMessage";

export { handlePodEvent } from "./realtimePodHandlers";
export { handleAutopilotEvent, handleLoopEvent } from "./realtimeFeatureHandlers";
export { handleBlockstoreEvent } from "@/stores/blockstoreSubscribe";

export type DebounceRef = React.MutableRefObject<ReturnType<typeof setTimeout> | null>;

function parseMaybe<T>(json: string | undefined): T | undefined {
  if (!json) return undefined;
  try { return JSON.parse(json) as T; } catch { return undefined; }
}

export function handleChannelEvent(event: RealtimeEvent, channelDebounceRef?: DebounceRef) {
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
        message_type: data.messageType,
        body: data.body,
        content: parseMaybe<MessageContent>(data.contentJson),
        mentions: parseMaybe<MessageMentions>(data.mentionsJson),
        reply_to: data.replyTo != null ? Number(data.replyTo) : undefined,
        created_at: data.createdAt,
        ...(data.senderPodInfo ? {
          sender_pod_info: {
            pod_key: data.senderPodInfo.podKey,
            ...(data.senderPodInfo.alias != null ? { alias: data.senderPodInfo.alias } : {}),
            ...(data.senderPodInfo.agent ? { agent: { name: data.senderPodInfo.agent.name } } : {}),
          },
        } : {}),
        ...(senderUserId != null && data.senderName ? {
          sender_user: { id: senderUserId, username: data.senderName, name: data.senderName },
        } : {}),
      });
      const currentUserId = readCurrentUser()?.id;
      const viewingChannelId = useChannelStore.getState().selectedChannelId;
      const isSelf = currentUserId != null && senderUserId === currentUserId;
      const isViewing = viewingChannelId === channelId;
      if (!isSelf && !isViewing) {
        msgState.incrementUnread(channelId);
      }
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
      });
      break;
    }
    case "channel:message_deleted": {
      const data = decodeEventData(ChannelMessageDeletedEventDataSchema, event.data);
      msgState.removeMessage(Number(data.channelId), Number(data.id));
      break;
    }
    case "channel:member_added":
    case "channel:member_removed": {
      const data = decodeEventData(ChannelMemberChangedEventDataSchema, event.data);
      const channelId = Number(data.channelId);
      const chState = useChannelStore.getState();
      const delta = event.type === "channel:member_added" ? 1 : -1;
      chState.patchChannelMemberCount?.(channelId, delta);
      if (chState.currentChannel?.id === channelId && channelDebounceRef) {
        if (channelDebounceRef.current) clearTimeout(channelDebounceRef.current);
        channelDebounceRef.current = setTimeout(() => {
          channelDebounceRef.current = null;
          useChannelStore.getState().fetchChannel?.(channelId);
        }, 300);
      }
      break;
    }
  }
}

export function handleInfraEvent(event: RealtimeEvent, ticketDebounceRef?: DebounceRef) {
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

      if (event.type === "ticket:status_changed" && data.slug && data.status) {
        ticketState.updateTicketStatusFromEvent?.(data.slug, data.status, data.previousStatus);
      } else if (event.type === "ticket:deleted" && data.slug) {
        ticketState.removeTicketFromEvent?.(data.slug);
      }

      debouncedTicketRefetch(ticketDebounceRef);
      break;
    }
    case "mr:created":
    case "mr:updated":
    case "mr:merged":
    case "mr:closed": {
      const data = decodeEventData(MrEventDataSchema, event.data);
      if (data.ticketSlug) useTicketStore.getState().fetchTicket?.(data.ticketSlug);
      if (data.podId != null) {
        const podId = Number(data.podId);
        const pods = JSON.parse(getPodState().pods_json()) as PodData[];
        const pod = pods.find((p) => p.id === podId);
        if (pod) usePodStore.getState().fetchPod?.(pod.pod_key);
      }
      break;
    }
    case "pipeline:updated": {
      const data = decodeEventData(PipelineEventDataSchema, event.data);
      if (data.ticketSlug) useTicketStore.getState().fetchTicket?.(data.ticketSlug);
      if (data.podId != null) {
        const podId = Number(data.podId);
        const pods = JSON.parse(getPodState().pods_json()) as PodData[];
        const pod = pods.find((p) => p.id === podId);
        if (pod) usePodStore.getState().fetchPod?.(pod.pod_key);
      }
      break;
    }
  }
}

function debouncedTicketRefetch(debounceRef: DebounceRef | undefined) {
  if (!debounceRef) {
    useTicketStore.getState().fetchTickets?.();
    return;
  }
  if (debounceRef.current) clearTimeout(debounceRef.current);
  debounceRef.current = setTimeout(() => {
    debounceRef.current = null;
    const s = useTicketStore.getState();
    s.fetchTickets?.();
    const currentTicket = parseWasmAny<{ slug: string }>(getTicketState().current_ticket_json());
    if (currentTicket?.slug) {
      s.fetchTicket?.(currentTicket.slug);
    }
  }, 500);
}
