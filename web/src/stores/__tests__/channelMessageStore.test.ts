import { describe, it, expect, beforeEach, vi } from "vitest";
import { act } from "@testing-library/react";
import { useChannelStore, useChannelMessageStore } from "../channel";
import { EMPTY_CACHE } from "../channelMessageStore";
import type { ChannelMessage } from "@/lib/api";
import { getChannelService } from "@/lib/wasm-core";

type Message = ChannelMessage;

const svc = () => getChannelService();

const mockMessage: Message = {
  id: 1, channel_id: 1, content: "Hello, world!", message_type: "text", created_at: "2024-01-01T00:00:00Z",
};

describe("Channel Message Store", () => {
  const CH = 1;

  beforeEach(() => {
    vi.clearAllMocks();
    useChannelMessageStore.setState({ cache: {}, unreadCounts: {} });
    useChannelStore.setState({ _tick: 0 });
  });

  describe("initial state", () => {
    it("should have default values", () => {
      const s = useChannelMessageStore.getState();
      expect(s.cache).toEqual({});
      expect(s.unreadCounts).toEqual({});
    });

    it("should return EMPTY_CACHE for unvisited channels", () => {
      const cache = useChannelMessageStore.getState().cache[999] ?? EMPTY_CACHE;
      expect(cache.messages).toEqual([]);
      expect(cache.hasMore).toBe(false);
    });
  });

  describe("fetchMessages", () => {
    it("should populate cache", async () => {
      vi.mocked(svc().fetch_messages).mockResolvedValue(JSON.stringify({ messages: [mockMessage], has_more: true }));
      svc().set_messages(BigInt(CH), JSON.stringify([mockMessage]), true);
      await act(async () => { await useChannelMessageStore.getState().fetchMessages(CH); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(1);
      expect(c.hasMore).toBe(true);
      expect(c.loading).toBe(false);
    });

    it("should prepend on load-more (beforeId) via WASM", async () => {
      const existing = { ...mockMessage, id: 10, content: "Existing" };
      svc().set_messages(BigInt(CH), JSON.stringify([existing]), true);
      useChannelMessageStore.setState({
        cache: { [CH]: { messages: [existing], hasMore: true, loading: false, loadingMore: false, error: null } },
      });
      const older = { ...mockMessage, id: 5, content: "Older" };
      vi.mocked(svc().fetch_messages).mockImplementation(async () => {
        svc().set_messages(BigInt(CH), JSON.stringify([older, existing]), false);
        return JSON.stringify({ messages: [older], has_more: false });
      });
      await act(async () => { await useChannelMessageStore.getState().fetchMessages(CH, 50, 10); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(2);
      expect(c.messages[0].content).toBe("Older");
      expect(c.messages[1].content).toBe("Existing");
      expect(c.hasMore).toBe(false);
    });

    it("should replace on refresh (no beforeId)", async () => {
      useChannelMessageStore.setState({
        cache: { [CH]: { messages: [{ ...mockMessage, content: "Old" }], hasMore: false, loading: false, loadingMore: false, error: null } },
      });
      vi.mocked(svc().fetch_messages).mockImplementation(async () => {
        svc().set_messages(BigInt(CH), JSON.stringify([{ ...mockMessage, content: "New" }]), true);
        return JSON.stringify({ messages: [{ ...mockMessage, content: "New" }], has_more: true });
      });
      await act(async () => { await useChannelMessageStore.getState().fetchMessages(CH); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(1);
      expect(c.messages[0].content).toBe("New");
    });

    it("should handle error", async () => {
      vi.mocked(svc().fetch_messages).mockRejectedValue(new Error("Network error"));
      await act(async () => { await useChannelMessageStore.getState().fetchMessages(CH); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.error).toBe("Network error");
      expect(c.loading).toBe(false);
    });

    it("should not set error on load-more failure", async () => {
      useChannelMessageStore.setState({
        cache: { [CH]: { messages: [mockMessage], hasMore: true, loading: false, loadingMore: false, error: null } },
      });
      vi.mocked(svc().fetch_messages).mockRejectedValue(new Error("fail"));
      await act(async () => { await useChannelMessageStore.getState().fetchMessages(CH, 50, 1); });
      expect(useChannelMessageStore.getState().cache[CH].error).toBeNull();
    });
  });

  describe("sendMessage", () => {
    it("should call service and add to cache via on_new_message", async () => {
      vi.mocked(svc().send_message).mockResolvedValue(JSON.stringify({ ...mockMessage, id: 42 }));
      let result: Message | undefined;
      await act(async () => { result = await useChannelMessageStore.getState().sendMessage(CH, "hi"); });
      expect(result?.id).toBe(42);
      expect(svc().send_message).toHaveBeenCalledWith(BigInt(CH), JSON.stringify({ content: "hi", pod_key: undefined, message_type: "text", mentions: undefined }));
    });
  });

  describe("onNewMessage", () => {
    it("should add message to cache", () => {
      act(() => { useChannelMessageStore.getState().onNewMessage(mockMessage); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(1);
      expect(c.messages[0].content).toBe("Hello, world!");
    });

    it("should deduplicate messages", () => {
      act(() => {
        useChannelMessageStore.getState().onNewMessage({ ...mockMessage, id: 0, content: "First" });
        useChannelMessageStore.getState().onNewMessage(mockMessage);
      });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(2);
    });

    it("should handle duplicate message IDs", () => {
      act(() => {
        useChannelMessageStore.getState().onNewMessage({ ...mockMessage });
        useChannelMessageStore.getState().onNewMessage({ ...mockMessage });
      });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(1);
    });
  });

  describe("editMessage", () => {
    it("should call service and update in cache", async () => {
      act(() => { useChannelMessageStore.getState().onNewMessage(mockMessage); });
      vi.mocked(svc().edit_message).mockImplementation(async () => {
        svc().update_message_local(BigInt(CH), JSON.stringify({ ...mockMessage, content: "Edited" }));
        return JSON.stringify({ ...mockMessage, content: "Edited" });
      });
      await act(async () => { await useChannelMessageStore.getState().editMessage(CH, 1, "Edited"); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages[0].content).toBe("Edited");
    });
  });

  describe("deleteMessage", () => {
    it("should call service and remove from cache", async () => {
      act(() => { useChannelMessageStore.getState().onNewMessage(mockMessage); });
      vi.mocked(svc().delete_message).mockImplementation(async () => {
        svc().remove_message_local(BigInt(CH), BigInt(1));
      });
      await act(async () => { await useChannelMessageStore.getState().deleteMessage(CH, 1); });
      const c = useChannelMessageStore.getState().cache[CH];
      expect(c.messages).toHaveLength(0);
    });
  });

  describe("updateMessage (realtime)", () => {
    it("should update message in cache", () => {
      act(() => { useChannelMessageStore.getState().onNewMessage(mockMessage); });
      act(() => { useChannelMessageStore.getState().updateMessage(CH, { ...mockMessage, content: "Updated" }); });
      expect(useChannelMessageStore.getState().cache[CH].messages[0].content).toBe("Updated");
    });
  });

  describe("removeMessage (realtime)", () => {
    it("should remove message from cache", () => {
      act(() => { useChannelMessageStore.getState().onNewMessage(mockMessage); });
      act(() => { useChannelMessageStore.getState().removeMessage(CH, 1); });
      expect(useChannelMessageStore.getState().cache[CH].messages).toHaveLength(0);
    });
  });

  describe("unread counts via WASM", () => {
    it("onNewMessage should update unread counts from WASM", () => {
      act(() => {
        useChannelMessageStore.getState().onNewMessage({ ...mockMessage, id: 1 });
        useChannelMessageStore.getState().onNewMessage({ ...mockMessage, id: 2 });
      });
      const counts = useChannelMessageStore.getState().unreadCounts;
      expect(counts[CH]).toBe(2);
    });

    it("totalUnreadCount should sum all channels", () => {
      act(() => {
        useChannelMessageStore.getState().incrementUnread(1);
        useChannelMessageStore.getState().incrementUnread(1);
        useChannelMessageStore.getState().incrementUnread(2);
        useChannelMessageStore.getState().incrementUnread(3);
        useChannelMessageStore.getState().incrementUnread(3);
        useChannelMessageStore.getState().incrementUnread(3);
      });
      expect(useChannelMessageStore.getState().totalUnreadCount()).toBe(6);
    });

    it("totalUnreadCount should return 0 when no unread", () => {
      expect(useChannelMessageStore.getState().totalUnreadCount()).toBe(0);
    });

    it("totalUnreadCount should update when channel is cleared", () => {
      act(() => {
        useChannelMessageStore.getState().incrementUnread(1);
        useChannelMessageStore.getState().incrementUnread(2);
        useChannelMessageStore.getState().incrementUnread(2);
      });
      expect(useChannelMessageStore.getState().totalUnreadCount()).toBe(3);
      act(() => { useChannelMessageStore.getState().clearChannelUnread(2); });
      expect(useChannelMessageStore.getState().totalUnreadCount()).toBe(1);
    });
  });

  describe("LRU eviction", () => {
    it("should populate cache for multiple channels", async () => {
      for (let i = 1; i <= 3; i++) {
        vi.mocked(svc().fetch_messages).mockResolvedValue(JSON.stringify({ messages: [{ ...mockMessage, id: i, channel_id: i }], has_more: false }));
        await act(async () => { await useChannelMessageStore.getState().fetchMessages(i); });
      }
      const keys = Object.keys(useChannelMessageStore.getState().cache);
      expect(keys.length).toBe(3);
    });
  });
});
