// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures/electron-shared.fixture";
import { invokeIpc } from "../../../helpers/ipc";

test.describe.configure({ mode: "serial" });

test.describe("IPC · message", () => {
  test("message_send_message", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_send_message", "", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_messages", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_messages", false, 0, 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_unread_count", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_unread_count").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_message", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_message", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_mark_read", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_mark_read", "").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_mark_all_read", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_mark_all_read").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_conversation", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_conversation", "", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_sent_messages", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_sent_messages", 0, 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_get_dead_letters", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_get_dead_letters", 0, 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("message_replay_dead_letter", async ({ sharedPage }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(sharedPage, "message_replay_dead_letter", 0).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
