"use client";

import { usePod } from "@/stores/pod";
import { getPodDisplayName, getShortPodKey } from "@/lib/pod-utils";

export function usePodTitle(podKey: string, fallback?: string): string {
  const pod = usePod(podKey);
  if (pod) return getPodDisplayName(pod);
  return fallback ?? getShortPodKey(podKey);
}
