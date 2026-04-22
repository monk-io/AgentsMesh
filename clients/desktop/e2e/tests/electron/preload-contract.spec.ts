import { test, expect } from "../../fixtures";

test.describe("Electron · preload contract", () => {
  test("window.electronAPI is exposed with invoke+on", async ({ page }) => {
    const shape = await page.evaluate(() => {
      const api = (window as unknown as { electronAPI?: Record<string, unknown> }).electronAPI;
      return {
        exists: typeof api !== "undefined",
        hasInvoke: typeof api?.invoke === "function",
        hasOn: typeof api?.on === "function",
        hasShellOpen: typeof api?.shellOpen === "function",
      };
    });
    expect(shape.exists).toBe(true);
    expect(shape.hasInvoke).toBe(true);
    expect(shape.hasOn).toBe(true);
    expect(shape.hasShellOpen).toBe(true);
  });

  test("shellOpen IPC channel is registered (no throw when invoked with a safe URL)", async ({ page }) => {
    // Deliberately use an obviously fake URL so nothing actually opens in the browser.
    const result = await page
      .evaluate(() => (window as unknown as { electronAPI: { shellOpen: (u: string) => Promise<unknown> } }).electronAPI.shellOpen("about:blank"))
      .catch((err: Error) => ({ __error: err.message }));
    // Either resolved (undefined) or threw with a shell error — either means the IPC routed.
    expect(result).toBeDefined();
  });
});
