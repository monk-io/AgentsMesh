import { test } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { currentRoute } from "../../helpers/nav";
import { expect } from "@playwright/test";

/** Repositories is now an Infrastructure tab — verify the redirect to /infra?tab=repositories. */
test("Repositories · legacy route redirects into Infrastructure tab", async ({ page }) => {
  await page.evaluate((slug) => {
    window.location.hash = `#/${slug}/repositories`;
  }, TEST_ORG_SLUG);

  await page.waitForFunction(
    () => /\/infra\b.*tab=repositories/.test(window.location.hash),
    null,
    { timeout: 15_000 },
  );

  const route = await currentRoute(page);
  expect(route).toContain("/infra");
  expect(route).toContain("tab=repositories");
});
