import { test } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { currentRoute } from "../../helpers/nav";
import { expect } from "@playwright/test";

/** Repositories is now an Infrastructure sub-tab inside Settings — verify the redirect. */
test("Repositories · legacy route redirects into Settings > Infrastructure", async ({ page }) => {
  await page.evaluate((slug) => {
    window.location.hash = `#/${slug}/repositories`;
  }, TEST_ORG_SLUG);

  await page.waitForFunction(
    () => window.location.hash.includes("/settings") && window.location.hash.includes("infra/repositories"),
    null,
    { timeout: 15_000 },
  );

  const route = await currentRoute(page);
  expect(route).toContain("/settings");
  expect(route).toContain("infra/repositories");
});
