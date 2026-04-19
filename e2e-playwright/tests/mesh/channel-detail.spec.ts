import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Channel Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("API: list channels", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/channels`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.channels).toBeDefined();
  });

  test("API: create and get channel detail", async ({ api }) => {
    const name = `e2e-ch-${Date.now()}`;
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/channels`, {
      name,
      description: "E2E channel detail test",
    });
    expect([200, 201]).toContain(createRes.status);
    const data = await createRes.json();
    const id = data.channel?.id || data.id;
    expect(id).toBeTruthy();

    const detailRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/channels/${id}`);
    expect(detailRes.status).toBe(200);
    const detail = await detailRes.json();
    expect(detail.channel).toBeTruthy();
    expect(detail.channel.name).toBe(name);

    await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/channels/${id}`);
  });

  test("UI: channels page loads without errors", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/channels`);
    await page.waitForLoadState("networkidle");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
