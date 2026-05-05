import { test, expect } from "../../fixtures";
import { invokeIpc } from "../../helpers/ipc";

/**
 * Regression guard: support-ticket list serialization between Go backend
 * and Rust core stayed broken silently because no e2e ever exercised the
 * IPC.
 *
 * Backend returns `{ data, total, page, page_size, total_pages }` (gin
 * paginated convention). The Rust `SupportTicketListResponse` originally
 * declared the field as `tickets`, so deserialization failed with
 * "missing field `tickets`". The renderer caught the error and showed
 * "Failed to load data" — no console error reached the desktop pageerror
 * handler, no test asserted on the IPC return shape.
 *
 * This spec calls `supportTicketList` straight through the IPC bridge and
 * asserts the response parses into the documented shape. If anyone ever
 * renames a field on either side again, this trips immediately.
 */
test("Support tickets · IPC response parses without error", async ({ page }) => {
  const json = await invokeIpc<string>(page, "supportTicketList", null, 1, 20);

  // Top-level call must not throw — no '{"kind":"invalid_json"...}'
  expect(json, "supportTicketList must return a JSON string").toBeTruthy();

  const parsed = JSON.parse(json) as Record<string, unknown>;

  // Field names track the Go handler shape (gin pagination), which
  // is also what the TS interface in clients/web/src/lib/api/
  // supportTicketTypes.ts expects.
  expect(parsed, "must have `data` array").toHaveProperty("data");
  expect(Array.isArray((parsed as { data: unknown[] }).data), "data is array").toBe(true);
  expect(parsed, "must have `total`").toHaveProperty("total");
});
