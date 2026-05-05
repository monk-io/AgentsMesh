import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Runner Detail Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: get single runner returns wrapped response", async ({ api, db }) => {
    const id = db.queryValue(
      `SELECT id FROM runners WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) { test.skip(); return; }

    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/runners/${id}`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.runner).toBeTruthy();
    expect(data.runner.id).toBeTruthy();
  });

  test("UI: runner detail page renders without errors", async ({ page, db }) => {
    const id = db.queryValue(
      `SELECT id FROM runners WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    if (!id) { test.skip(); return; }

    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/runners/${id}`);
    await page.waitForLoadState("networkidle");

    const body = await page.textContent("body");
    expect(body).not.toContain("missing field");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
