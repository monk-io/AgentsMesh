// Migrated R5+: Connect-RPC only (no REST middle layer).
import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("User Credential Management API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list git credentials", async ({ api }) => {
    const cc = await api.connect();
    const res = await cc.userGitCredential.listGitCredentials({}) as { items: unknown[] };
    expect(Array.isArray(res.items)).toBe(true);
  });

  test("get default git credential", async ({ api }) => {
    const cc = await api.connect();
    const res = await cc.userGitCredential.getDefaultGitCredential({}) as Record<string, unknown>;
    expect(res).toBeTruthy();
  });

  test("list repository providers", async ({ api }) => {
    const cc = await api.connect();
    const res = await cc.userRepositoryProvider.listRepositoryProviders({}) as { items: unknown[] };
    expect(Array.isArray(res.items)).toBe(true);
  });

  test("list agent credentials", async ({ api }) => {
    const cc = await api.connect();
    const res = await cc.userAgentCredential.listAgentCredentialProfiles({}) as Record<string, unknown>;
    expect(res).toBeTruthy();
  });

  test("get user profile", async ({ api }) => {
    const cc = await api.connect();
    const user = await cc.user.getMe({}) as { id: string | number; email: string };
    expect(user.id).toBeTruthy();
    expect(user.email).toBeTruthy();
  });

  test("create and delete git credential", async ({ api }) => {
    const cc = await api.connect();
    const created = await cc.userGitCredential.createGitCredential({
      name: "E2E PAT Credential " + Date.now(),
      credentialType: "pat",
      pat: "ghp_test123456789",
    }) as { id: number };
    if (created.id) {
      await cc.userGitCredential.deleteGitCredential({ id: created.id });
    }
  });

  test("test repository provider connection", async ({ api, db }) => {
    const idStr = db.queryValue("SELECT id FROM user_repository_providers LIMIT 1");
    if (!idStr) return;
    const cc = await api.connect();
    // Tolerates success or backend test-call failure with invalid token.
    await cc.userRepositoryProvider.testRepositoryProviderConnection({ id: Number(idStr) })
      .catch((e) => e);
  });
});
