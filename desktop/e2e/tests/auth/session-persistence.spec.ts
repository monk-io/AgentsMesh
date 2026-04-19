import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { currentRoute } from "../../helpers/nav";

test.describe("Auth · session persistence", () => {
  test("hash route indicates a logged-in page (not /login)", async ({ page }) => {
    // Give React Router a moment to settle after auth-state restore + reload.
    await page.waitForTimeout(500);
    const route = await currentRoute(page);
    expect(route).not.toContain("/login");
    expect(
      route.includes(`/${TEST_ORG_SLUG}/`) ||
      route.includes("/workspace") ||
      route.includes("/onboarding") ||
      route.includes("/settings")
    ).toBe(true);
  });

  test("localStorage has at least one agentsmesh-* key", async ({ page }) => {
    const keys = await page.evaluate(() => Object.keys(window.localStorage).filter((k) => k.startsWith("agentsmesh-")));
    expect(keys.length).toBeGreaterThan(0);
  });
});
