import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Extension Management API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list skill registries", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/skill-registries`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.skill_registries).toBeDefined();
  });

  test("list skill registry overrides", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/skill-registry-overrides`);
    expect(res.status).toBe(200);
  });

  test("list market skills", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/market/skills`);
    expect(res.status).toBe(200);
  });

  test("list market mcp servers", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/market/mcp-servers`);
    expect(res.status).toBe(200);
  });

  test("list repo skills", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/repositories/${id}/skills`);
    expect(res.status).toBe(200);
  });

  test("list repo mcp servers", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) return;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/repositories/${id}/mcp-servers`);
    expect(res.status).toBe(200);
  });
});
