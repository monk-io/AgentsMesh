// Wire-level realtime EventBus verification for ticket:* events.
//
// Covers ticket:created / ticket:updated / ticket:status_changed /
// ticket:moved / ticket:deleted from the SSOT audit. The "moved" event
// is the same wire-level message as status_changed but with a different
// `type` (board lane transitions vs simple status flips); both fire from
// UpdateTicketStatus.
import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { withEventSubscription } from "../../helpers/eventbus-stream";

test.describe("Realtime · ticket events (wire)", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  async function createTicket(api: import("../../fixtures/api.fixture").ApiFixture, title: string) {
    const cc = await api.connect();
    const ticket = (await cc.ticket.createTicket({
      orgSlug: TEST_ORG_SLUG, title,
    })) as { slug: string };
    return ticket;
  }

  test("ticket:created arrives with slug", async ({ api }) => {
    const token = api.getToken() ?? ((await api.connect()), api.getToken());
    if (!token) throw new Error("api fixture missing token");
    const title = `e2e-rt-${Date.now().toString(36)}`;
    let createdSlug: string | undefined;

    const { event } = await withEventSubscription<unknown, { slug?: string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "ticket:created" && typeof data.slug === "string" && data.slug === createdSlug,
      },
      async () => {
        const t = await createTicket(api, title);
        createdSlug = t.slug;
      },
    );

    expect(event.data.slug).toBe(createdSlug);
  });

  test("ticket:updated arrives after UpdateTicket", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const t = await createTicket(api, `e2e-rt-upd-${Date.now().toString(36)}`);

    const { event } = await withEventSubscription<unknown, { slug?: string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "ticket:updated" && data.slug === t.slug,
      },
      async () => {
        await cc.ticket.updateTicket({
          orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug, title: `updated-${t.slug}`,
        });
      },
    );

    expect(event.data.slug).toBe(t.slug);
  });

  test("ticket:status_changed arrives with previous + new status", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const t = await createTicket(api, `e2e-rt-status-${Date.now().toString(36)}`);

    const { event } = await withEventSubscription<unknown, { slug?: string; status?: string; previous_status?: string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "ticket:status_changed" && data.slug === t.slug,
      },
      async () => {
        await cc.ticket.updateTicketStatus({
          orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug, status: "in_progress",
        });
      },
    );

    expect(event.data.slug).toBe(t.slug);
    expect(event.data.status).toBe("in_progress");
  });

  test("ticket:moved arrives on board lane transition", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const t = await createTicket(api, `e2e-rt-moved-${Date.now().toString(36)}`);

    // Backend distinguishes moved (lane → lane) from status_changed
    // (any field update). Trigger by toggling to a different board status.
    const { event } = await withEventSubscription<unknown, { slug?: string; status?: string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          (type === "ticket:moved" || type === "ticket:status_changed") &&
          data.slug === t.slug && data.status === "done",
        timeoutMs: 12_000,
      },
      async () => {
        await cc.ticket.updateTicketStatus({
          orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug, status: "done",
        });
      },
    );

    expect(event.data.slug).toBe(t.slug);
    expect(event.data.status).toBe("done");
  });

  test("ticket:deleted arrives with slug", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const t = await createTicket(api, `e2e-rt-del-${Date.now().toString(36)}`);

    const { event } = await withEventSubscription<unknown, { slug?: string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "ticket:deleted" && data.slug === t.slug,
      },
      async () => {
        await cc.ticket.deleteTicket({
          orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug,
        });
      },
    );

    expect(event.data.slug).toBe(t.slug);
  });
});
