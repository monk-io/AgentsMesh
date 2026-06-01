import { create } from "zustand";
import { useMemo } from "react";
import { create as protoCreate, toBinary } from "@bufbuild/protobuf";
import { getRepoState } from "@/lib/wasm-core";
import { getErrorMessage } from "@/lib/utils";
import { repositoryApi } from "@/lib/api/facade/repository";
import { repositoryToProto } from "@/lib/api/repoProtoMap";
import type { RepositoryData } from "@/lib/viewModels/repository";
import {
  ReplaceCachedRepositoriesRequestSchema,
} from "@proto/repo_state/v1/repo_state_pb";

export type Repository = RepositoryData;

interface RepositoryState {
  _tick: number;
  isLoading: boolean;
  fetched: boolean;
  error: string | null;
  fetchRepositories: () => Promise<void>;
  deleteRepository: (id: number) => Promise<void>;
}

const rs = () => getRepoState();
const bump = () => useRepositoryStore.setState((s) => ({ _tick: s._tick + 1 }));

export function useRepositories(): Repository[] {
  const tick = useRepositoryStore((s) => s._tick);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  return useMemo(() => JSON.parse(rs().repositories_json()), [tick]);
}

function dispatchReplaceCachedRepositories(items: RepositoryData[]): void {
  const req = protoCreate(ReplaceCachedRepositoriesRequestSchema, {
    repositories: items.map(repositoryToProto),
  });
  rs().replace_cached_repositories(toBinary(ReplaceCachedRepositoriesRequestSchema, req));
}

export const useRepositoryStore = create<RepositoryState>((set) => ({
  _tick: 0,
  isLoading: false,
  fetched: false,
  error: null,

  fetchRepositories: async () => {
    set({ isLoading: true, error: null });
    try {
      const { items } = await repositoryApi.list();
      dispatchReplaceCachedRepositories(items);
      bump();
      set({ isLoading: false, fetched: true });
    } catch (e) {
      set({ isLoading: false, error: getErrorMessage(e, "Failed to fetch repositories") });
    }
  },

  deleteRepository: async (id: number) => {
    await repositoryApi.delete(id);
    rs().remove_repository(String(id));
    bump();
  },
}));
