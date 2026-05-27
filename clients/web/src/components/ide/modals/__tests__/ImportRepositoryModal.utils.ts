import { vi } from "vitest";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import {
  ListRepositoryProvidersResponseSchema,
  ListProviderRepositoriesResponseSchema,
  RepositoryProviderSchema,
  ProviderRepositorySchema,
} from "@proto/user_credential/v1/user_credential_pb";
import {
  RepositorySchema,
  CreateRepositoryRequestSchema,
} from "@proto/repository/v1/repository_pb";
import type { RepositoryProviderData, ProviderRepositoryData } from "@/lib/viewModels/repositoryProvider";
import type { RepositoryData } from "@/lib/viewModels/repository";
import { getUserCredentialService, getRepositoryService } from "@/lib/wasm-core";

export const mockProvider: RepositoryProviderData = {
  id: 1,
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

export const createMockOnClose = () => vi.fn<() => void>();
export const createMockOnImported = () => vi.fn<() => void>();

// useImportWizard now drives the Connect-RPC binary lane:
//   - listRepositoryProvidersConnect → list_repo_providers_connect
//   - listProviderRepositoriesConnect → list_provider_repositories_connect
//   - createRepositoryConnect → createRepositoryConnect (camelCase)
// The mocks below resolve proto-encoded Uint8Array — the wizard decodes
// them with @bufbuild/protobuf at runtime. Tests assert on Connect
// invocation + decoded request bodies, not on JSON wire payloads.
export const stableCredSvc = {
  listRepositoryProvidersConnect: vi.fn(),
  listProviderRepositoriesConnect: vi.fn(),
};

export const stableRepoSvc = {
  createRepositoryConnect: vi.fn(),
  // Kept for older tests that assert on `stableRepoSvc.create` directly —
  // the alias points at the same Connect mock so call inspection still works.
  get create() { return stableRepoSvc.createRepositoryConnect; },
};

function encodeProviders(providers: RepositoryProviderData[]): Uint8Array {
  // RepositoryProvider proto doesn't carry `user_id` or `client_id` —
  // those exist only on the snake_case web shape (useImportWizard maps
  // them as optional). Drop them when round-tripping through the wire.
  const items = providers.map((p) =>
    create(RepositoryProviderSchema, {
      id: BigInt(p.id),
      providerType: p.provider_type,
      name: p.name,
      baseUrl: p.base_url,
      hasClientId: p.has_client_id,
      hasBotToken: p.has_bot_token,
      hasIdentity: p.has_identity,
      isDefault: p.is_default,
      isActive: p.is_active,
      createdAt: p.created_at,
      updatedAt: p.updated_at,
    }),
  );
  const resp = create(ListRepositoryProvidersResponseSchema, { items, total: BigInt(items.length) });
  return toBinary(ListRepositoryProvidersResponseSchema, resp);
}

function encodeProviderRepos(repos: ProviderRepositoryData[]): Uint8Array {
  const items = repos.map((r) =>
    create(ProviderRepositorySchema, {
      id: r.id,
      name: r.name,
      slug: r.slug,
      description: r.description ?? "",
      defaultBranch: r.default_branch ?? "main",
      visibility: r.visibility ?? "",
      httpCloneUrl: r.http_clone_url ?? "",
      sshCloneUrl: r.ssh_clone_url ?? "",
      webUrl: r.web_url ?? "",
    }),
  );
  const resp = create(ListProviderRepositoriesResponseSchema, {
    items,
    total: BigInt(items.length),
  });
  return toBinary(ListProviderRepositoriesResponseSchema, resp);
}

function encodeRepository(r: RepositoryData): Uint8Array {
  return toBinary(
    RepositorySchema,
    create(RepositorySchema, {
      id: BigInt(r.id),
      organizationId: BigInt(r.organization_id),
      name: r.name,
      slug: r.slug,
      providerType: r.provider_type,
      providerBaseUrl: r.provider_base_url,
      httpCloneUrl: r.http_clone_url,
      sshCloneUrl: r.ssh_clone_url,
      externalId: r.external_id,
      defaultBranch: r.default_branch ?? "main",
      ticketPrefix: r.ticket_prefix ?? "",
      visibility: r.visibility ?? "organization",
      isActive: r.is_active,
      createdAt: r.created_at,
      updatedAt: r.updated_at,
    }),
  );
}

function applyMocks() {
  vi.mocked(getUserCredentialService).mockReturnValue(stableCredSvc as never);
  vi.mocked(getRepositoryService).mockReturnValue(stableRepoSvc as never);
}

export function setupProviderMocks(providers: RepositoryProviderData[] = [mockProvider, mockGitLabProvider]) {
  applyMocks();
  stableCredSvc.listRepositoryProvidersConnect.mockResolvedValue(encodeProviders(providers));
  stableCredSvc.listProviderRepositoriesConnect.mockResolvedValue(encodeProviderRepos([mockRepository]));
}

export function mockRepositoryCreate(response?: RepositoryData) {
  applyMocks();
  stableRepoSvc.createRepositoryConnect.mockResolvedValue(
    encodeRepository(response ?? mockCreatedRepository),
  );
  return stableRepoSvc;
}

export const createRepositoryResponse = (r: RepositoryData): RepositoryData => r;

// Decode the last `createRepositoryConnect` request payload — Connect
// requests travel as Uint8Array (proto bytes), so the JSON-string
// inspection pattern no longer applies. Tests assert on this decoded
// shape, which carries the same snake_case-ish field names as the
// legacy JSON payload via proto-to-camelCase fields.
export interface CreateRepoCall {
  org_slug: string;
  provider_type: string;
  provider_base_url: string;
  http_clone_url?: string;
  ssh_clone_url?: string;
  external_id: string;
  name: string;
  slug: string;
  default_branch?: string;
  ticket_prefix?: string;
  visibility?: string;
}

export function lastCreateRepoCall(): CreateRepoCall {
  const calls = stableRepoSvc.createRepositoryConnect.mock.calls;
  if (calls.length === 0) throw new Error("createRepositoryConnect not called");
  const req = fromBinary(CreateRepositoryRequestSchema, calls[calls.length - 1][0] as Uint8Array);
  return {
    org_slug: req.orgSlug,
    provider_type: req.providerType,
    provider_base_url: req.providerBaseUrl,
    http_clone_url: req.httpCloneUrl,
    ssh_clone_url: req.sshCloneUrl,
    external_id: req.externalId,
    name: req.name,
    slug: req.slug,
    default_branch: req.defaultBranch,
    ticket_prefix: req.ticketPrefix,
    visibility: req.visibility,
  };
}
