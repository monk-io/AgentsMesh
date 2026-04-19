import { create } from "zustand";
import { useMemo } from "react";
import { getRepoState, parseWasmAny } from "@/lib/wasm-core";

export interface Repository {
  id: number; organization_id: number; git_provider_id: number; external_id: string;
  name: string; slug: string; default_branch: string; ticket_prefix?: string;
  is_active: boolean; created_at: string; updated_at: string;
  git_provider_name?: string; git_provider_type?: string;
}

export interface Branch {
  name: string; commit_sha: string; is_default: boolean; is_protected: boolean;
}

interface RepositoryState {
  _tick: number; isLoading: boolean; error: string | null;
  setRepositories: (repos: Repository[]) => void;
  setCurrentRepository: (repo: Repository | null) => void;
  addRepository: (repo: Repository) => void;
  updateRepository: (id: number, updates: Partial<Repository>) => void;
  removeRepository: (id: number) => void;
  setBranches: (branches: Branch[]) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  reset: () => void;
  getRepositoriesByProvider: (providerId: number) => Repository[];
}

const rs = () => getRepoState();
const bump = () => useRepositoryStore.setState((s) => ({ _tick: s._tick + 1 }));

export function useRepositories(): Repository[] {
  const tick = useRepositoryStore((s) => s._tick);
  return useMemo(() => JSON.parse(rs().repositories_json()), [tick]);
}

export function useCurrentRepository(): Repository | null {
  const tick = useRepositoryStore((s) => s._tick);
  return useMemo(() => parseWasmAny<Repository>(rs().current_repo_json()), [tick]);
}

export function useBranches(): Branch[] {
  const tick = useRepositoryStore((s) => s._tick);
  return useMemo(() => JSON.parse(rs().branches_json()), [tick]);
}

export const useRepositoryStore = create<RepositoryState>((set) => ({
  _tick: 0, isLoading: false, error: null,

  setRepositories: (repos) => { rs().set_repositories(JSON.stringify(repos)); bump(); },
  setCurrentRepository: (repo) => { rs().set_current_repo(repo ? JSON.stringify(repo) : ""); bump(); },
  addRepository: (repo) => { rs().add_repository(JSON.stringify(repo)); bump(); },

  updateRepository: (id, updates) => {
    const repos: Repository[] = JSON.parse(rs().repositories_json());
    const existing = repos.find((r) => r.id === id);
    if (existing) rs().update_repository(String(id), JSON.stringify({ ...existing, ...updates }));
    bump();
  },

  removeRepository: (id) => { rs().remove_repository(String(id)); bump(); },
  setBranches: (branches) => { rs().set_branches(JSON.stringify(branches)); bump(); },
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),

  reset: () => {
    rs().set_repositories("[]"); rs().set_current_repo(""); rs().set_branches("[]");
    set({ _tick: 0, isLoading: false, error: null });
  },

  getRepositoriesByProvider: (providerId) => {
    const repos: Repository[] = JSON.parse(rs().repositories_json());
    return repos.filter((r) => r.git_provider_id === providerId);
  },
}));
