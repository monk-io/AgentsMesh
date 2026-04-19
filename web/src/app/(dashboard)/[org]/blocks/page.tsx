"use client";

import React, { useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { useTranslations } from "next-intl";

import { DocumentView } from "@/components/blocks/DocumentView";
import { SearchPanel } from "@/components/blocks/search/SearchPanel";
import { WorkspaceBreadcrumb } from "@/components/blocks/WorkspaceBreadcrumb";
import { Button } from "@/components/ui/button";
import { CenteredSpinner } from "@/components/ui/spinner";
import { getErrorMessage } from "@/lib/utils";
import { blockstoreApi } from "@/lib/api/blockstoreApi";
import type { Workspace } from "@/lib/api/blockstoreTypes";
import { useBlockstoreStore } from "@/stores/blockstore";
// Side-effect: registers reconnect handler + event handler hooks for store.
import "@/stores/blockstoreSubscribe";

export default function BlockstorePage() {
  const t = useTranslations();
  const searchParams = useSearchParams();
  const wsParam = searchParams.get("ws");
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [searchOpen, setSearchOpen] = useState(false);

  // hydrate runs on first load and on every workspace switch — extracted so
  // the initial EnsureDefault path and WorkspaceBreadcrumb.onSelect share
  // the same "set active + warm the store with type_defs" sequence.
  const hydrate = async (ws: Workspace) => {
    setWorkspace(ws);
    void useBlockstoreStore.getState().actions.loadTypeDefs(ws.id);
  };

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        // `?ws=<id>` lets callers (notably the E2E suite) target a specific
        // workspace instead of auto-ensuring the default one. The id is
        // matched against the listed workspaces so a bad id falls back to
        // the default rather than 404'ing the whole page.
        let ws: Workspace | null = null;
        if (wsParam) {
          const list = await blockstoreApi.listWorkspaces();
          ws = list.workspaces.find((w) => w.id === wsParam) ?? null;
        }
        if (!ws) {
          ws = await useBlockstoreStore.getState().actions.ensureDefaultWorkspace();
        }
        if (cancelled) return;
        await hydrate(ws);
      } catch (e) {
        if (!cancelled) setError(getErrorMessage(e, t("blockstore.loadFailed")));
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [t, wsParam]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setSearchOpen(true);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  if (error) {
    return <div className="p-6 text-sm text-destructive">{error}</div>;
  }

  if (!workspace || !workspace.root_block_id) {
    return <CenteredSpinner />;
  }

  return (
    <>
      <div className="flex items-center justify-between gap-2 px-8 pt-4">
        <WorkspaceBreadcrumb current={workspace} onSelect={(ws) => void hydrate(ws)} />
        <Button
          variant="outline"
          size="sm"
          onClick={() => setSearchOpen(true)}
          className="gap-2 text-xs"
        >
          <span>Search</span>
          <kbd className="rounded border bg-muted px-1 text-[10px]">⌘K</kbd>
        </Button>
      </div>
      <DocumentView workspaceID={workspace.id} rootBlockID={workspace.root_block_id} />
      <SearchPanel
        workspaceID={workspace.id}
        open={searchOpen}
        onClose={() => setSearchOpen(false)}
        onJumpToBlock={(blockID) => {
          const el = document.getElementById(`block-${blockID}`);
          el?.scrollIntoView({ behavior: "smooth", block: "center" });
          el?.classList.add("ring-2", "ring-primary");
          setTimeout(() => el?.classList.remove("ring-2", "ring-primary"), 1500);
        }}
      />
    </>
  );
}
