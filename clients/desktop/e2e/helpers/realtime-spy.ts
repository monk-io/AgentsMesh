import type { Page } from "@playwright/test";

// Spy on realtime IPC events arriving at the renderer.
//
// Desktop renderer subscribes to `realtime:event` IPC messages via the
// preload bridge (clients/desktop/src/preload/index.ts). To verify the
// main→renderer bridge works without depending on UI re-renders, we
// install a window-side listener that buffers every event JSON it
// receives and lets the test pull/wait for matches.
//
// Pattern: page.evaluate stamps a global array + onRealtimeEvent
// listener; subsequent waitForRealtimeEvent polls that array.

export interface RealtimeSpyHandle {
  // Resolves with the first event whose JSON contains `marker`, or
  // throws on timeout.
  waitFor: (predicate: (eventJson: string) => boolean, timeoutMs?: number) => Promise<string>;
  // Returns all events seen so far (snapshot).
  snapshot: () => Promise<string[]>;
  // Detach the renderer-side listener. Idempotent.
  dispose: () => Promise<void>;
}

const SPY_GLOBAL = "__realtime_spy_events__";
const SPY_UNSUB = "__realtime_spy_unsub__";

declare global {
  interface Window {
    [SPY_GLOBAL]?: string[];
    [SPY_UNSUB]?: () => void;
  }
}

// Install a renderer-side capture buffer. Safe to call multiple times —
// subsequent calls reset the buffer + re-register the listener.
export async function installRealtimeSpy(page: Page): Promise<RealtimeSpyHandle> {
  await page.evaluate(
    ({ spyGlobal, spyUnsub }) => {
      const w = window as unknown as Record<string, unknown>;
      if (typeof w[spyUnsub] === "function") {
        (w[spyUnsub] as () => void)();
      }
      w[spyGlobal] = [];
      const api = (window as unknown as { electronAPI?: { onRealtimeEvent?: (cb: (json: string) => void) => () => void } }).electronAPI;
      if (!api?.onRealtimeEvent) {
        throw new Error("electronAPI.onRealtimeEvent not exposed — preload bridge incomplete");
      }
      w[spyUnsub] = api.onRealtimeEvent((eventJson: string) => {
        const arr = w[spyGlobal] as string[];
        arr.push(eventJson);
      });
    },
    { spyGlobal: SPY_GLOBAL, spyUnsub: SPY_UNSUB },
  );

  return {
    snapshot: async () => {
      return page.evaluate((spyGlobal) => {
        const w = window as unknown as Record<string, unknown>;
        return (w[spyGlobal] as string[] | undefined)?.slice() ?? [];
      }, SPY_GLOBAL);
    },
    waitFor: async (predicate, timeoutMs = 10_000) => {
      const deadline = Date.now() + timeoutMs;
      while (Date.now() < deadline) {
        const events = await page.evaluate((spyGlobal) => {
          const w = window as unknown as Record<string, unknown>;
          return (w[spyGlobal] as string[] | undefined)?.slice() ?? [];
        }, SPY_GLOBAL);
        const match = events.find(predicate);
        if (match !== undefined) return match;
        await new Promise((r) => setTimeout(r, 100));
      }
      throw new Error(`realtime-spy: timed out after ${timeoutMs}ms waiting for matching event`);
    },
    dispose: async () => {
      await page.evaluate(
        ({ spyGlobal, spyUnsub }) => {
          const w = window as unknown as Record<string, unknown>;
          if (typeof w[spyUnsub] === "function") (w[spyUnsub] as () => void)();
          delete w[spyUnsub];
          delete w[spyGlobal];
        },
        { spyGlobal: SPY_GLOBAL, spyUnsub: SPY_UNSUB },
      ).catch(() => undefined);
    },
  };
}
