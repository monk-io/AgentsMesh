import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Ticket Labels & Relations API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list labels", async ({ api }) => {
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/labels`);
    expect(res.status).toBe(200);
  });

  test("create and delete label", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/labels`, {
      name: "e2e-label",
      color: "#ff0000",
    });
    expect([200, 201]).toContain(createRes.status);
    const data = await createRes.json();
    const id = data.label?.id || data.id;
    if (id) {
      const delRes = await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/labels/${id}`);
      expect([200, 204]).toContain(delRes.status);
    }
  });

  test("get ticket relations", async ({ api }) => {
    const ticketsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets?limit=1`);
    const tickets = (await ticketsRes.json()).tickets || [];
    if (tickets.length === 0) return;

    const slug = tickets[0].slug;
    const relRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}/relations`);
    expect(relRes.status).toBe(200);
  });

  test("get ticket commits", async ({ api }) => {
    const ticketsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets?limit=1`);
    const tickets = (await ticketsRes.json()).tickets || [];
    if (tickets.length === 0) return;

    const slug = tickets[0].slug;
    const res = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}/commits`);
    expect(res.status).toBe(200);
  });

  test("delete ticket", async ({ api }) => {
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets`, {
      title: "E2E Delete Test",
    });
    const slug = (await createRes.json()).ticket?.slug;
    if (!slug) return;

    const delRes = await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/${slug}`);
    expect([200, 204]).toContain(delRes.status);
  });

  test("batch get ticket pods", async ({ api }) => {
    const ticketsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets?limit=1`);
    const tickets = (await ticketsRes.json()).tickets || [];
    if (tickets.length === 0) return;

    const res = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/tickets/batch-pods`, {
      ticket_ids: [tickets[0].id],
    });
    expect([200, 400]).toContain(res.status);
  });
});
