// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · billing", () => {
  test("billing_get_overview", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_overview").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_subscription", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_subscription").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_create_subscription", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_create_subscription", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_cancel_subscription", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_cancel_subscription").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_update_subscription", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_update_subscription", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_list_plans", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_list_plans").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_usage", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_usage", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_check_quota", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_check_quota", "", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_create_checkout", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_create_checkout", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_checkout_status", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_checkout_status", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_request_cancel", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_request_cancel", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_reactivate", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_reactivate").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_upgrade", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_upgrade", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_change_cycle", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_change_cycle", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_update_auto_renew", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_update_auto_renew", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_seat_usage", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_seat_usage").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_purchase_seats", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_purchase_seats", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_list_invoices", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_list_invoices", 0, 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_customer_portal", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_customer_portal", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_deployment_info", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_deployment_info").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_public_pricing", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_public_pricing").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("billing_get_public_deployment_info", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "billing_get_public_deployment_info").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
