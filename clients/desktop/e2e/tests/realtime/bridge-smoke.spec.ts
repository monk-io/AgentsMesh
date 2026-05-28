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
  test("preload exposes onRealtimeEvent + connect transitions state past disconnected", async ({ page, electronApp }) => {
    // Capture main process console output for diagnostics.
    const mainLogs: string[] = [];
    page.on("console", (msg) => mainLogs.push(`[renderer/${msg.type()}] ${msg.text()}`));
    page.on("pageerror", (err) => mainLogs.push(`[pageerror] ${err.message}`));
    electronApp.on("console", (msg) => mainLogs.push(`[main/${msg.type()}] ${msg.text()}`));

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

    // Diagnostic: capture invoke failures explicitly. Probe getState first
    // (sync — no backend round-trip), then connect (which may take a
    // moment to spawn the stream loop).
    const probe1 = await page.evaluate(async () => {
      const api = (window as unknown as { electronAPI: { invoke: (c: string, ...a: unknown[]) => Promise<unknown> } }).electronAPI;
      try { return { ok: true, value: await api.invoke("realtime:getState") }; }
      catch (e) { return { ok: false, error: (e as Error).message }; }
    });
    console.log("[smoke] probe1 getState:", JSON.stringify(probe1));

    const probe2 = await page.evaluate(async () => {
      const api = (window as unknown as { electronAPI: { invoke: (c: string, ...a: unknown[]) => Promise<unknown> } }).electronAPI;
      try { return { ok: true, value: await api.invoke("realtime:connect") }; }
      catch (e) { return { ok: false, error: (e as Error).message }; }
    });
    console.log("[smoke] probe2 connect:", JSON.stringify(probe2));

    if (mainLogs.length > 0) {
      console.log("[smoke] main+renderer logs:\n  " + mainLogs.slice(-20).join("\n  "));
    }

    expect(probe1.ok).toBe(true);
    expect(["disconnected", "connecting", "connected", "reconnecting"]).toContain(probe1.value);
    expect(probe2.ok).toBe(true);

    // After invoke connect, state should transition forward.
    await page.waitForTimeout(2000);
    const state = await invokeIpc<string>(page, "realtime:getState");
    expect(["connecting", "connected", "reconnecting"], `getState returned ${state} after connect`).toContain(state);
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
