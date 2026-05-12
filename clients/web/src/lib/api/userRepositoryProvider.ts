// Connect-RPC adapter for proto.user_credential.v1.UserRepositoryProviderService.
//
// Adapter pattern matches userGitCredential.ts — see that file for the
// rationale. User-scoped service.
//
// PR #329 lineage: RepositoryProvider preserves has_identity, has_bot_token,
// has_client_id, is_active — the fields the legacy serde DTO dropped.

import {
  CreateRepositoryProviderRequestSchema,
  DeleteRepositoryProviderRequestSchema,
  DeleteRepositoryProviderResponseSchema,
  GetRepositoryProviderRequestSchema,
  ListProviderRepositoriesRequestSchema,
  ListProviderRepositoriesResponseSchema,
  ListRepositoryProvidersRequestSchema,
  ListRepositoryProvidersResponseSchema,
  RepositoryProviderSchema,
  SetDefaultRepositoryProviderRequestSchema,
  SetDefaultRepositoryProviderResponseSchema,
  TestRepositoryProviderConnectionRequestSchema,
  TestRepositoryProviderConnectionResponseSchema,
  UpdateRepositoryProviderRequestSchema,
  type ProviderRepository as ProtoProviderRepository,
  type RepositoryProvider as ProtoRepositoryProvider,
} from "@proto/user_credential/v1/user_credential_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getUserCredentialService } from "@/lib/wasm-core";

export interface RepositoryProviderData {
  id: number;
  provider_type: string;
  name: string;
  base_url: string;
  has_client_id: boolean;
  has_bot_token: boolean;
  has_identity: boolean;
  is_default: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface ProviderRepositoryData {
  id: string;
  name: string;
  slug: string;
  description: string;
  default_branch: string;
  visibility: string;
  http_clone_url: string;
  ssh_clone_url: string;
  web_url: string;
}

function fromProto(p: ProtoRepositoryProvider): RepositoryProviderData {
  return {
    id: Number(p.id),
    provider_type: p.providerType,
    name: p.name,
    base_url: p.baseUrl,
    has_client_id: p.hasClientId,
    has_bot_token: p.hasBotToken,
    has_identity: p.hasIdentity,
    is_default: p.isDefault,
    is_active: p.isActive,
    created_at: p.createdAt,
    updated_at: p.updatedAt,
  };
}

function fromProtoRepo(r: ProtoProviderRepository): ProviderRepositoryData {
  return {
    id: r.id,
    name: r.name,
    slug: r.slug,
    description: r.description,
    default_branch: r.defaultBranch,
    visibility: r.visibility,
    http_clone_url: r.httpCloneUrl,
    ssh_clone_url: r.sshCloneUrl,
    web_url: r.webUrl,
  };
}

export async function listRepositoryProviders(): Promise<{
  items: RepositoryProviderData[];
  total: number;
}> {
  const req = create(ListRepositoryProvidersRequestSchema, {});
  const bytes = toBinary(ListRepositoryProvidersRequestSchema, req);
  const respBytes = await getUserCredentialService().listRepositoryProvidersConnect(bytes);
  const resp = fromBinary(ListRepositoryProvidersResponseSchema, new Uint8Array(respBytes));
  return { items: resp.items.map(fromProto), total: Number(resp.total) };
}

export async function getRepositoryProvider(id: number): Promise<RepositoryProviderData> {
  const req = create(GetRepositoryProviderRequestSchema, { id: BigInt(id) });
  const bytes = toBinary(GetRepositoryProviderRequestSchema, req);
  const respBytes = await getUserCredentialService().getRepositoryProviderConnect(bytes);
  return fromProto(fromBinary(RepositoryProviderSchema, new Uint8Array(respBytes)));
}

export interface CreateRepositoryProviderInput {
  provider_type: string;
  name: string;
  base_url: string;
  client_id?: string;
  client_secret?: string;
  bot_token?: string;
}

export async function createRepositoryProvider(input: CreateRepositoryProviderInput): Promise<RepositoryProviderData> {
  const req = create(CreateRepositoryProviderRequestSchema, {
    providerType: input.provider_type,
    name: input.name,
    baseUrl: input.base_url,
    clientId: input.client_id,
    clientSecret: input.client_secret,
    botToken: input.bot_token,
  });
  const bytes = toBinary(CreateRepositoryProviderRequestSchema, req);
  const respBytes = await getUserCredentialService().createRepositoryProviderConnect(bytes);
  return fromProto(fromBinary(RepositoryProviderSchema, new Uint8Array(respBytes)));
}

export interface UpdateRepositoryProviderInput {
  name?: string;
  base_url?: string;
  client_id?: string;
  client_secret?: string;
  bot_token?: string;
  is_active?: boolean;
}

export async function updateRepositoryProvider(id: number, input: UpdateRepositoryProviderInput): Promise<RepositoryProviderData> {
  const req = create(UpdateRepositoryProviderRequestSchema, {
    id: BigInt(id),
    name: input.name,
    baseUrl: input.base_url,
    clientId: input.client_id,
    clientSecret: input.client_secret,
    botToken: input.bot_token,
    isActive: input.is_active,
  });
  const bytes = toBinary(UpdateRepositoryProviderRequestSchema, req);
  const respBytes = await getUserCredentialService().updateRepositoryProviderConnect(bytes);
  return fromProto(fromBinary(RepositoryProviderSchema, new Uint8Array(respBytes)));
}

export async function deleteRepositoryProvider(id: number): Promise<void> {
  const req = create(DeleteRepositoryProviderRequestSchema, { id: BigInt(id) });
  const bytes = toBinary(DeleteRepositoryProviderRequestSchema, req);
  const respBytes = await getUserCredentialService().deleteRepositoryProviderConnect(bytes);
  fromBinary(DeleteRepositoryProviderResponseSchema, new Uint8Array(respBytes));
}

export async function setDefaultRepositoryProvider(id: number): Promise<void> {
  const req = create(SetDefaultRepositoryProviderRequestSchema, { id: BigInt(id) });
  const bytes = toBinary(SetDefaultRepositoryProviderRequestSchema, req);
  const respBytes = await getUserCredentialService().setDefaultRepositoryProviderConnect(bytes);
  fromBinary(SetDefaultRepositoryProviderResponseSchema, new Uint8Array(respBytes));
}

export async function testRepositoryProviderConnection(id: number): Promise<{ success: boolean; message: string }> {
  const req = create(TestRepositoryProviderConnectionRequestSchema, { id: BigInt(id) });
  const bytes = toBinary(TestRepositoryProviderConnectionRequestSchema, req);
  const respBytes = await getUserCredentialService().testRepositoryProviderConnectionConnect(bytes);
  const resp = fromBinary(TestRepositoryProviderConnectionResponseSchema, new Uint8Array(respBytes));
  return { success: resp.success, message: resp.message };
}

export async function listProviderRepositories(opts: {
  id: number; page?: number; per_page?: number; search?: string;
}): Promise<{ items: ProviderRepositoryData[]; total: number }> {
  const req = create(ListProviderRepositoriesRequestSchema, {
    id: BigInt(opts.id),
    page: opts.page,
    perPage: opts.per_page,
    search: opts.search,
  });
  const bytes = toBinary(ListProviderRepositoriesRequestSchema, req);
  const respBytes = await getUserCredentialService().listProviderRepositoriesConnect(bytes);
  const resp = fromBinary(ListProviderRepositoriesResponseSchema, new Uint8Array(respBytes));
  return { items: resp.items.map(fromProtoRepo), total: Number(resp.total) };
}
