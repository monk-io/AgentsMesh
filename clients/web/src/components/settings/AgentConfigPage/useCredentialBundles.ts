import { useCallback, useState } from "react";
import type { AgentData } from "@/lib/api";
import type { CredentialProfileViewModel } from "../_shared/credentialViewModel";
import { getEnvBundleService } from "@/lib/wasm-core";
import type { CredentialFormData } from "./types";
import type { AgentConfigMessages } from "./useAgentConfigMessages";
import {
  type WireEnvBundle,
  toCredentialProfile,
} from "./envBundleWire";

export interface CredentialBundlesState {
  credentialProfiles: CredentialProfileViewModel[];
  /** True iff no custom bundle is marked primary — the "no bundle" UI row
   *  is highlighted as the effective default in that case. */
  noPrimaryBundle: boolean;
}

export interface CredentialBundlesActions {
  loadCredentialBundles: (agent: AgentData) => Promise<void>;
  handleSaveProfile: (
    data: CredentialFormData,
    editingProfile: CredentialProfileViewModel | null,
    agent: AgentData
  ) => Promise<void>;
  handleSetDefault: (profileId: number) => Promise<void>;
  handleClearPrimaryBundle: () => Promise<void>;
  handleDeleteProfile: (profileId: number) => Promise<void>;
}

/**
 * Owns the credential-kind EnvBundle slice of the agent config page.
 *
 * The agent itself is supplied per-action rather than stored in this hook
 * — the parent `useAgentConfig` owns the agent identity and re-invokes
 * `loadCredentialBundles` whenever it changes.
 */
export function useCredentialBundles(
  msgs: AgentConfigMessages,
  t: (key: string) => string
): CredentialBundlesState & CredentialBundlesActions {
  const [credentialProfiles, setCredentialProfiles] = useState<CredentialProfileViewModel[]>([]);
  const [noPrimaryBundle, setNoPrimaryBundle] = useState(true);

  const loadCredentialBundles = useCallback(async (agent: AgentData) => {
    try {
      const res = await getEnvBundleService()
        .list("credential", agent.slug)
        .then((j: string) => JSON.parse(j))
        .catch(() => ({ items: [] }));
      const bundles: WireEnvBundle[] = res.items ?? [];
      const profiles = bundles.map((b) => toCredentialProfile(b, agent.slug));
      setCredentialProfiles(profiles);
      setNoPrimaryBundle(!profiles.some((p) => p.is_default));
    } catch (err) {
      // The list endpoint already short-circuits to {items: []} on failure;
      // anything reaching here is a programming error worth surfacing.
      msgs.reportError(err, t, "settings.agentConfig.loadFailed");
    }
  }, [msgs, t]);

  const handleClearPrimaryBundle = useCallback(async () => {
    try {
      msgs.setError(null);
      const currentDefault = credentialProfiles.find((p) => p.is_default);
      if (currentDefault) {
        await getEnvBundleService().update(
          BigInt(currentDefault.id),
          JSON.stringify({ kind_primary: false })
        );
      }
      // Reflect locally so the UI updates before the next loadAll.
      setCredentialProfiles((prev) =>
        prev.map((p) => (p.is_default ? { ...p, is_default: false } : p))
      );
      setNoPrimaryBundle(true);
      msgs.reportSuccess(t("settings.agentCredentials.defaultSet"));
    } catch (err) {
      msgs.reportError(err, t, "settings.agentCredentials.failedToSetDefault");
    }
  }, [credentialProfiles, msgs, t]);

  const handleSetDefault = useCallback(async (profileId: number) => {
    try {
      msgs.setError(null);
      await getEnvBundleService().set_primary(BigInt(profileId));
      setCredentialProfiles((prev) =>
        prev.map((p) => ({ ...p, is_default: p.id === profileId }))
      );
      setNoPrimaryBundle(false);
      msgs.reportSuccess(t("settings.agentCredentials.defaultSet"));
    } catch (err) {
      msgs.reportError(err, t, "settings.agentCredentials.failedToSetDefault");
    }
  }, [msgs, t]);

  const handleDeleteProfile = useCallback(async (profileId: number) => {
    try {
      msgs.setError(null);
      await getEnvBundleService().delete(BigInt(profileId));
      setCredentialProfiles((prev) => {
        const next = prev.filter((p) => p.id !== profileId);
        setNoPrimaryBundle(!next.some((p) => p.is_default));
        return next;
      });
      msgs.reportSuccess(t("settings.agentCredentials.profileDeleted"));
    } catch (err) {
      msgs.reportError(err, t, "settings.agentCredentials.failedToDelete");
    }
  }, [msgs, t]);

  // Credential values are encrypted server-side under kind='credential'; the
  // form passes them as a plain { ENV_NAME: value } map under the bundle's
  // `data` field, identical to a fresh create payload.
  const handleSaveProfile = useCallback(
    async (
      data: CredentialFormData,
      editingProfile: CredentialProfileViewModel | null,
      agent: AgentData
    ) => {
      if (editingProfile) {
        await getEnvBundleService().update(
          BigInt(editingProfile.id),
          JSON.stringify({
            name: data.name,
            description: data.description || undefined,
            data: Object.keys(data.credentials).length > 0 ? data.credentials : undefined,
          })
        );
        msgs.reportSuccess(t("settings.agentCredentials.profileUpdated"));
      } else {
        await getEnvBundleService().create(
          JSON.stringify({
            agent_slug: agent.slug,
            name: data.name,
            description: data.description || undefined,
            kind: "credential",
            data: data.credentials,
          })
        );
        msgs.reportSuccess(t("settings.agentCredentials.profileCreated"));
      }
      await loadCredentialBundles(agent);
    },
    [loadCredentialBundles, msgs, t]
  );

  return {
    credentialProfiles,
    noPrimaryBundle,
    loadCredentialBundles,
    handleSaveProfile,
    handleSetDefault,
    handleClearPrimaryBundle,
    handleDeleteProfile,
  };
}
