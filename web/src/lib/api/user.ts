import { getUserApiService, getApiClient } from "@/lib/wasm-core";

export interface UserSummary {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
}

export const userApi = {
  getMe: async () => {
    const json = await getUserApiService().get_me();
    return JSON.parse(json);
  },
  getOrganizations: async () => {
    const json = await getUserApiService().get_organizations();
    return JSON.parse(json) as { organizations: Array<{ id: number; name: string; slug: string; role: string }> };
  },
  // TODO(wasm): add a dedicated search_users method to UserApiService once the core
  // crate gains full-text user search. For now delegate to the shared ApiClient.
  search: async (q: string, limit = 10): Promise<{ users: UserSummary[] }> => {
    const query = `q=${encodeURIComponent(q)}&limit=${limit}`;
    const json = await getApiClient().get(`/api/v1/users/search?${query}`);
    return typeof json === "string" ? JSON.parse(json) : json;
  },
};
