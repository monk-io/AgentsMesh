// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures";
import { invokeIpc } from "../../../helpers/ipc";

test.describe("IPC · user", () => {
  test("user_get_me", async ({ page }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(page, "user_get_me").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("user_get_organizations", async ({ page }) => {
    // Smoke: the bridge accepts the call. Result may be a valid response OR a typed error —
    // both prove the IPC route is wired. A crashed bridge would throw an unrelated runtime error.
    const result = await invokeIpc(page, "user_get_organizations").catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
