import { request } from "./base";

export interface UserSummary {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
}

// User API
export const userApi = {
  getMe: () =>
    request<{ user: UserSummary }>("/api/v1/users/me"),

  getOrganizations: () =>
    request<{ organizations: Array<{ id: number; name: string; slug: string; role: string }> }>(
      "/api/v1/users/me/organizations"
    ),

  search: (q: string, limit = 10) =>
    request<{ users: UserSummary[] }>(
      `/api/v1/users/search?q=${encodeURIComponent(q)}&limit=${limit}`
    ),
};
