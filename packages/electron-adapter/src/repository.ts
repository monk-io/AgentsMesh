import { invoke } from "./invoke";
import type { IRepositoryService } from "@agentsmesh/service-interface";

// Web's wasm-side `WasmRepositoryService` exposes `<verb>Connect(bytes)`
// methods. The hand-written IPC handlers on the Rust napi side only
// cover the legacy json-shaped surface (`repositoryList` /
// `repositoryGet` etc), not the proto wire. Renderers that go through
// `lib/api/repositoryConnect.ts` need the proto-binary entry points
// too, so we forward them through the generic `connectCall` IPC handler
// in main/index.ts (registerLegacyApiAliases) — main owns the auth
// header injection, the URL prefix, and the binary-over-IPC marshalling.
async function connectCall(method: string, request: Uint8Array): Promise<Uint8Array> {
  const resp = await invoke<number[] | Uint8Array>(
    "connectCall",
    "proto.repository.v1.RepositoryService",
    method,
    Array.from(request),
  );
  return resp instanceof Uint8Array ? resp : new Uint8Array(resp);
}

export class ElectronRepositoryService implements IRepositoryService {
  async list(): Promise<string> {
    return invoke<string>("repositoryList");
  }

  async get(id: bigint): Promise<string> {
    return invoke<string>("repositoryGet", Number(id));
  }

  async create(json: string): Promise<string> {
    return invoke<string>("repositoryCreate", json);
  }

  async update(id: bigint, json: string): Promise<string> {
    return invoke<string>("repositoryUpdate", Number(id), json);
  }

  async delete(id: bigint): Promise<void> {
    await invoke<void>("repositoryDelete", Number(id));
  }

  async list_branches(id: bigint): Promise<string> {
    return invoke<string>("repositoryListBranches", Number(id));
  }

  async sync_branches(id: bigint, json: string): Promise<string> {
    return invoke<string>("repositorySyncBranches", Number(id), json);
  }

  async list_merge_requests(id: bigint, branch?: string | null, state?: string | null): Promise<string> {
    return invoke<string>("repositoryListMergeRequests", Number(id), branch, state);
  }

  async register_webhook(id: bigint): Promise<void> {
    await invoke<void>("repositoryRegisterWebhook", Number(id));
  }

  async delete_webhook(id: bigint): Promise<void> {
    await invoke<void>("repositoryDeleteWebhook", Number(id));
  }

  async get_webhook_secret(id: bigint): Promise<string> {
    return invoke<string>("repositoryGetWebhookSecret", Number(id));
  }

  async get_webhook_status(id: bigint): Promise<string> {
    return invoke<string>("repositoryGetWebhookStatus", Number(id));
  }

  async mark_webhook_configured(id: bigint): Promise<void> {
    await invoke<void>("repositoryMarkWebhookConfigured", Number(id));
  }

  // ── Connect-RPC binary surface (mirrors WasmRepositoryService) ──

  listRepositoriesConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("ListRepositories", request);
  }
  getRepositoryConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("GetRepository", request);
  }
  createRepositoryConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("CreateRepository", request);
  }
  updateRepositoryConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("UpdateRepository", request);
  }
  deleteRepositoryConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("DeleteRepository", request);
  }
  listRepositoryBranchesConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("ListRepositoryBranches", request);
  }
  syncRepositoryBranchesConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("SyncRepositoryBranches", request);
  }
  listRepositoryMergeRequestsConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("ListRepositoryMergeRequests", request);
  }
  registerRepositoryWebhookConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("RegisterRepositoryWebhook", request);
  }
  deleteRepositoryWebhookConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("DeleteRepositoryWebhook", request);
  }
  getRepositoryWebhookStatusConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("GetRepositoryWebhookStatus", request);
  }
  getRepositoryWebhookSecretConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("GetRepositoryWebhookSecret", request);
  }
  markRepositoryWebhookConfiguredConnect(request: Uint8Array): Promise<Uint8Array> {
    return connectCall("MarkRepositoryWebhookConfigured", request);
  }
}
