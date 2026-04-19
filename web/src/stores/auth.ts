import { create } from "zustand";
import { initWasmCore, getAuthManager, getApiClient } from "@/lib/wasm-core";
import { getErrorMessage } from "@/lib/utils";

interface User {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
}

interface Organization {
  id: number;
  name: string;
  slug: string;
  role?: string;
  logo_url?: string;
  subscription_plan?: string;
  subscription_status?: string;
  created_at?: string;
  updated_at?: string;
}

interface AuthState {
  user: User | null;
  currentOrg: Organization | null;
  organizations: Organization[];
  _hasHydrated: boolean;
  error: string | null;

  login: (email: string, password: string) => Promise<void>;
  fetchOrganizations: () => Promise<void>;
  switchOrg: (slug: string) => void;
  refreshSession: () => Promise<void>;

  setAuth: (token: string, user: User, refreshToken?: string) => void;
  setOrganizations: (orgs: Organization[]) => void;
  setCurrentOrg: (org: Organization) => void;
  logout: () => void;
  isAuthenticated: () => boolean;
  setHasHydrated: (state: boolean) => void;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  currentOrg: null,
  organizations: [],
  _hasHydrated: false,
  error: null,

  login: async (email, password) => {
    await initWasmCore();
    try {
      const mgr = getAuthManager();
      const sessionJson = await mgr.login(email, password);
      const session = JSON.parse(sessionJson);

      const client = getApiClient();
      client.set_token(session.token, session.refresh_token);
      set({ user: session.user, error: null });

      const orgsJson = await mgr.fetch_organizations();
      const orgs: Organization[] = JSON.parse(orgsJson);
      set({ organizations: orgs, currentOrg: orgs[0] || null });
      if (orgs[0]) client.set_org_slug(orgs[0].slug);
    } catch (e) {
      set({ error: getErrorMessage(e, "Login failed") });
      throw e;
    }
  },

  fetchOrganizations: async () => {
    await initWasmCore();
    try {
      const orgsJson = await getAuthManager().fetch_organizations();
      const orgs: Organization[] = JSON.parse(orgsJson);
      set({ organizations: orgs });
      if (!get().currentOrg && orgs.length > 0) {
        set({ currentOrg: orgs[0] });
        getApiClient().set_org_slug(orgs[0].slug);
      }
    } catch (e) {
      set({ error: getErrorMessage(e, "Failed to fetch organizations") });
    }
  },

  switchOrg: (slug) => {
    getAuthManager().switch_org(slug);
    getApiClient().set_org_slug(slug);
    const org = get().organizations.find((o) => o.slug === slug);
    if (org) set({ currentOrg: org });
  },

  refreshSession: async () => {
    await initWasmCore();
    try {
      const tokenJson = await getAuthManager().refresh_token();
      const { token, refresh_token } = JSON.parse(tokenJson);
      getApiClient().set_token(token, refresh_token);
    } catch (e) {
      set({ error: getErrorMessage(e, "Session refresh failed") });
      throw e;
    }
  },

  setAuth: (token, user, refreshToken) => {
    try {
      const client = getApiClient();
      client.set_token(token, refreshToken || "");
    } catch { /* WASM not ready yet */ }
    set({ user, error: null });
  },

  setOrganizations: (organizations) => {
    set({ organizations });
    if (!get().currentOrg && organizations.length > 0) {
      set({ currentOrg: organizations[0] });
      try { getApiClient().set_org_slug(organizations[0].slug); } catch { /* noop */ }
    }
  },

  setCurrentOrg: (org) => {
    set({ currentOrg: org });
    try { getApiClient().set_org_slug(org.slug); } catch { /* noop */ }
  },

  logout: () => {
    try { getAuthManager().logout().catch(() => {}); } catch { /* noop */ }
    try { getApiClient().clear_auth(); } catch { /* noop */ }
    set({ user: null, currentOrg: null, organizations: [], error: null });
  },

  isAuthenticated: () => !!get().user,
  setHasHydrated: (state) => set({ _hasHydrated: state }),
  clearError: () => set({ error: null }),
}));

export type { User, Organization };
