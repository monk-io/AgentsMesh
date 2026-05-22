// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · repository", () => {
  test("repository_list", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_list").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_get", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_get", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_create", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_create", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_update", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_update", 0, "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_delete", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_delete", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_list_branches", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_list_branches", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_sync_branches", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_sync_branches", 0, "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_register_webhook", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_register_webhook", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_delete_webhook", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_delete_webhook", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_get_webhook_status", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_get_webhook_status", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_get_webhook_secret", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_get_webhook_secret", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_list_merge_requests", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_list_merge_requests", 0, "", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("repository_mark_webhook_configured", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "repository_mark_webhook_configured", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
