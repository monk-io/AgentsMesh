// Multi-tab UI propagation for ticket:status_changed.
//
// Both tabs land on /tickets after a ticket is created via Connect-RPC.
// The board view places each ticket card under the column matching its
// status — flipping the ticket through statuses moves the card across
// columns. We assert the parent column's `data-column-status` to remain
// locale-free.
//
// Wire-level coverage in tests/realtime/ticket-events-wire.spec.ts;
// this spec validates handler → updateTicketStatusFromEvent → React.
import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";
import { TEST_ORG_SLUG } from "../../helpers/env";

test.describe("Ticket status · multi-tab UI propagation", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("tab A status change → tab B ticket moves between board columns", async ({ context, api }) => {
    const cc = await api.connect();

    const stamp = Date.now().toString(36);
    const t = (await cc.ticket.createTicket({
      orgSlug: TEST_ORG_SLUG, title: `e2e-status-${stamp}`,
    })) as { slug: string };

    const tabA = await context.newPage();
    const tabB = await context.newPage();
    await Promise.all([
      tabA.goto(`/${TEST_ORG_SLUG}/tickets`),
      tabB.goto(`/${TEST_ORG_SLUG}/tickets`),
    ]);

    // CreateTicket defaults to "backlog" status; board view places the
    // card in the column with matching data-column-status.
    const cardIn = (status: string) =>
      `[data-testid="kanban-column"][data-column-status="${status}"] [data-testid="ticket-card"][data-ticket-slug="${t.slug}"]`;

    await Promise.all([
      expect(tabA.locator(cardIn("backlog"))).toHaveCount(1, { timeout: 30_000 }),
      expect(tabB.locator(cardIn("backlog"))).toHaveCount(1, { timeout: 30_000 }),
    ]);

    // EventSubscriptionManager bootstrap settle window before publish.
    await tabA.waitForTimeout(1500);

    await cc.ticket.updateTicketStatus({
      orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug, status: "in_progress",
    });

    await Promise.all([
      expect(tabA.locator(cardIn("in_progress"))).toHaveCount(1, { timeout: 10_000 }),
      expect(tabB.locator(cardIn("in_progress"))).toHaveCount(1, { timeout: 10_000 }),
      expect(tabA.locator(cardIn("backlog"))).toHaveCount(0, { timeout: 10_000 }),
      expect(tabB.locator(cardIn("backlog"))).toHaveCount(0, { timeout: 10_000 }),
    ]);

    // "in_review" stays in expanded columns; "done" defaults to collapsed
    // (CollapsedColumn doesn't render individual tickets), so we assert
    // the move into in_review where the card stays visible.
    await cc.ticket.updateTicketStatus({
      orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug, status: "in_review",
    });

    await Promise.all([
      expect(tabA.locator(cardIn("in_review"))).toHaveCount(1, { timeout: 10_000 }),
      expect(tabB.locator(cardIn("in_review"))).toHaveCount(1, { timeout: 10_000 }),
    ]);

    await tabA.close();
    await tabB.close();
    await cc.ticket.deleteTicket({ orgSlug: TEST_ORG_SLUG, ticketSlug: t.slug }).catch(() => undefined);
  });
});
