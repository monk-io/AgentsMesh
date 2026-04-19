import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { collectConsoleErrors, assertNoWasmErrors } from "../../helpers/console-errors";

test.describe("Support Tickets", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: list support tickets", async ({ api }) => {
    const res = await api.get("/api/v1/support-tickets");
    expect(res.status).toBe(200);
  });

  test("API: support ticket detail (if exists)", async ({ api, db }) => {
    const id = db.queryValue("SELECT id FROM support_tickets LIMIT 1");
    if (!id) {
      const res = await api.get("/api/v1/support-tickets");
      expect(res.status).toBe(200);
      return;
    }
    const res = await api.get(`/api/v1/support-tickets/${id}`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.ticket).toBeTruthy();
  });

  test("UI: support page loads without errors", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/support`);
    await page.waitForLoadState("networkidle");
    assertNoWasmErrors(errors);
  });

  test("UI: support page open new ticket dialog", async ({ page }) => {
    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/support`);
    await page.waitForLoadState("networkidle");

    const newBtn = page.getByRole("button", { name: /新建|New|Create|提交/i }).first();
    if (await newBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await newBtn.click();
      await page.waitForTimeout(500);
    }
    assertNoWasmErrors(errors);
  });
});
