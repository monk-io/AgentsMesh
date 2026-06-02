import { test, expect } from "../../fixtures";
import { TicketsBoardPage } from "../../pages/tickets-board.page";
import { TicketDetailPage } from "../../pages/tickets-detail.page";

// Regression coverage for the ElectronTicketState stub bug. Before the
// proto-bytes mirror fix (packages/electron-adapter/src/ticket_state.ts),
// state_adapters.ts shipped an ElectronTicketState whose board_columns_json()
// / current_ticket_json() returned empty and every mutator was a no-op:
// Connect-RPC fetched real data, the store called replace_board_columns(bytes)
// / set_current_ticket(bytes), the stub dropped the bytes, the kanban rendered
// empty and ticket detail showed "not found". ipc/_generated/ticket.api.spec
// only proves the IPC route is wired — it never checked the state cache layer.
// This spec closes that gap (mirrors repositories-list.spec.ts).
test.describe("Desktop tickets · board + detail render", () => {
  test("kanban renders seeded tickets (not the empty stub)", async ({ page }) => {
    const board = new TicketsBoardPage(page);
    await board.goto();
    await board.expectOnPage();

    // Stub fingerprint: zero cards while dev seed has DEV-1/DEV-2 (both backlog).
    await expect(
      board.ticketCards.first(),
      "no ticket cards while backend has seeded tickets — ElectronTicketState stub regressed?",
    ).toBeVisible({ timeout: 10_000 });
    expect(await board.ticketCards.count()).toBeGreaterThan(0);

    await expect(page.locator('[data-ticket-slug="DEV-1"]').first()).toBeVisible();
  });

  test("ticket detail loads the ticket (not 'not found')", async ({ page }) => {
    const detail = new TicketDetailPage(page);
    await detail.goto("DEV-1");
    await detail.expectOnPage("DEV-1");

    // set_current_ticket no-op → current_ticket_json() null → "not found".
    // Positive-assert the seeded title renders instead of the empty-state text.
    await expect(
      page.getByText("Implement JWT-based user authentication", { exact: false }).first(),
      "ticket detail empty while backend has DEV-1 — set_current_ticket dropped the bytes?",
    ).toBeVisible({ timeout: 10_000 });
  });
});
