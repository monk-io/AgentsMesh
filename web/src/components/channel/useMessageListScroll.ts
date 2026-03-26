"use client";

import { useEffect, useRef, useState, useCallback } from "react";
import type { TransformedMessage } from "./types";

interface UseMessageListScrollOptions {
  messages: TransformedMessage[];
  loading?: boolean;
  loadingMore?: boolean;
}

interface UseMessageListScrollReturn {
  containerRef: React.RefObject<HTMLDivElement | null>;
  bottomRef: React.RefObject<HTMLDivElement | null>;
  isAtBottom: boolean;
  newMessageCount: number;
  handleScroll: () => void;
  scrollToBottom: () => void;
}

/**
 * IM-standard scroll behavior for message lists.
 * Handles: stick-to-bottom, scroll position restoration on load-more,
 * new message counting, and jump-to-latest.
 */
export function useMessageListScroll({
  messages,
  loading,
  loadingMore,
}: UseMessageListScrollOptions): UseMessageListScrollReturn {
  const bottomRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Stick-to-bottom tracking
  const [isAtBottom, setIsAtBottom] = useState(true);
  const [newMessageCount, setNewMessageCount] = useState(0);

  const handleScroll = useCallback(() => {
    const el = containerRef.current;
    if (!el) return;
    const atBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 50;
    setIsAtBottom(atBottom);
  }, []);

  // Reset new message count when user scrolls to bottom
  useEffect(() => {
    if (isAtBottom) setNewMessageCount(0);
  }, [isAtBottom]);

  // Scroll behavior: 3 distinct scenarios
  const prevStateRef = useRef<{ length: number; firstId?: number }>({ length: 0 });
  const wasLoadingMoreRef = useRef(false);
  const initialLoadDone = useRef(false);

  useEffect(() => {
    if (loadingMore) wasLoadingMoreRef.current = true;
  }, [loadingMore]);

  useEffect(() => {
    const prev = prevStateRef.current;
    const firstId = messages.length > 0 ? messages[0].id : undefined;

    if (wasLoadingMoreRef.current && !loadingMore) {
      // Load-more completed — restore scroll to previous first message
      wasLoadingMoreRef.current = false;
      if (prev.firstId != null && containerRef.current) {
        const el = containerRef.current.querySelector(`[data-message-id="${prev.firstId}"]`);
        if (el) el.scrollIntoView({ block: "start" });
      }
    } else if (!initialLoadDone.current && messages.length > 0 && !loading) {
      // Initial load completed — instant scroll to bottom
      initialLoadDone.current = true;
      bottomRef.current?.scrollIntoView({ behavior: "instant" as ScrollBehavior });
    } else if (messages.length > prev.length && prev.length > 0) {
      // New messages appended (WebSocket / send)
      if (isAtBottom) {
        bottomRef.current?.scrollIntoView({ behavior: "smooth" });
      } else {
        setNewMessageCount((c) => c + (messages.length - prev.length));
      }
    }

    prevStateRef.current = { length: messages.length, firstId };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [messages, loadingMore, loading, isAtBottom]);

  // Reset on channel switch (messages become empty)
  useEffect(() => {
    if (messages.length === 0) {
      initialLoadDone.current = false;
      prevStateRef.current = { length: 0 };
      setNewMessageCount(0);
      setIsAtBottom(true);
    }
  }, [messages.length]);

  const scrollToBottom = useCallback(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
    setNewMessageCount(0);
  }, []);

  return { containerRef, bottomRef, isAtBottom, newMessageCount, handleScroll, scrollToBottom };
}
