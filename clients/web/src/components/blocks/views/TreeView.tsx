"use client";

import React from "react";

import type { Block, ViewSpec } from "@/lib/api/blockstoreTypes";

import { BlockRenderer } from "../BlockRenderer";

import { ViewHeader } from "./ViewListFallback";
import { SummaryBar } from "./SummaryBar";
import { useViewBlocks } from "./useViewBlocks";

// TreeView renders the view's source blocks as top-level roots, each recursive
// via nest refs. Differs from DocumentView in that it never renders the view
// block's own content — only projects its source rows as a forest.
export function TreeView({ viewBlock, spec }: { viewBlock: Block; spec: ViewSpec }) {
  const roots = useViewBlocks(spec, viewBlock.workspace_id);
  return (
    <section className="flex flex-col gap-3">
      <ViewHeader title={spec.title ?? `${spec.source_type} · tree`} count={roots.length} />
      <SummaryBar blocks={roots} summaryColumns={spec.summary_columns ?? []} />
      <div className="flex flex-col gap-1 border-l border-muted-foreground/20 pl-3">
        {roots.map((b) => (
          <BlockRenderer key={b.id} blockID={b.id} depth={0} />
        ))}
      </div>
    </section>
  );
}
