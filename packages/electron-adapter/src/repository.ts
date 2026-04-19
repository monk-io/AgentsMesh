import { invoke } from "./invoke";
import type { IRepositoryService } from "@agentsmesh/service-interface";

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
}
