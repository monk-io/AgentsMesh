import { getRepositoryService } from "@/lib/wasm-core";
export type { RepositoryData, CreateRepositoryRequest, UpdateRepositoryRequest, WebhookStatus, WebhookResult, WebhookSecretResponse } from "./repositoryTypes";

export const repositoryApi = {
  list: async () => {
    const json = await getRepositoryService().list();
    return JSON.parse(json);
  },
  get: async (id: number) => {
    const json = await getRepositoryService().get(BigInt(id));
    return JSON.parse(json);
  },
  create: async (data: Record<string, unknown>) => {
    const json = await getRepositoryService().create(JSON.stringify(data));
    return JSON.parse(json);
  },
  update: async (id: number, data: Record<string, unknown>) => {
    const json = await getRepositoryService().update(BigInt(id), JSON.stringify(data));
    return JSON.parse(json);
  },
  delete: async (id: number) => {
    await getRepositoryService().delete(BigInt(id));
  },
  getWebhookStatus: async (id: number) => {
    const json = await getRepositoryService().get_webhook_status(BigInt(id));
    return JSON.parse(json);
  },
  registerWebhook: async (id: number) => {
    await getRepositoryService().register_webhook(BigInt(id));
  },
  getWebhookSecret: async (id: number) => {
    const json = await getRepositoryService().get_webhook_secret(BigInt(id));
    return JSON.parse(json);
  },
  deleteWebhook: async (id: number) => {
    await getRepositoryService().delete_webhook(BigInt(id));
  },
  markWebhookConfigured: async (id: number) => {
    await getRepositoryService().mark_webhook_configured(BigInt(id));
  },
  listMergeRequests: async (repoId: number, branch?: string) => {
    const json = await getRepositoryService().list_merge_requests(BigInt(repoId), branch ?? null);
    return JSON.parse(json);
  },
};
