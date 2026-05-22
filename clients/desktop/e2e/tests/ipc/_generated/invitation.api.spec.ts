// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · invitation", () => {
  test("invitation_list_invitations_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_list_invitations_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_create_invitation_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_create_invitation_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_revoke_invitation_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_revoke_invitation_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_resend_invitation_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_resend_invitation_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_accept_invitation_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_accept_invitation_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_list_pending_invitations_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_list_pending_invitations_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("invitation_get_invitation_by_token_connect", async ({ sharedPage }) => {
    const result = await invokeIpc(sharedPage, "invitation_get_invitation_by_token_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
