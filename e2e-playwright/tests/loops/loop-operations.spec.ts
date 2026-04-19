import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { collectConsoleErrors, assertNoWasmErrors } from "../../helpers/console-errors";

test.describe("Loop Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("loops: open create dialog", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/loops`);
    await page.waitForLoadState("networkidle");

    const createBtn = page.getByRole("button", { name: /新建|Create|New/i }).first();
    if (await createBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await createBtn.click();
      await page.waitForTimeout(500);
    }
    assertNoWasmErrors(errors);
  });

  test("loops: list → detail navigation", async ({ page, api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
      name: `E2E Loop Nav ${Date.now()}`,
      agent_slug: "claude-code",
      schedule: "0 * * * *",
      prompt_template: "echo nav test",
    });
    expect([200, 201]).toContain(createRes.status);
    const slug = (await createRes.json()).loop?.slug;

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/loops`);
    await page.waitForLoadState("networkidle");

    const link = page.locator(`a[href*="loops/${slug}"]`).first();
    if (await link.isVisible({ timeout: 5000 }).catch(() => false)) {
      await link.click();
      await page.waitForLoadState("networkidle");
    }
    assertNoWasmErrors(errors);

    if (slug) await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`);
  });
});
