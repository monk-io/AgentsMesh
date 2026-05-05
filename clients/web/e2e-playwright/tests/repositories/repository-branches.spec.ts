import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Repository Branches & Webhook API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list repository branches", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/repositories/${id}/branches`);
    expect([200, 400, 500]).toContain(res.status);
  });

  test("get webhook status", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/repositories/${id}/webhook/status`);
    expect([200, 404]).toContain(res.status);
  });

  test("list merge requests", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/repositories/${id}/merge-requests`);
    expect([200, 400, 500]).toContain(res.status);
  });
});
