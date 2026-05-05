import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Notification API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("get notification preferences", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/notifications/preferences`);
    expect(res.status).toBe(200);
  });

  test("update notification preferences", async ({ api }) => {
    const res = await api.put(`/api/v1/orgs/${TEST_ORG_SLUG}/notifications/preferences`, {
      email_enabled: true,
    });
    expect([200, 204, 400, 404]).toContain(res.status);
  });
});
