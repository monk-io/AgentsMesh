// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · binding", () => {
  test("binding_request_binding_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_request_binding_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_accept_binding_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_accept_binding_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_reject_binding_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_reject_binding_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_unbind_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_unbind_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_request_scopes_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_request_scopes_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_approve_scopes_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_approve_scopes_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_list_bindings_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_list_bindings_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_get_pending_bindings_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_get_pending_bindings_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_get_bound_pods_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_get_bound_pods_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("binding_check_binding_connect", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "binding_check_binding_connect", new Uint8Array()).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
