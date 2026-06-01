// repoProtoMap converter test — RepositoryData (snake_case view-model) →
// proto.repository.v1.Repository (camelCase + BigInt). Verifies field-name
// translation, the BigInt boundary on int64 ids, and webhook_config
// nested-message round-trip.

import { describe, it, expect } from "vitest";
import { repositoryToProto } from "../repoProtoMap";
import type { RepositoryData } from "@/lib/viewModels/repository";

const fixture: RepositoryData = {
  id: 7,
  organization_id: 2,
  provider_type: "github",
  provider_base_url: "https://github.com",
  http_clone_url: "https://github.com/owner/repo.git",
  ssh_clone_url: "git@github.com:owner/repo.git",
  external_id: "owner/repo",
  name: "repo",
  slug: "owner-repo",
  default_branch: "main",
  ticket_prefix: "REPO",
  visibility: "private",
  imported_by_user_id: 99,
  is_active: true,
  webhook_config: {
    id: "wh-1",
    url: "https://api.example.com/wh",
    events: ["push", "pull_request"],
    is_active: true,
    needs_manual_setup: false,
    last_error: undefined,
    created_at: "2026-05-01T00:00:00Z",
  },
  created_at: "2026-05-01T00:00:00Z",
  updated_at: "2026-05-27T10:00:00Z",
};

describe("repositoryToProto", () => {
  it("translates snake_case scalars to camelCase fields", () => {
    const proto = repositoryToProto(fixture);
    expect(proto.providerType).toBe("github");
    expect(proto.providerBaseUrl).toBe("https://github.com");
    expect(proto.httpCloneUrl).toBe("https://github.com/owner/repo.git");
    expect(proto.sshCloneUrl).toBe("git@github.com:owner/repo.git");
    expect(proto.externalId).toBe("owner/repo");
    expect(proto.name).toBe("repo");
    expect(proto.slug).toBe("owner-repo");
    expect(proto.defaultBranch).toBe("main");
    expect(proto.ticketPrefix).toBe("REPO");
    expect(proto.visibility).toBe("private");
    expect(proto.isActive).toBe(true);
    expect(proto.createdAt).toBe("2026-05-01T00:00:00Z");
    expect(proto.updatedAt).toBe("2026-05-27T10:00:00Z");
  });

  it("converts int64 fields (id / organization_id / imported_by_user_id) to BigInt", () => {
    const proto = repositoryToProto(fixture);
    expect(proto.id).toBe(BigInt(7));
    expect(proto.organizationId).toBe(BigInt(2));
    expect(proto.importedByUserId).toBe(BigInt(99));
  });

  it("defaults clone URLs to empty string when source absent", () => {
    const minimal: RepositoryData = {
      ...fixture,
      http_clone_url: undefined,
      ssh_clone_url: undefined,
    };
    const proto = repositoryToProto(minimal);
    expect(proto.httpCloneUrl).toBe("");
    expect(proto.sshCloneUrl).toBe("");
  });

  it("importedByUserId stays undefined when source absent (optional int64)", () => {
    const noImporter: RepositoryData = { ...fixture, imported_by_user_id: undefined };
    const proto = repositoryToProto(noImporter);
    expect(proto.importedByUserId).toBeUndefined();
  });

  it("translates nested webhook_config snake_case → camelCase", () => {
    const proto = repositoryToProto(fixture);
    expect(proto.webhookConfig).toBeDefined();
    expect(proto.webhookConfig?.id).toBe("wh-1");
    expect(proto.webhookConfig?.url).toBe("https://api.example.com/wh");
    expect(proto.webhookConfig?.events).toEqual(["push", "pull_request"]);
    expect(proto.webhookConfig?.isActive).toBe(true);
    expect(proto.webhookConfig?.needsManualSetup).toBe(false);
    expect(proto.webhookConfig?.createdAt).toBe("2026-05-01T00:00:00Z");
  });

  it("emits undefined webhookConfig when source absent", () => {
    const noHook: RepositoryData = { ...fixture, webhook_config: undefined };
    const proto = repositoryToProto(noHook);
    expect(proto.webhookConfig).toBeUndefined();
  });
});
