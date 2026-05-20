"use client";

import { useEffect, useState } from "react";
import { getEnvBundleService } from "@/lib/wasm-core";
import type { EnvBundleSummary } from "@/lib/api";

interface WireEnvBundle {
  id: number;
  agent_slug?: string | null;
  name: string;
  kind: string;
  kind_primary: boolean;
  configured_fields?: string[];
}

/**
 * useLoopEnvBundles — fetch credential + runtime EnvBundles for the selected
 * agent. Reactive to dialog `open` state and the agent slug; cancels in-flight
 * loads when either changes so we never set state on a stale request.
 *
 * Mirrors `useEnvBundles` in CreatePodForm: both kinds load in parallel so one
 * empty/failed list doesn't suppress the other.
 */
export function useLoopEnvBundles(args: {
  open: boolean;
  agentSlug: string | null;
}): {
  envBundles: EnvBundleSummary[];
  loadingBundles: boolean;
} {
  const { open, agentSlug } = args;
  const [envBundles, setEnvBundles] = useState<EnvBundleSummary[]>([]);
  const [loadingBundles, setLoadingBundles] = useState(false);

  useEffect(() => {
    if (!open || !agentSlug) {
      setEnvBundles([]);
      return;
    }
    let cancelled = false;
    const load = async () => {
      setLoadingBundles(true);
      try {
        const svc = getEnvBundleService();
        const [credRes, runtimeRes] = await Promise.all([
          svc.list("credential", agentSlug).then((j: string) => JSON.parse(j)).catch(() => ({ items: [] })),
          svc.list("runtime", agentSlug).then((j: string) => JSON.parse(j)).catch(() => ({ items: [] })),
        ]);
        if (cancelled) return;
        const mapBundle = (b: WireEnvBundle): EnvBundleSummary => ({
          id: b.id,
          name: b.name,
          agent_slug: b.agent_slug ?? agentSlug,
          kind: b.kind,
          kind_primary: b.kind_primary,
          configured_fields: b.configured_fields,
        });
        const credBundles: EnvBundleSummary[] = (credRes.items ?? []).map(mapBundle);
        const runtimeBundles: EnvBundleSummary[] = (runtimeRes.items ?? []).map(mapBundle);
        setEnvBundles([...credBundles, ...runtimeBundles]);
      } catch {
        if (!cancelled) setEnvBundles([]);
      } finally {
        if (!cancelled) setLoadingBundles(false);
      }
    };
    load();
    return () => {
      cancelled = true;
    };
  }, [open, agentSlug]);

  return { envBundles, loadingBundles };
}
