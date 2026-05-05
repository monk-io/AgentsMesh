import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Runner Diagnostics API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list runner pods", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM runners WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/${id}/pods`);
    expect(res.status).toBe(200);
  });

  test("list runner logs", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM runners WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/${id}/logs`);
    expect(res.status).toBe(200);
  });

  test("list runner tokens", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/grpc/tokens`);
    expect(res.status).toBe(200);
  });

  test("create and delete runner token", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/grpc/tokens`, {
      description: "E2E test token",
    });
    expect([200, 201]).toContain(createRes.status);
    const data = await createRes.json();
    const tokenId = data.token?.id || data.id;
    if (tokenId) {
      const delRes = await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/grpc/tokens/${tokenId}`);
      expect([200, 204]).toContain(delRes.status);
    }
  });
});
