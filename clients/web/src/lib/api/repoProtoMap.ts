// RepositoryData (web snake_case view-model) ↔ proto.repository.v1.Repository
// Mirror of autopilotProtoMap.ts / runnerProtoMap.ts — denormalises renderer
// view-model types into the proto repository schema for binary-bytes dispatch
// to the wasm bridge.
//
// Used by stores/repository.ts to encode mutation requests.

import { create as protoCreate } from "@bufbuild/protobuf";
import {
  RepositorySchema,
  RepositoryWebhookConfigSchema,
  type Repository,
} from "@proto/repository/v1/repository_pb";
import type { RepositoryData } from "@/lib/viewModels/repository";

export function repositoryToProto(r: RepositoryData): Repository {
  return protoCreate(RepositorySchema, {
    id: BigInt(r.id),
    organizationId: BigInt(r.organization_id),
    providerType: r.provider_type,
    providerBaseUrl: r.provider_base_url,
    httpCloneUrl: r.http_clone_url ?? "",
    sshCloneUrl: r.ssh_clone_url ?? "",
    externalId: r.external_id,
    name: r.name,
    slug: r.slug,
    defaultBranch: r.default_branch,
    ticketPrefix: r.ticket_prefix,
    visibility: r.visibility,
    importedByUserId:
      r.imported_by_user_id === undefined ? undefined : BigInt(r.imported_by_user_id),
    isActive: r.is_active,
    webhookConfig: r.webhook_config
      ? protoCreate(RepositoryWebhookConfigSchema, {
          id: r.webhook_config.id,
          url: r.webhook_config.url,
          events: r.webhook_config.events ?? [],
          isActive: r.webhook_config.is_active,
          needsManualSetup: r.webhook_config.needs_manual_setup,
          lastError: r.webhook_config.last_error,
          createdAt: r.webhook_config.created_at,
        })
      : undefined,
    createdAt: r.created_at,
    updatedAt: r.updated_at,
  });
}
