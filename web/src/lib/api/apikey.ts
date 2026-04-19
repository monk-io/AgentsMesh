import { getApiKeyService } from "@/lib/wasm-core";
export type { APIKeyData, CreateAPIKeyRequest, UpdateAPIKeyRequest } from "./apikeyTypes";

export const apiKeyApi = {
  list: async () => {
    const json = await getApiKeyService().list();
    return JSON.parse(json);
  },
  create: async (data: { name: string; scopes?: string[] }) => {
    const json = await getApiKeyService().create(JSON.stringify(data));
    return JSON.parse(json);
  },
  update: async (id: number, data: Record<string, unknown>) => {
    const json = await getApiKeyService().update(BigInt(id), JSON.stringify(data));
    return JSON.parse(json);
  },
  revoke: async (id: number) => {
    await getApiKeyService().revoke(BigInt(id));
  },
};
