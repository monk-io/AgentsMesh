"use client";

import React, { useEffect, useRef } from "react";

import { cn } from "@/lib/utils";

export interface EditableTextProps {
  value: string;
  onChange: (next: string) => void;
  placeholder?: string;
  className?: string;
  debounceMs?: number;
  /** Fired when Enter is pressed (without Shift). Caller typically inserts a sibling block. */
  onEnter?: () => void;
  /** Fired when Backspace is pressed while the editor is empty. Typically deletes the block. */
  onBackspaceEmpty?: () => void;
  /** Fired when "/" is pressed on an empty editor. Caller typically opens a SlashMenu. */
  onSlashOnEmpty?: () => void;
  /** When flipped to true, the editor grabs focus and positions the caret at the end. */
  autoFocus?: boolean;
}

// Minimal contentEditable wrapper. Keeps the DOM text in sync with `value`
// without re-rendering while the user is typing (re-rendering blows away the
// caret). Changes are flushed on blur and on a debounce timer so downstream
// dispatch isn't called on every keystroke.
export function EditableText({
  value,
  onChange,
  placeholder,
  className,
  debounceMs = 300,
  onEnter,
  onBackspaceEmpty,
  onSlashOnEmpty,
  autoFocus,
}: EditableTextProps) {
  const ref = useRef<HTMLDivElement | null>(null);
  const lastEmitted = useRef(value);
  const pendingTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const node = ref.current;
    if (!node) return;
    if (node.innerText !== value) {
      node.innerText = value;
      lastEmitted.current = value;
    }
  }, [value]);

  useEffect(() => {
    if (!autoFocus) return;
    const node = ref.current;
    if (!node) return;
    node.focus();
    placeCaretAtEnd(node);
  }, [autoFocus]);

  const flush = () => {
    const node = ref.current;
    if (!node) return;
    const next = node.innerText;
    if (next !== lastEmitted.current) {
      lastEmitted.current = next;
      onChange(next);
    }
  };

  const scheduleFlush = () => {
    if (pendingTimer.current) clearTimeout(pendingTimer.current);
    pendingTimer.current = setTimeout(flush, debounceMs);
  };

  useEffect(() => {
    return () => {
      if (pendingTimer.current) clearTimeout(pendingTimer.current);
    };
  }, []);

  const onKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Enter" && !e.shiftKey && onEnter) {
      e.preventDefault();
      flush();
      onEnter();
      return;
    }
    if (e.key === "Backspace" && onBackspaceEmpty) {
      const node = ref.current;
      if (node && node.innerText.length === 0) {
        e.preventDefault();
        onBackspaceEmpty();
      }
    }
    if (e.key === "/" && onSlashOnEmpty) {
      const node = ref.current;
      if (node && node.innerText.length === 0) {
        e.preventDefault();
        onSlashOnEmpty();
      }
    }
  };

  const isEmpty = value.length === 0;

  return (
    <div
      ref={ref}
      contentEditable
      suppressContentEditableWarning
      onInput={scheduleFlush}
      onBlur={flush}
      onKeyDown={onKeyDown}
      data-placeholder={placeholder}
      className={cn(
        "min-h-[1.25em] whitespace-pre-wrap break-words",
        isEmpty && "before:text-muted-foreground/60 before:content-[attr(data-placeholder)]",
        className,
      )}
    />
  );
}

function placeCaretAtEnd(node: HTMLElement) {
  const range = document.createRange();
  range.selectNodeContents(node);
  range.collapse(false);
  const sel = window.getSelection();
  if (!sel) return;
  sel.removeAllRanges();
  sel.addRange(range);
}
