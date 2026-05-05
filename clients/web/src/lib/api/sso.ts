import { getSSOService } from "@/lib/wasm-core";
export type { SSOConfig } from "./ssoTypes";

export const ssoApi = {
  discover: async (email: string) => {
    const raw = await getSSOService().discover(email);
    if (!raw || raw === "undefined") return { configs: [] };
    return JSON.parse(raw);
  },
  ldapAuth: async (domain: string, data: { username: string; password: string }) => {
    const json = await getSSOService().ldap_auth(domain, JSON.stringify(data));
    return JSON.parse(json);
  },
};

export function getSSOAuthURL(config: { protocol: string; domain: string; provider_url?: string }, redirectUrl?: string): string {
  const base = config.provider_url || "";
  return redirectUrl ? `${base}?redirect=${encodeURIComponent(redirectUrl)}` : base;
}
