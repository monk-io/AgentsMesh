import { test } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { currentRoute } from "../../helpers/nav";
import { expect } from "@playwright/test";

/** Runners is now an Infrastructure tab — verify the redirect to /infra?tab=runners. */
test("Runners · legacy route redirects into Infrastructure tab", async ({ page }) => {
  // Directly trigger the legacy hash; the page's redirect effect bounces us to Infra.
  await page.evaluate((slug) => {
    window.location.hash = `#/${slug}/runners`;
  }, TEST_ORG_SLUG);

  await page.waitForFunction(
    () => /\/infra\b.*tab=runners/.test(window.location.hash),
    null,
    { timeout: 15_000 },
  );

  const route = await currentRoute(page);
  expect(route).toContain("/infra");
  expect(route).toContain("tab=runners");
});
