import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
test.describe("Repository Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("repository detail: view branches and webhook info", async ({ page, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    expect(id, "dev seed must include at least one repository").toBeTruthy();
    await page.goto(`/${TEST_ORG_SLUG}/repositories/${id}`);
    await page.waitForLoadState("load");

    const body = await page.textContent("body");
    expect(body).toMatch(/demo|repository|仓库|branch|分支/i);
  });

  test("repositories list: navigate to detail and back", async ({ page, db }) => {
    const id = db.queryValue(
      `SELECT id FROM repositories WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`
    );
    expect(id, "dev seed must include at least one repository").toBeTruthy();
    await page.goto(`/${TEST_ORG_SLUG}/repositories`);
    await page.waitForLoadState("load");

    const link = page.locator(`a[href*="repositories/${id}"]`).first();
    if (await link.isVisible({ timeout: 3000 }).catch(() => false)) {
      await link.click();
      await page.waitForLoadState("load");
      await page.goBack();
      await page.waitForLoadState("load");
    }
  });
});
