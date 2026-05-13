// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test, expect } from "../../../fixtures";
import { invokeIpc } from "../../../helpers/ipc";

test.describe("IPC · promocode", () => {
  test("promocode_validate_promo_code_connect", async ({ page }) => {
    const result = await invokeIpc(page, "promocode_validate_promo_code_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("promocode_redeem_promo_code_connect", async ({ page }) => {
    const result = await invokeIpc(page, "promocode_redeem_promo_code_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });

  test("promocode_get_redemption_history_connect", async ({ page }) => {
    const result = await invokeIpc(page, "promocode_get_redemption_history_connect", []).catch((err: Error) => ({ __ipcError: err.message }));
    expect(result).toBeDefined();
  });
});
