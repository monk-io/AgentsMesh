import { test, expect } from "../../fixtures/index";
import { SettingsNavPage } from "../../pages/settings/settings-nav.page";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Organization API Keys Settings", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: list api keys returns array", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/api-keys`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(Array.isArray(data.api_keys)).toBe(true);
  });

  test("API: create and delete api key", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/api-keys`, {
      name: "E2E Test Key",
      scopes: ["pods:read"],
    });
    expect([200, 201]).toContain(createRes.status);
    const data = await createRes.json();
    expect(data.api_key).toBeTruthy();
    expect(data.api_key.id).toBeTruthy();
    expect(data.raw_key).toBeTruthy();

    const delRes = await api.delete(
      `/api/v1/orgs/${TEST_ORG_SLUG}/api-keys/${data.api_key.id}`
    );
    expect([200, 204]).toContain(delRes.status);
  });

  test("UI: api keys settings page loads without errors", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    const nav = new SettingsNavPage(page, TEST_ORG_SLUG);
    await nav.goto("organization", "api-keys");

    const body = await page.textContent("body");
    expect(body).toMatch(/API|key|密钥/i);

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
