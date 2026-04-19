import { create } from "zustand";
import type { ChannelMessage, MentionPayload } from "@/lib/api/channelTypes";
import { getErrorMessage } from "@/lib/utils";
import { getChannelService } from "@/lib/wasm-core";

export interface ChannelMessageCache {
  messages: ChannelMessage[]; hasMore: boolean; loading: boolean; loadingMore: boolean; error: string | null;
}
export const EMPTY_CACHE: ChannelMessageCache = { messages: [], hasMore: false, loading: false, loadingMore: false, error: null };
export const INITIAL_MESSAGE_LIMIT = 20;
export const LOAD_MORE_MESSAGE_LIMIT = 30;

export interface ChannelMessageState {
  cache: Record<number, ChannelMessageCache>; unreadCounts: Record<number, number>;
  fetchMessages: (channelId: number, limit?: number, beforeId?: number) => Promise<void>;
  sendMessage: (channelId: number, content: string, podKey?: string, mentions?: MentionPayload[]) => Promise<ChannelMessage>;
  onNewMessage: (message: ChannelMessage) => void;
  editMessage: (channelId: number, messageId: number, content: string) => Promise<void>;
  deleteMessage: (channelId: number, messageId: number) => Promise<void>;
  updateMessage: (channelId: number, data: Partial<ChannelMessage> & { id: number }) => void;
  removeMessage: (channelId: number, messageId: number) => void;
  fetchUnreadCounts: () => Promise<void>; markRead: (channelId: number, messageId: number) => Promise<void>;
  muteChannel: (channelId: number, muted: boolean) => Promise<void>;
  incrementUnread: (channelId: number) => void;
  clearChannelUnread: (channelId: number) => void;
  totalUnreadCount: () => number;
}

const svc = () => getChannelService();

const getC = (s: ChannelMessageState, id: number) => s.cache[id] ?? EMPTY_CACHE;
const setC = (s: ChannelMessageState, id: number, p: Partial<ChannelMessageCache>) => ({
  cache: { ...s.cache, [id]: { ...getC(s, id), ...p } },
});

function readMessages(channelId: number): { messages: ChannelMessage[]; hasMore: boolean } {
  const raw = svc().get_messages_json(BigInt(channelId));
  if (!raw) return { messages: [], hasMore: false };
  const parsed = typeof raw === "string" ? JSON.parse(raw) : raw;
  return { messages: parsed.messages || [], hasMore: parsed.has_more ?? false };
}

function readUnreadCounts(): Record<number, number> {
  return JSON.parse(svc().unread_counts_json());
}

export const useChannelMessageStore = create<ChannelMessageState>((set, get) => {
  return {
    cache: {}, unreadCounts: {},

    fetchMessages: async (channelId, limit = INITIAL_MESSAGE_LIMIT, beforeId) => {
      const isMore = beforeId !== undefined;
      const cur = getC(get(), channelId);
      if (isMore ? cur.loadingMore : cur.loading) return;
      set((s) => setC(s, channelId, isMore ? { loadingMore: true } : { loading: true, error: null }));
      try {
        await svc().fetch_messages(BigInt(channelId), limit, beforeId !== undefined ? BigInt(beforeId) : undefined);
        const synced = readMessages(channelId);
        set((s) => setC(s, channelId, { messages: synced.messages, hasMore: synced.hasMore, loading: false, loadingMore: false, error: null }));
      } catch (e: unknown) { set((s) => setC(s, channelId, { loading: false, loadingMore: false, error: isMore ? null : getErrorMessage(e, "Unknown error") })); }
    },

    sendMessage: async (channelId, content, podKey, mentions) => {
      const json = await svc().send_message(BigInt(channelId), JSON.stringify({ content, pod_key: podKey, message_type: "text", mentions }));
      const msg = JSON.parse(json) as ChannelMessage;
      set((s) => ({ ...setC(s, channelId, readMessages(channelId)), unreadCounts: readUnreadCounts() }));
      return msg;
    },

    onNewMessage: (message) => {
      svc().on_new_message(JSON.stringify(message));
      const channelId = message.channel_id;
      set((s) => ({ ...setC(s, channelId, readMessages(channelId)), unreadCounts: readUnreadCounts() }));
    },

    editMessage: async (channelId, messageId, content) => {
      await svc().edit_message(BigInt(channelId), BigInt(messageId), content);
      set((s) => setC(s, channelId, readMessages(channelId)));
    },

    deleteMessage: async (channelId, messageId) => {
      await svc().delete_message(BigInt(channelId), BigInt(messageId));
      set((s) => setC(s, channelId, readMessages(channelId)));
    },

    updateMessage: (channelId, data) => {
      svc().update_message_local(BigInt(channelId), JSON.stringify(data));
      set((s) => setC(s, channelId, readMessages(channelId)));
    },

    removeMessage: (channelId, messageId) => {
      svc().remove_message_local(BigInt(channelId), BigInt(messageId));
      set((s) => setC(s, channelId, readMessages(channelId)));
    },

    fetchUnreadCounts: async () => {
      try {
        await svc().fetch_unread_counts();
        set({ unreadCounts: readUnreadCounts() });
      } catch { /* silent */ }
    },

    markRead: async (channelId, messageId) => {
      try {
        await svc().mark_read(BigInt(channelId), BigInt(messageId));
        set({ unreadCounts: readUnreadCounts() });
      } catch { /* silent */ }
    },

    muteChannel: async (channelId, muted) => { await svc().mute_channel(BigInt(channelId), muted); },

    incrementUnread: (channelId) => {
      set((state) => ({
        unreadCounts: {
          ...state.unreadCounts,
          [channelId]: (state.unreadCounts[channelId] ?? 0) + 1,
        },
      }));
    },

    clearChannelUnread: (channelId) => {
      set((state) => {
        if (!(channelId in state.unreadCounts)) return {};
        const counts = { ...state.unreadCounts };
        delete counts[channelId];
        return { unreadCounts: counts };
      });
    },

    totalUnreadCount: () => {
      return Object.values(get().unreadCounts).reduce((sum, c) => sum + c, 0);
    },
  };
});
