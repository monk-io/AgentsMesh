import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";

/**
 * EnvBundle API regression: every legacy /users/agent-credentials route
 * has been replaced by /users/env-bundles with a unified payload shape
 * (kind + data). Tests verify the new shape and 401/404 boundary cases.
 */
test.describe("EnvBundle API", () => {
  test.beforeEach(async () => {
    clearAuthRateLimit();
  });

  test("list env bundles", async ({ api }) => {
    const res = await api.get("/api/v1/users/env-bundles");
    expect(res.status).toBe(200);
    const body = await res.json();
    expect(Array.isArray(body.items)).toBe(true);
  });

  test("list env bundles without auth returns 401", async ({ api }) => {
    const res = await api.getWithToken("/api/v1/users/env-bundles", "bad");
    expect(res.status).toBe(401);
  });

  test("create env bundle (credential kind)", async ({ api, db }) => {
    const name = `E2E Bundle ${Date.now()}`;
    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE 'E2E Bundle%'`);

    const res = await api.post("/api/v1/users/env-bundles", {
      agent_slug: "claude-code",
      name,
      description: "E2E test credential bundle",
      kind: "credential",
      data: { ANTHROPIC_API_KEY: "sk-ant-test-e2e" },
    });
    expect([200, 201]).toContain(res.status);

    const created = await res.json();
    expect(created.bundle?.kind).toBe("credential");
    expect(created.bundle?.name).toBe(name);
    // credential kind never echoes values back, only field names
    expect(created.bundle?.configured_fields).toContain("ANTHROPIC_API_KEY");
    expect(created.bundle?.configured_values).toBeUndefined();

    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE 'E2E Bundle%'`);
  });

  test("create env bundle (runtime kind echoes values back)", async ({ api, db }) => {
    const name = `E2E Runtime ${Date.now()}`;
    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE 'E2E Runtime%'`);

    const res = await api.post("/api/v1/users/env-bundles", {
      name,
      kind: "runtime",
      data: { LOG_LEVEL: "debug" },
    });
    expect([200, 201]).toContain(res.status);

    const created = await res.json();
    expect(created.bundle?.kind).toBe("runtime");
    // Non-encrypted kinds round-trip plaintext values
    expect(created.bundle?.configured_values?.LOG_LEVEL).toBe("debug");

    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE 'E2E Runtime%'`);
  });

  test("delete non-existent env bundle returns 404", async ({ api }) => {
    const res = await api.delete("/api/v1/users/env-bundles/999999");
    expect(res.status).toBe(404);
  });
});
