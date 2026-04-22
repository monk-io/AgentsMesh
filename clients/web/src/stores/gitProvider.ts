import { create } from "zustand";
import { useMemo } from "react";
import { getGitProviderState, parseWasmAny } from "@/lib/wasm-core";

export interface GitProvider {
  id: number; organization_id: number;
  provider_type: "gitlab" | "github" | "gitee";
  name: string; base_url: string; is_default: boolean; is_active: boolean;
  created_at: string; updated_at: string;
}

export interface GitProviderProject {
  id: string; name: string; slug: string; default_branch: string;
  web_url: string; description?: string;
}

interface GitProviderState {
  _tick: number; isLoading: boolean; isSyncing: boolean; error: string | null;
  setProviders: (providers: GitProvider[]) => void;
  setCurrentProvider: (provider: GitProvider | null) => void;
  addProvider: (provider: GitProvider) => void;
  updateProvider: (id: number, updates: Partial<GitProvider>) => void;
  removeProvider: (id: number) => void;
  setAvailableProjects: (projects: GitProviderProject[]) => void;
  setLoading: (loading: boolean) => void;
  setSyncing: (syncing: boolean) => void;
  setError: (error: string | null) => void;
  reset: () => void;
}

const ws = () => getGitProviderState();
const bump = () => useGitProviderStore.setState((s) => ({ _tick: s._tick + 1 }));

export function useGitProviders(): GitProvider[] {
  const tick = useGitProviderStore((s) => s._tick);
  return useMemo(() => JSON.parse(ws().providers_json()), [tick]);
}

export function useCurrentGitProvider(): GitProvider | null {
  const tick = useGitProviderStore((s) => s._tick);
  return useMemo(() => parseWasmAny<GitProvider>(ws().current_provider_json()), [tick]);
}

export function useAvailableProjects(): GitProviderProject[] {
  const tick = useGitProviderStore((s) => s._tick);
  return useMemo(() => JSON.parse(ws().available_projects_json()), [tick]);
}

export const useGitProviderStore = create<GitProviderState>((set) => ({
  _tick: 0, isLoading: false, isSyncing: false, error: null,

  setProviders: (providers) => { ws().set_providers(JSON.stringify(providers)); bump(); },
  setCurrentProvider: (provider) => { ws().set_current_provider(provider ? JSON.stringify(provider) : ""); bump(); },
  addProvider: (provider) => { ws().add_provider(JSON.stringify(provider)); bump(); },

  updateProvider: (id, updates) => {
    const providers: GitProvider[] = JSON.parse(ws().providers_json());
    const existing = providers.find((p) => p.id === id);
    if (existing) ws().update_provider(String(id), JSON.stringify({ ...existing, ...updates }));
    bump();
  },

  removeProvider: (id) => { ws().remove_provider(String(id)); bump(); },
  setAvailableProjects: (projects) => { ws().set_available_projects(JSON.stringify(projects)); bump(); },
  setLoading: (isLoading) => set({ isLoading }),
  setSyncing: (isSyncing) => set({ isSyncing }),
  setError: (error) => set({ error }),

  reset: () => {
    ws().set_providers("[]"); ws().set_current_provider(""); ws().set_available_projects("[]");
    set({ _tick: 0, isLoading: false, isSyncing: false, error: null });
  },
}));
