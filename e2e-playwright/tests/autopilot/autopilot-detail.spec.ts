import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Autopilot Detail Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: list autopilot controllers", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/autopilot-controllers`);
    expect(res.status).toBe(200);
  });

  test("UI: mesh page with autopilot loads without errors", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/mesh`);
    await page.waitForLoadState("networkidle");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
