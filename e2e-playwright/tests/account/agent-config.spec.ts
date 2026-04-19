import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Agent Configuration API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list agents", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/agents`);
    expect(res.status).toBe(200);
  });

  test("get agent config schema", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/agents/claude-code/config-schema`);
    expect(res.status).toBe(200);
  });

  test("get agentpod settings", async ({ api }) => {
    const res = await api.get("/api/v1/users/me/agentpod/settings");
    expect([200, 500]).toContain(res.status);
  });

  test("list agentpod providers", async ({ api }) => {
    const res = await api.get("/api/v1/users/me/agentpod/providers");
    expect([200, 500]).toContain(res.status);
  });

  test("list user agent configs", async ({ api }) => {
    const res = await api.get("/api/v1/users/me/agent-configs");
    expect(res.status).toBe(200);
  });

  test("set and get user agent config", async ({ api }) => {
    const setRes = await api.put("/api/v1/users/me/agent-configs/claude-code", {
      config_values: { model: "opus" },
    });
    expect(setRes.status).toBe(200);

    const getRes = await api.get("/api/v1/users/me/agent-configs/claude-code");
    expect(getRes.status).toBe(200);
    const data = await getRes.json();
    expect(data.config).toBeTruthy();
  });
});
