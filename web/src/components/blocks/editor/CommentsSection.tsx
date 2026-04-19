"use client";

import React, { useMemo, useState } from "react";
import { MessageSquare, Send } from "lucide-react";

import type { BlockRef } from "@/lib/api/blockstoreTypes";
import { BLOCK_TYPE_COMMENT, REL_COMMENTS_ON } from "@/lib/api/blockstoreTypes";
import { cn } from "@/lib/utils";
import { useBlockstoreStore } from "@/stores/blockstore";

import { useBlockstoreDispatch } from "./useBlockstoreDispatch";

interface Props {
  blockID: string;
  workspaceID: string;
}

/**
 * CommentsSection renders a collapsible thread of `comment` blocks anchored
 * on `blockID` via rel='comments_on'. Writing a comment posts two ops in one
 * batch: createBlock(comment) + addRef(comments_on). The block store's
 * realtime push surfaces the new comment to every viewer without any
 * special-case wiring.
 */
export function CommentsSection({ blockID, workspaceID }: Props) {
  const [open, setOpen] = useState(false);
  const [draft, setDraft] = useState("");

  // Subscribe to the raw refs map and derive filtered list via useMemo so
  // each render sees a stable array identity. Using a filtering selector
  // directly would return a new array every render and hit React's
  // `getSnapshot should be cached` infinite-loop guard.
  const refs = useBlockstoreStore((s) => s.refs);
  const commentRefs = useMemo(
    () =>
      Object.values(refs).filter(
        (r) => r.to_id === blockID && r.rel === REL_COMMENTS_ON,
      ),
    [refs, blockID],
  );
  const count = commentRefs.length;

  return (
    <div className="pl-1">
      <button
        type="button"
        onClick={() => setOpen((v) => !v)}
        className={cn(
          "inline-flex items-center gap-1 rounded px-2 py-0.5 text-xs transition-colors",
          count > 0
            ? "text-foreground/80 hover:bg-accent"
            : "text-muted-foreground/70 hover:bg-accent hover:text-foreground",
        )}
      >
        <MessageSquare className="h-3 w-3" />
        {count === 0 ? "Add comment" : `${count} comment${count === 1 ? "" : "s"}`}
      </button>
      {open && (
        <div className="mt-1 flex flex-col gap-1 rounded border border-border bg-muted/30 p-2 text-xs">
          {commentRefs.length === 0 && (
            <p className="text-muted-foreground">No comments yet.</p>
          )}
          {commentRefs.map((commentRef) => (
            <CommentItem key={commentRef.id} commentRef={commentRef} />
          ))}
          <CommentComposer
            blockID={blockID}
            workspaceID={workspaceID}
            draft={draft}
            setDraft={setDraft}
          />
        </div>
      )}
    </div>
  );
}

function CommentItem({ commentRef }: { commentRef: BlockRef }) {
  const comment = useBlockstoreStore((s) => s.blocks[commentRef.from_id]);
  if (!comment) return null;
  const text = (comment.data?.text as string | undefined) ?? "";
  const when = new Date(comment.created_at).toLocaleString();
  return (
    <div className="rounded bg-background p-1.5 shadow-sm">
      <div className="flex items-baseline justify-between text-[10px] text-muted-foreground">
        <span>user #{comment.created_by}</span>
        <span>{when}</span>
      </div>
      <div className="whitespace-pre-wrap break-words text-sm">{text}</div>
    </div>
  );
}

function CommentComposer({
  blockID,
  workspaceID,
  draft,
  setDraft,
}: {
  blockID: string;
  workspaceID: string;
  draft: string;
  setDraft: (v: string) => void;
}) {
  const dispatch = useBlockstoreDispatch(workspaceID);
  const canPost = draft.trim().length > 0;

  const post = async () => {
    if (!canPost) return;
    await dispatch.createCommentOn(blockID, draft.trim());
    setDraft("");
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        void post();
      }}
      className="mt-1 flex gap-1"
    >
      <input
        type="text"
        value={draft}
        onChange={(e) => setDraft(e.target.value)}
        placeholder="Write a comment…"
        className="flex-1 rounded border border-border bg-background px-2 py-1 text-sm outline-none focus:ring-1 focus:ring-ring"
      />
      <button
        type="submit"
        disabled={!canPost}
        className={cn(
          "rounded px-2 py-1 text-xs",
          canPost
            ? "bg-primary text-primary-foreground hover:opacity-90"
            : "bg-muted text-muted-foreground",
        )}
      >
        <Send className="h-3 w-3" />
      </button>
    </form>
  );
}

// (Re-export the type so callers do not need to touch BLOCK_TYPE_COMMENT here.)
export { BLOCK_TYPE_COMMENT };
