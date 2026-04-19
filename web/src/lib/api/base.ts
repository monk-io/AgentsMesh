/**
 * Legacy HTTP client adapters — thin wrappers over the WASM ApiClient.
 *
 * New code should prefer dedicated WASM services (e.g. `getChannelService`),
 * but several modules (blockstore, grant) still use the old `request` / `orgPath`
 * helpers. Keep these adapters until those callers are migrated too.
 */

import { getApiClient } from "@/lib/wasm-core";

export { ApiError } from "./api-types";

type RequestMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

interface RequestOptions {
  method?: RequestMethod;
  body?: unknown;
}

/** Prefix a path with the active org slug (e.g. `/channels` → `/api/v1/orgs/:slug/channels`). */
export function orgPath(path: string): string {
  return getApiClient().org_path(path);
}

export async function request<T = unknown>(endpoint: string, opts: RequestOptions = {}): Promise<T> {
  const client = getApiClient();
  const body = opts.body === undefined ? "" : JSON.stringify(opts.body);
  let raw: string;
  switch (opts.method) {
    case "POST":
      raw = await client.post(endpoint, body);
      break;
    case "PUT":
      raw = await client.put(endpoint, body);
      break;
    case "PATCH":
      raw = await client.patch(endpoint, body);
      break;
    case "DELETE":
      raw = await client.delete(endpoint);
      break;
    default:
      raw = await client.get(endpoint);
  }
  if (!raw) return undefined as T;
  try { return JSON.parse(raw) as T; } catch { return raw as unknown as T; }
}
