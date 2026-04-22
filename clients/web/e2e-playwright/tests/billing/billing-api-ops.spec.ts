import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Billing API Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("get billing overview", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/overview`);
    expect(res.status).toBe(200);
  });

  test("get billing subscription", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/subscription`);
    expect([200, 404]).toContain(res.status);
  });

  test("list billing plans", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/plans`);
    expect(res.status).toBe(200);
  });

  test("get billing usage", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/usage`);
    expect(res.status).toBe(200);
  });

  test("get seat usage", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/seats`);
    expect(res.status).toBe(200);
  });

  test("list billing invoices", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/invoices`);
    expect(res.status).toBe(200);
  });

  test("get deployment info", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/deployment`);
    expect([200, 404]).toContain(res.status);
  });

  test("check quota", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/billing/quota/check?resource=pods`);
    expect(res.status).toBe(200);
  });
});
