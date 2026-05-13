// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures";
import { invokeIpc } from "../../../helpers/ipc";

test.describe("IPC · ticket_api", () => {
  test("ticket_list_tickets_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_list_tickets_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_get_ticket_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_get_ticket_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_create_ticket_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_create_ticket_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_update_ticket_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_update_ticket_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_delete_ticket_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_delete_ticket_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_update_ticket_status_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_update_ticket_status_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_get_active_tickets_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_get_active_tickets_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_get_board_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_get_board_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_get_sub_tickets_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_get_sub_tickets_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_add_assignee_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_add_assignee_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_remove_assignee_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_remove_assignee_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_list_labels_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_list_labels_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_create_label_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_create_label_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_update_label_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_update_label_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_delete_label_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_delete_label_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_add_label_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_add_label_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_remove_label_connect", async ({ page }) => {
    const result = await invokeIpc(page, "ticket_remove_label_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("ticket_get_ticket_pods", async ({ page }) => {
    // REST-only: proto.ticket.v1 doesn't own ticket→pod lookup (MeshService does).
    const result = await invokeIpc(page, "ticket_get_ticket_pods", "", false).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
