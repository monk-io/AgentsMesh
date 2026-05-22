// Migrated R5+: Connect-RPC only (no REST middle layer).
import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { collectConsoleErrors, assertNoWasmErrors } from "../../helpers/console-errors";

test.describe("Channel Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("channels: select channel and view messages", async ({ page, api }) => {
    const cc = await api.connect();
    const { items } = await cc.channel.listChannels({ orgSlug: TEST_ORG_SLUG }) as {
      items: unknown[];
    };
    if (!items || items.length === 0) { test.skip(); return; }

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/channels`);
    await page.waitForLoadState("load");

    const firstChannel = page.locator(`[data-channel-id], a[href*="channels"]`).first();
    if (await firstChannel.isVisible({ timeout: 3000 }).catch(() => false)) {
      await firstChannel.click();
      await page.waitForTimeout(1000);
    }
    assertNoWasmErrors(errors);
  });

  test("channels: create channel dialog", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/channels`);
    await page.waitForLoadState("load");

    const createBtn = page.getByRole("button", { name: /新建|Create|New/i }).first();
    if (await createBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await createBtn.click();
      await page.waitForTimeout(500);
    }
    assertNoWasmErrors(errors);
  });
});
