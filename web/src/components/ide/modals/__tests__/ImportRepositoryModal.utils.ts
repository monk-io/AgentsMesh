import { vi } from "vitest";
import type { RepositoryProviderData, ProviderRepositoryData } from "@/lib/api/userRepositoryProviderTypes";
import type { RepositoryData } from "@/lib/api/repositoryTypes";
import { getUserCredentialService, getRepositoryService } from "@/lib/wasm-core";

export const mockProvider: RepositoryProviderData = {
  id: 1,
  user_id: 1,
  name: "My GitHub",
  provider_type: "github",
  base_url: "https://github.com",
  has_client_id: false,
  has_bot_token: false,
  has_identity: true,
  is_default: true,
  is_active: true,
  created_at: "2024-01-01T00:00:00Z",
  updated_at: "2024-01-01T00:00:00Z",
};

export const mockGitLabProvider: RepositoryProviderData = {
  id: 2,
  user_id: 1,
  name: "My GitLab",
  provider_type: "gitlab",
  base_url: "https://gitlab.com",
  has_client_id: false,
  has_bot_token: true,
  has_identity: false,
  is_default: false,
  is_active: true,
  created_at: "2024-01-01T00:00:00Z",
  updated_at: "2024-01-01T00:00:00Z",
};

export const mockRepository: ProviderRepositoryData = {
  id: "repo-1",
  name: "my-project",
  slug: "org/my-project",
  description: "A test project",
  default_branch: "main",
  visibility: "private",
  http_clone_url: "https://github.com/org/my-project.git",
  ssh_clone_url: "git@github.com:org/my-project.git",
  web_url: "https://github.com/org/my-project",
};

export const mockCreatedRepository: RepositoryData = {
  id: 1,
  organization_id: 1,
  name: "my-project",
  slug: "org/my-project",
  provider_type: "github",
  provider_base_url: "https://github.com",
  http_clone_url: "https://github.com/org/my-project.git",
  external_id: "repo-1",
  default_branch: "main",
  visibility: "organization",
  is_active: true,
  created_at: "2024-01-01T00:00:00Z",
  updated_at: "2024-01-01T00:00:00Z",
};

export const createMockOnClose = () => vi.fn();
export const createMockOnImported = () => vi.fn();

export const createListRepositoriesResponse = (repositories: ProviderRepositoryData[] = [mockRepository]) =>
  JSON.stringify({ repositories, page: 1, per_page: 20 });

export const createRepositoryResponse = (repository: RepositoryData = mockCreatedRepository) =>
  JSON.stringify({ repository });

const stableCredSvc = {
  list_repo_providers: vi.fn().mockResolvedValue('{"providers":[]}'),
  list_provider_repositories: vi.fn().mockResolvedValue('{"repositories":[]}'),
  list_git_credentials: vi.fn().mockResolvedValue('{"credentials":[]}'),
  create_git_credential: vi.fn().mockResolvedValue('{}'),
  get_git_credential: vi.fn().mockResolvedValue('{}'),
  update_git_credential: vi.fn().mockResolvedValue('{}'),
  delete_git_credential: vi.fn().mockResolvedValue(undefined),
  get_default_git_credential: vi.fn().mockResolvedValue('{}'),
  set_default_git_credential: vi.fn().mockResolvedValue(undefined),
  clear_default_git_credential: vi.fn().mockResolvedValue(undefined),
  list_agent_credentials: vi.fn().mockResolvedValue('{"credentials":[]}'),
  list_agent_credentials_for_agent: vi.fn().mockResolvedValue('{"credentials":[]}'),
  create_agent_credential: vi.fn().mockResolvedValue('{}'),
  get_agent_credential: vi.fn().mockResolvedValue('{}'),
  update_agent_credential: vi.fn().mockResolvedValue('{}'),
  delete_agent_credential: vi.fn().mockResolvedValue(undefined),
  set_default_agent_credential: vi.fn().mockResolvedValue(undefined),
  create_repo_provider: vi.fn().mockResolvedValue('{}'),
  get_repo_provider: vi.fn().mockResolvedValue('{}'),
  update_repo_provider: vi.fn().mockResolvedValue('{}'),
  delete_repo_provider: vi.fn().mockResolvedValue(undefined),
  set_default_repo_provider: vi.fn().mockResolvedValue(undefined),
  test_repo_provider: vi.fn().mockResolvedValue(undefined),
};

const stableRepoSvc = {
  list: vi.fn().mockResolvedValue('{"repositories":[]}'),
  get: vi.fn().mockResolvedValue('{}'),
  create: vi.fn().mockResolvedValue('{}'),
  update: vi.fn().mockResolvedValue('{}'),
  delete: vi.fn().mockResolvedValue(undefined),
  list_branches: vi.fn().mockResolvedValue('{"branches":[]}'),
  sync_branches: vi.fn().mockResolvedValue('{"branches":[]}'),
  register_webhook: vi.fn().mockResolvedValue(undefined),
  delete_webhook: vi.fn().mockResolvedValue(undefined),
  get_webhook_status: vi.fn().mockResolvedValue('{}'),
  get_webhook_secret: vi.fn().mockResolvedValue('{}'),
  list_merge_requests: vi.fn().mockResolvedValue('{"merge_requests":[]}'),
};

export function setupProviderMocks(providers = [mockProvider, mockGitLabProvider]) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  vi.mocked(getUserCredentialService).mockReturnValue(stableCredSvc as any);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  vi.mocked(getRepositoryService).mockReturnValue(stableRepoSvc as any);

  stableCredSvc.list_repo_providers.mockResolvedValue(
    JSON.stringify({ providers }),
  );
  stableCredSvc.list_provider_repositories.mockResolvedValue(
    createListRepositoriesResponse(),
  );
}

export function mockRepositoryCreate(response?: string) {
  stableRepoSvc.create.mockResolvedValue(response ?? createRepositoryResponse());
  return stableRepoSvc;
}

export { stableCredSvc, stableRepoSvc };
