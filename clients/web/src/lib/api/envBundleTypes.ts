/**
 * Frontend type for EnvBundle (the backend's named, owner-scoped KV map
 * that AgentFile references via `USE_ENV_BUNDLE "name"`). Mirrors the
 * Rust `EnvBundle` wire shape so the wasm getters round-trip cleanly.
 *
 * `kind` is a free-form string (no enum constraint) — code defines the
 * recognized values: `credential`, `runtime`, `shared`.
 *
 * For `credential` kind the backend strips values from the wire (only
 * `configured_fields` is populated). For non-secret kinds values come
 * back through `configured_values`.
 */
export interface EnvBundle {
  id: number;
  owner_scope: string;
  owner_id: number;
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
 * Compact projection used by CreatePodForm and lists where only the
 * picker-relevant subset matters (name + primary hint).
 */
export interface EnvBundleSummary {
  id: number;
  name: string;
  agent_slug?: string | null;
  kind: string;
  kind_primary: boolean;
  configured_fields?: string[];
}

export interface EnvBundleListResponse {
  items: EnvBundle[];
}

export interface CreateEnvBundleRequest {
  agent_slug?: string | null;
  name: string;
  description?: string | null;
  kind: string;
  kind_primary?: boolean;
  data: Record<string, string>;
}

export interface UpdateEnvBundleRequest {
  name?: string;
  description?: string | null;
  kind?: string;
  kind_primary?: boolean;
  data?: Record<string, string>;
  is_active?: boolean;
}
