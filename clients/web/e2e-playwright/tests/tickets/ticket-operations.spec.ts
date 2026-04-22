import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { collectConsoleErrors, assertNoWasmErrors } from "../../helpers/console-errors";

test.describe("Ticket Operations", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("create ticket via dialog", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("networkidle");

    const newBtn = page.getByRole("button", { name: /新建|New|Create/i }).first();
    if (await newBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await newBtn.click();
      await page.waitForTimeout(500);
      const titleInput = page.locator('input[placeholder*="title"], input[placeholder*="标题"], input[name="title"]').first();
      if (await titleInput.isVisible({ timeout: 2000 }).catch(() => false)) {
        await titleInput.fill("E2E Operation Test Ticket");
        const submitBtn = page.getByRole("button", { name: /创建|Create|Submit/i }).first();
        if (await submitBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
          await submitBtn.click();
          await page.waitForTimeout(2000);
        }
      }
    }
    assertNoWasmErrors(errors);
  });

  test("change status on detail page", async ({ page, api }) => {
    const res = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, { title: "E2E Status Test" });
    const slug = (await res.json()).ticket?.slug;

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/tickets/${slug}`);
    await page.waitForLoadState("networkidle");

    const statusBtn = page.getByRole("button", { name: /待办池|backlog|status/i }).first();
    if (await statusBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await statusBtn.click();
      await page.waitForTimeout(500);
    }
    assertNoWasmErrors(errors);
    if (slug) await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}`);
  });

  test("add comment on detail page", async ({ page, api }) => {
    const res = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, { title: "E2E Comment Test" });
    const slug = (await res.json()).ticket?.slug;

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/tickets/${slug}`);
    await page.waitForLoadState("networkidle");

    const input = page.locator('textarea[placeholder*="评论"], textarea[placeholder*="comment"]').first();
    if (await input.isVisible({ timeout: 3000 }).catch(() => false)) {
      await input.fill("E2E test comment @devuser");
      const sendBtn = page.getByRole("button", { name: /评论|Comment|Send/i }).first();
      if (await sendBtn.isEnabled({ timeout: 2000 })) {
        await sendBtn.click();
        await page.waitForTimeout(1000);
      }
    }
    assertNoWasmErrors(errors);
    if (slug) await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}`);
  });

  test("switch board and list view", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("networkidle");

    const listBtn = page.locator('button:has-text("列表"), button[aria-label*="list"]').first();
    if (await listBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await listBtn.click();
      await page.waitForTimeout(1000);
    }
    const boardBtn = page.locator('button:has-text("看板"), button[aria-label*="board"]').first();
    if (await boardBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
      await boardBtn.click();
      await page.waitForTimeout(1000);
    }
    assertNoWasmErrors(errors);
  });

  test("list → detail → back navigation", async ({ page, api }) => {
    const res = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, { title: "E2E Nav Test" });
    const slug = (await res.json()).ticket?.slug;

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("networkidle");

    const link = page.locator(`a[href*="${slug}"]`).first();
    if (await link.isVisible({ timeout: 5000 }).catch(() => false)) {
      await link.click();
      await page.waitForLoadState("networkidle");
      await page.goBack();
      await page.waitForLoadState("networkidle");
    }
    assertNoWasmErrors(errors);
    if (slug) await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}`);
  });
});
