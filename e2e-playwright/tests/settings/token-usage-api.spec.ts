import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Token Usage API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("get token usage dashboard", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/token-usage/dashboard`);
    expect([200, 404]).toContain(res.status);
  });

  test("get token usage summary", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/token-usage/summary`);
    expect([200, 404]).toContain(res.status);
  });
});
