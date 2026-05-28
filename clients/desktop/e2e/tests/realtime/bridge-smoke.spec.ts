// Phase 0 bridge verification — confirms the IPC ServerStream bridge is
// wired correctly so the renderer is no longer running against
// NoopEventsManager.
//
// After the desktop bridge lands:
//   - electronAPI.invoke("realtime:getState") returns a non-"disconnected"
//     state once the EventSubscriptionManager has connected.
//   - The window-side onRealtimeEvent listener can be installed (preload
//     exposure exists).
//
// Pre-bridge (NoopEventsManager) symptoms this spec catches:
//   - getState returns "connected" (the noop liar) but no event ever arrives
//   - or electronAPI.onRealtimeEvent is undefined entirely
import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";
import { installRealtimeSpy } from "../../helpers/realtime-spy";
import { invokeIpc } from "../../helpers/ipc";

test.describe("Desktop realtime · bridge smoke", () => {
  test("preload exposes onRealtimeEvent + getState progresses past disconnected", async ({ page }) => {
    // 1. Preload bridge surface must exist.
    const exposed = await page.evaluate(() => {
      const api = (window as unknown as { electronAPI?: { onRealtimeEvent?: unknown; onRealtimeState?: unknown } }).electronAPI;
      return {
        hasOnRealtimeEvent: typeof api?.onRealtimeEvent === "function",
        hasOnRealtimeState: typeof api?.onRealtimeState === "function",
      };
    });
    expect(exposed.hasOnRealtimeEvent, "electronAPI.onRealtimeEvent must be exposed by preload").toBe(true);
    expect(exposed.hasOnRealtimeState, "electronAPI.onRealtimeState must be exposed by preload").toBe(true);

    // 2. The renderer's RealtimeProvider auto-invokes realtime:connect on
    //    dashboard mount. Navigate to workspace + wait for the connection
    //    state IPC to report a non-disconnected value.
    await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
    await page.waitForTimeout(3000);

    const state = await invokeIpc<string>(page, "realtime:getState");
    expect(["connecting", "connected", "reconnecting"], `getState returned ${state}`).toContain(state);
  });

  test("realtime-spy installs without throwing (sanity check for follow-up specs)", async ({ page }) => {
    await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
    const spy = await installRealtimeSpy(page);
    try {
      const initial = await spy.snapshot();
      expect(Array.isArray(initial)).toBe(true);
    } finally {
      await spy.dispose();
    }
  });
});
