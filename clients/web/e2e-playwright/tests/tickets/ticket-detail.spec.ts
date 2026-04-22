import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Ticket Detail Page", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  let createdSlug: string | null = null;

  test.afterEach(async ({ api }) => {
    if (createdSlug) {
      await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${createdSlug}`);
      createdSlug = null;
    }
  });

  test("API: get single ticket returns wrapped response", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, {
      title: "E2E Detail Test Ticket",
      description: "Ticket for detail page test",
    });
    expect([200, 201]).toContain(createRes.status);
    const created = await createRes.json();
    createdSlug = created.ticket?.slug || created.slug;
    expect(createdSlug).toBeTruthy();

    const detailRes = await api.get(
      `/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${createdSlug}`
    );
    expect(detailRes.status).toBe(200);
    const detail = await detailRes.json();
    expect(detail.ticket).toBeTruthy();
    expect(detail.ticket.slug).toBe(createdSlug);
    expect(detail.ticket.title).toBe("E2E Detail Test Ticket");
  });

  test("UI: ticket detail page renders without errors", async ({ page, api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, {
      title: "E2E UI Detail Ticket",
      description: "UI detail page rendering test",
    });
    const created = await createRes.json();
    createdSlug = created.ticket?.slug || created.slug;

    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/tickets/${createdSlug}`);
    await page.waitForLoadState("networkidle");

    const body = await page.textContent("body");
    expect(body).toContain("E2E UI Detail Ticket");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON")
    );
    expect(jsonErrors).toHaveLength(0);
  });

  test("UI: navigate from ticket list to detail without errors", async ({ page, api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, {
      title: "E2E Nav Detail Ticket",
      description: "Test list-to-detail navigation",
    });
    const created = await createRes.json();
    createdSlug = created.ticket?.slug || created.slug;

    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("networkidle");

    const ticketLink = page.locator(`a[href*="${createdSlug}"], [data-ticket-slug="${createdSlug}"]`).first();
    if (await ticketLink.isVisible({ timeout: 5000 }).catch(() => false)) {
      await ticketLink.click();
      await page.waitForLoadState("networkidle");

      const jsonErrors = consoleErrors.filter(
        (e) => e.includes("missing field") || e.includes("is not valid JSON") || e.includes("Failed to fetch")
      );
      expect(jsonErrors).toHaveLength(0);
    }
  });

  test("UI: ticket board page loads without errors", async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("networkidle");

    const jsonErrors = consoleErrors.filter(
      (e) => e.includes("missing field") || e.includes("is not valid JSON") || e.includes("Failed to fetch board")
    );
    expect(jsonErrors).toHaveLength(0);
  });
});
