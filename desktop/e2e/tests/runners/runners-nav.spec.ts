import { test } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { currentRoute } from "../../helpers/nav";
import { expect } from "@playwright/test";

/** Runners is now an Infrastructure sub-tab inside Settings — verify the redirect. */
test("Runners · legacy route redirects into Settings > Infrastructure", async ({ page }) => {
  // Directly trigger the legacy hash; the page's redirect effect bounces us to Settings.
  await page.evaluate((slug) => {
    window.location.hash = `#/${slug}/runners`;
  }, TEST_ORG_SLUG);

  await page.waitForFunction(
    () => window.location.hash.includes("/settings") && window.location.hash.includes("infra/runners"),
    null,
    { timeout: 15_000 },
  );

  const route = await currentRoute(page);
  expect(route).toContain("/settings");
  expect(route).toContain("infra/runners");
});
