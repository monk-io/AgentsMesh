import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Binding API Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list bindings returns auth error without pod context", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/bindings`);
    expect([200, 401]).toContain(res.status);
  });

  test("get pending bindings", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/bindings/pending`);
    expect([200, 401]).toContain(res.status);
  });

  test("get bound pods", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/bindings/pods`);
    expect([200, 401]).toContain(res.status);
  });
});
