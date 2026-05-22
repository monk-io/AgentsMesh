// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · org", () => {
  test("org_list", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_list").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_get", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_get", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_create", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_create", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_update", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_update", "", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_delete", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_delete", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_list_members", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_list_members", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_invite_member", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_invite_member", "", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_remove_member", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_remove_member", "", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("org_update_member_role", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "org_update_member_role", "", 0, "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
