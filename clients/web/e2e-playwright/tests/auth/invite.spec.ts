import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Invitation Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: get invitation by invalid token returns 404", async ({ api }) => {
    const res = await api.get("/api/v1/invitations/invalid-token-12345");
    expect([404, 400]).toContain(res.status);
  });

  test("UI: invite page with invalid token shows error gracefully", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto("/invite/invalid-token-e2e");
    await page.waitForLoadState("networkidle");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
