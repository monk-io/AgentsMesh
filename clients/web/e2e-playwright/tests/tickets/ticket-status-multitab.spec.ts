// Multi-tab UI propagation for ticket:status_changed.
//
// FIXME(follow-up): Tickets page lists status badges per ticket — the
// selector pattern is stable (`[data-ticket-slug]` on TicketCard.tsx),
// but the status badge text varies per locale. To assert cross-tab UI
// update we'd need either:
//   (a) a `data-status` attribute on the status badge so we can assert
//       on the machine-readable state regardless of i18n; or
//   (b) lock the test locale to en-US and assert on translated text.
//
// Wire-level coverage exists in tests/realtime/ticket-events-wire.spec.ts
// (ticket:status_changed wire frames). The UI assertion this spec would
// add is the badge swap from "in_progress" → "done" in tab B after tab A's
// UpdateTicketStatus RPC.
//
// Tracking: follow-up issue "test(e2e-web): ticket status multi-tab UI
// propagation" — depends on (a) above.
import { test } from "../../fixtures/index";

test.describe("Ticket status · multi-tab UI propagation", () => {
  test.fixme("tab A status change → tab B badge updates", async () => {
    // See file-level comment.
  });
});
