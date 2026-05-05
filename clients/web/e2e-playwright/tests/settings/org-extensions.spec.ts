import { test, expect } from "../../fixtures/index";
import { SettingsNavPage } from "../../pages/settings/settings-nav.page";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Organization Extensions Settings", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: list skill registries", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/skill-registries`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.skill_registries).toBeDefined();
  });

  test("API: list skill registry overrides", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/skill-registry-overrides`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.overrides).toBeDefined();
  });

  test("UI: extensions settings page loads without errors", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    const nav = new SettingsNavPage(page, TEST_ORG_SLUG);
    await nav.goto("organization", "extensions");

    const body = await page.textContent("body");
    expect(body).toMatch(/extension|registry|扩展|注册/i);

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
