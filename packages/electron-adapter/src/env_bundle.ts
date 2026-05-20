import { invoke } from "./invoke";

// Desktop counterpart of WasmEnvBundleService. Wraps node-bridge napi methods
// (`env_bundle_*` → camelCase `envBundle*` IPC channels) and presents the same
// surface the renderer reaches via `getEnvBundleService()`. Mirrors the call
// shape the web tree already calls — `list(kind, agentSlug)` returning the
// raw JSON string the backend produces, etc.
export class ElectronEnvBundleService {
  async list(kind: string, agentSlug: string): Promise<string> {
    return invoke<string>("envBundleList", kind || null, agentSlug || null);
  }

  async get(id: bigint): Promise<string> {
    return invoke<string>("envBundleGet", Number(id));
  }

  async create(json: string): Promise<string> {
    return invoke<string>("envBundleCreate", json);
  }

  async update(id: bigint, json: string): Promise<string> {
    return invoke<string>("envBundleUpdate", Number(id), json);
  }

  async delete(id: bigint): Promise<void> {
    await invoke<void>("envBundleDelete", Number(id));
  }

  async set_primary(id: bigint): Promise<string> {
    return invoke<string>("envBundleSetPrimary", Number(id));
  }
}
