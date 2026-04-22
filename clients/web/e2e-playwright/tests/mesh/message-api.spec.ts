import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Mesh Message API Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list messages via topology", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/mesh/topology`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.topology).toBeTruthy();
  });

  test("get channel unread counts", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/channels/unread`);
    expect(res.status).toBe(200);
  });
});
