import { BrowserWindow, ipcMain } from "electron";
import type { AppState } from "@agentsmesh/node-bridge";

// Bridges the backend EventBus realtime stream into the Electron renderer.
//
// Topology:
//   backend EventsService.Subscribe (Connect HTTP/2)
//     ↓
//   AppState.eventsSubscribeAll(jsonCallback) — Rust events crate, owned by
//   main process (single connection per AppState instance, lives until
//   AppState is replaced via server switch).
//     ↓
//   webContents.send("realtime:event", eventJson)
//     ↓
//   preload `onRealtimeEvent` listener → renderer ElectronEventsManager →
//   wasm-shaped EventSubscriptionManager → handler dispatch.
//
// Why route through main and not stream Connect directly from renderer:
// the auth token + base URL + reconnect policy already live in the Rust
// AppState. Spawning a parallel renderer-side Connect client would duplicate
// auth state, fight over org-switch races, and require shipping the Rust
// events crate via wasm (the whole reason desktop exists separately).

type RealtimeState = "disconnected" | "connecting" | "connected" | "reconnecting";

export interface RealtimeBridge {
  // Tear down the subscription, forwarder, and IPC handlers. Required
  // before swapping AppState (server-switch flow) so the old stream's
  // callback closure doesn't keep firing into a stale window.
  dispose: () => Promise<void>;
  currentState: () => RealtimeState;
}

export async function setupRealtimeBridge(
  appState: AppState,
  getMainWindow: () => BrowserWindow | null,
): Promise<RealtimeBridge> {
  let state: RealtimeState = "disconnected";

  const send = (channel: string, payload: unknown) => {
    const win = getMainWindow();
    if (!win || win.isDestroyed()) return;
    win.webContents.send(channel, payload);
  };

  // Two long-lived NAPI subscriptions — fan in from the Rust event loop into
  // a single per-window IPC stream.
  const eventSubId = await appState.eventsSubscribeAll((eventJson: string) => {
    send("realtime:event", eventJson);
  });
  const stateSubId = await appState.eventsOnConnectionStateChange((next: string) => {
    state = next as RealtimeState;
    send("realtime:state", next);
  });

  const connectHandler = async (): Promise<void> => {
    await appState.eventsConnect();
  };
  const disconnectHandler = async (): Promise<void> => {
    await appState.eventsDisconnect();
  };
  const getStateHandler = (): RealtimeState => state;

  ipcMain.handle("realtime:connect", connectHandler);
  ipcMain.handle("realtime:disconnect", disconnectHandler);
  ipcMain.handle("realtime:getState", getStateHandler);

  return {
    currentState: () => state,
    dispose: async () => {
      ipcMain.removeHandler("realtime:connect");
      ipcMain.removeHandler("realtime:disconnect");
      ipcMain.removeHandler("realtime:getState");
      try { await appState.eventsDisconnect(); } catch { /* best-effort */ }
      try { await appState.eventsUnsubscribe(eventSubId); } catch { /* best-effort */ }
      try { await appState.eventsUnsubscribe(stateSubId); } catch { /* best-effort */ }
    },
  };
}
