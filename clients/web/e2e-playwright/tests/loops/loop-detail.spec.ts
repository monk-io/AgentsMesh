import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { collectConsoleErrors, assertNoWasmErrors } from "../../helpers/console-errors";

test.describe("Loop Detail Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  let createdSlug: string | null = null;

  test.afterEach(async ({ api }) => {
    if (createdSlug) {
      await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${createdSlug}`);
      createdSlug = null;
    }
  });

  test("API: get loop detail returns wrapped response", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
      name: `E2E Loop Detail ${Date.now()}`,
      agent_slug: "claude-code",
      schedule: "0 * * * *",
      prompt_template: "echo test",
    });
    expect([200, 201]).toContain(createRes.status);
    const created = await createRes.json();
    createdSlug = created.loop?.slug;

    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${createdSlug}`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.loop).toBeTruthy();
  });

  test("UI: loop detail page renders without errors", async ({ page, api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
      name: `E2E Loop UI ${Date.now()}`,
      agent_slug: "claude-code",
      schedule: "0 * * * *",
      prompt_template: "echo test",
    });
    const created = await createRes.json();
    createdSlug = created.loop?.slug;

    const errors = collectConsoleErrors(page);
    await page.goto(`/${TEST_ORG_SLUG}/loops/${createdSlug}`);
    await page.waitForLoadState("networkidle");
    assertNoWasmErrors(errors);
  });
});
