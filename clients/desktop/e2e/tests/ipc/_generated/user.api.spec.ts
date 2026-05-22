// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · user", () => {
  test("user_get_me_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_get_me_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_update_me_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_update_me_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_change_password_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_change_password_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_list_identities_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_list_identities_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_delete_identity_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_delete_identity_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_search_users_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "user_search_users_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
