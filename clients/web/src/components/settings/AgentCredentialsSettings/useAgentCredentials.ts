"use client";

import { useState, useEffect, useCallback } from "react";
import { type AgentData } from "@/lib/api";
import { getAgentService, getEnvBundleService } from "@/lib/wasm-core";
import type { AgentCredentialsState, AgentCredentialsActions, CredentialFormData } from "./types";
import type {
  CredentialProfileViewModel,
  CredentialProfilesByAgent,
} from "../_shared/credentialViewModel";

// EnvBundle wire shape from the backend (flat list). The settings page still
// thinks in per-agent groups, so this hook adapts EnvBundle rows into the
// settings-private CredentialProfileViewModel grouped by agent slug.
interface WireEnvBundle {
  id: number;
  agent_slug?: string | null;
  name: string;
  description?: string | null;
  kind: string;
  kind_primary: boolean;
  is_active: boolean;
  configured_fields?: string[];
  configured_values?: Record<string, string>;
  created_at: string;
  updated_at: string;
}

function bundleToProfile(b: WireEnvBundle): CredentialProfileViewModel {
  return {
    id: b.id,
    agent_slug: b.agent_slug ?? "",
    name: b.name,
    description: b.description ?? undefined,
    is_default: b.kind_primary,
    is_active: b.is_active,
    configured_fields: b.configured_fields,
    configured_values: b.configured_values,
    created_at: b.created_at,
    updated_at: b.updated_at,
  };
}

function groupByAgent(bundles: WireEnvBundle[]): CredentialProfilesByAgent[] {
  const groups: Record<string, CredentialProfilesByAgent> = {};
  for (const b of bundles) {
    const slug = b.agent_slug ?? "";
    if (!groups[slug]) {
      groups[slug] = { agent_slug: slug, agent_name: "", profiles: [] };
    }
    groups[slug].profiles.push(bundleToProfile(b));
  }
  return Object.values(groups);
}

export function useAgentCredentials(
  t: (key: string) => string
): AgentCredentialsState & AgentCredentialsActions {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const [profilesByAgent, setProfilesByAgent] = useState<CredentialProfilesByAgent[]>([]);
  const [agents, setAgents] = useState<AgentData[]>([]);
  const [expandedAgents, setExpandedAgents] = useState<Set<string>>(new Set());
  const [agentsWithoutPrimaryBundle, setAgentsWithoutPrimaryBundle] = useState<Set<string>>(new Set());

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const [bundlesRes, agentsRes] = await Promise.all([
        getEnvBundleService().list("credential", "").then((j: string) => JSON.parse(j)),
        getAgentService().list_agents().then((j: string) => JSON.parse(j)),
      ]);

      const grouped = groupByAgent(bundlesRes.items || []);
      setProfilesByAgent(grouped);
      const agentList = [
        ...(agentsRes.builtin_agents || []),
        ...(agentsRes.custom_agents || []),
        ...(agentsRes.agents || []),
      ];
      setAgents(agentList);

      const noPrimarySet = new Set<string>();
      agentList.forEach((a: AgentData) => noPrimarySet.add(a.slug));
      grouped.forEach((g) => {
        if (g.profiles.some((p) => p.is_default)) {
          noPrimarySet.delete(g.agent_slug);
        }
      });
      setAgentsWithoutPrimaryBundle(noPrimarySet);

      const expandedIds = new Set<string>();
      if (agentList.length > 0) expandedIds.add(agentList[0].slug);
      grouped.forEach((g) => {
        if (g.profiles.length > 0) expandedIds.add(g.agent_slug);
      });
      setExpandedAgents(expandedIds);
    } catch (err) {
      console.error("Failed to load env bundles:", err);
      setError(t("settings.agentCredentials.failedToLoad"));
    } finally {
      setLoading(false);
    }
  }, [t]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const toggleAgent = useCallback((agentSlug: string) => {
    setExpandedAgents((prev) => {
      const next = new Set(prev);
      if (next.has(agentSlug)) next.delete(agentSlug);
      else next.add(agentSlug);
      return next;
    });
  }, []);

  const handleClearPrimaryBundle = useCallback(async (agentSlug: string) => {
    try {
      setError(null);
      const group = profilesByAgent.find((g) => g.agent_slug === agentSlug);
      const currentDefault = group?.profiles.find((p) => p.is_default);
      if (currentDefault) {
        await getEnvBundleService().update(
          BigInt(currentDefault.id),
          JSON.stringify({ kind_primary: false })
        );
      }
      setSuccess(t("settings.agentCredentials.defaultSet"));
      await loadData();
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error("Failed to clear primary bundle:", err);
      setError(t("settings.agentCredentials.failedToSetDefault"));
    }
  }, [profilesByAgent, loadData, t]);

  const handleSetDefault = useCallback(async (profileId: number) => {
    try {
      setError(null);
      await getEnvBundleService().set_primary(BigInt(profileId));
      setSuccess(t("settings.agentCredentials.defaultSet"));
      await loadData();
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error("Failed to set primary:", err);
      setError(t("settings.agentCredentials.failedToSetDefault"));
    }
  }, [loadData, t]);

  const handleDelete = useCallback(async (profileId: number) => {
    try {
      setError(null);
      await getEnvBundleService().delete(BigInt(profileId));
      setSuccess(t("settings.agentCredentials.profileDeleted"));
      await loadData();
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error("Failed to delete bundle:", err);
      setError(t("settings.agentCredentials.failedToDelete"));
    }
  }, [loadData, t]);

  const handleSaveProfile = useCallback(async (
    agentSlug: string,
    data: CredentialFormData,
    editingProfile: CredentialProfileViewModel | null
  ) => {
    try {
      if (editingProfile) {
        await getEnvBundleService().update(
          BigInt(editingProfile.id),
          JSON.stringify({
            name: data.name,
            description: data.description || undefined,
            data: Object.keys(data.credentials).length > 0 ? data.credentials : undefined,
          })
        );
        setSuccess(t("settings.agentCredentials.profileUpdated"));
      } else {
        await getEnvBundleService().create(
          JSON.stringify({
            agent_slug: agentSlug,
            name: data.name,
            description: data.description || undefined,
            kind: "credential",
            data: data.credentials,
          })
        );
        setSuccess(t("settings.agentCredentials.profileCreated"));
      }
      await loadData();
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error("Failed to save env bundle:", err);
      throw err;
    }
  }, [loadData, t]);

  const getProfilesForAgent = useCallback((agentSlug: string): CredentialProfileViewModel[] => {
    const group = profilesByAgent.find((g) => g.agent_slug === agentSlug);
    return group?.profiles || [];
  }, [profilesByAgent]);

  return {
    loading, error, success,
    profilesByAgent, agents, expandedAgents, agentsWithoutPrimaryBundle,
    toggleAgent, handleClearPrimaryBundle, handleSetDefault,
    handleDelete, handleSaveProfile, getProfilesForAgent,
    setError, setSuccess,
  };
}
