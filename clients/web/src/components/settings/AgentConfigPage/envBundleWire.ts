import type { CredentialProfileViewModel } from "../_shared/credentialViewModel";
import type { RuntimeBundleViewModel } from "./types";

/**
 * Wire shape of an EnvBundle row as returned by the backend list endpoint.
 * Both `credential` and `runtime` kinds share this shape; the consumer
 * decides how to project it.
 *
 * `configured_values` is only populated for plaintext kinds (runtime,
 * eventually shared). For credential kind the backend strips values and
 * leaves the map nil — only `configured_fields` echoes the key names.
 */
export interface WireEnvBundle {
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

/**
 * Project a wire EnvBundle into the CredentialProfileViewModel shape that
 * `CredentialsSection` consumes. The view model predates the EnvBundle
 * refactor; this adapter keeps the legacy component untouched while wire
 * shapes migrated under the hood.
 */
export function toCredentialProfile(
  b: WireEnvBundle,
  fallbackAgentSlug: string
): CredentialProfileViewModel {
  return {
    id: b.id,
    agent_slug: b.agent_slug ?? fallbackAgentSlug,
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

/**
 * Project a wire EnvBundle into a RuntimeBundleViewModel. Runtime values
 * round-trip in plaintext, so `configured_values` survives the projection
 * intact — the section UI shows the raw KV pairs.
 */
export function toRuntimeBundle(
  b: WireEnvBundle,
  fallbackAgentSlug: string
): RuntimeBundleViewModel {
  return {
    id: b.id,
    agent_slug: b.agent_slug ?? fallbackAgentSlug,
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
