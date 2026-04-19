/**
 * EventSubscriptionManager — Electron IPC implementation.
 *
 * WebSocket runs in Electron main process (eventsConnect/eventsDisconnect via node-bridge).
 * JS receives events via Electron IPC events (entity:event, events:connection_state).
 */

import { useAuthStore } from "@/stores/auth";
import type {
  EventType, EventHandler, RealtimeEvent, ConnectionState,
} from "./types";

type UnlistenFn = () => void;

async function invoke(channel: string, ...args: unknown[]): Promise<unknown> {
  const api = (globalThis as any).window?.electronAPI;
  if (!api) return null;
  return api.invoke(channel, ...args);
}

function listenIpc<T>(channel: string, cb: (payload: T) => void): UnlistenFn {
  const api = (globalThis as any).window?.electronAPI;
  if (!api?.on) return () => {};
  return api.on(channel, (_e: unknown, payload: T) => cb(payload));
}

export interface EventSubscriptionManagerOptions {
  onConnectionStateChange?: (state: ConnectionState) => void;
}

export class EventSubscriptionManager {
  private connectionState: ConnectionState = "disconnected";
  private handlers: Map<EventType, Set<EventHandler>> = new Map();
  private globalHandlers: Set<EventHandler> = new Set();
  private connectionStateListeners: Set<(state: ConnectionState) => void> = new Set();
  private unlistenEvent: UnlistenFn | null = null;
  private unlistenState: UnlistenFn | null = null;

  constructor(options: EventSubscriptionManagerOptions = {}) {
    if (options.onConnectionStateChange) {
      this.connectionStateListeners.add(options.onConnectionStateChange);
    }
  }

  private setConnectionState(state: ConnectionState): void {
    if (this.connectionState !== state) {
      this.connectionState = state;
      this.connectionStateListeners.forEach((listener) => {
        try { listener(state); }
        catch (error) { console.error("[EventSubscriptionManager] Listener error:", error); }
      });
    }
  }

  async connect(): Promise<void> {
    if (this.connectionState === "connected" || this.connectionState === "connecting") return;

    const { currentOrg } = useAuthStore.getState();
    if (!currentOrg) {
      console.warn("[EventSubscriptionManager] Cannot connect: no org");
      return;
    }

    this.setConnectionState("connecting");

    this.unlistenEvent = listenIpc<RealtimeEvent>("entity:event", (ev) => this.handleMessage(ev));
    this.unlistenState = listenIpc<{ state: ConnectionState }>("events:connection_state", (p) => {
      this.setConnectionState(p.state);
    });

    try {
      await invoke("eventsConnect", currentOrg.slug);
    } catch (error) {
      console.error("[EventSubscriptionManager] Connect failed:", error);
      this.setConnectionState("disconnected");
    }
  }

  async disconnect(): Promise<void> {
    this.unlistenEvent?.();
    this.unlistenState?.();
    this.unlistenEvent = null;
    this.unlistenState = null;

    try { await invoke("eventsDisconnect"); } catch { /* ignore */ }
    this.setConnectionState("disconnected");
  }

  subscribe<T = unknown>(eventType: EventType, handler: EventHandler<T>): () => void {
    if (!this.handlers.has(eventType)) this.handlers.set(eventType, new Set());
    this.handlers.get(eventType)!.add(handler as EventHandler);
    return () => { this.handlers.get(eventType)?.delete(handler as EventHandler); };
  }

  subscribeAll(handler: EventHandler): () => void {
    this.globalHandlers.add(handler);
    return () => { this.globalHandlers.delete(handler); };
  }

  getConnectionState(): ConnectionState { return this.connectionState; }

  onConnectionStateChange(listener: (state: ConnectionState) => void): () => void {
    this.connectionStateListeners.add(listener);
    listener(this.connectionState);
    return () => { this.connectionStateListeners.delete(listener); };
  }

  private handleMessage(event: RealtimeEvent): void {
    if (event.type === "pong") return;
    this.handlers.get(event.type)?.forEach((handler) => {
      try { handler(event); }
      catch (error) { console.error(`[EventSubscriptionManager] Handler error for ${event.type}:`, error); }
    });
    this.globalHandlers.forEach((handler) => {
      try { handler(event); }
      catch (error) { console.error("[EventSubscriptionManager] Global handler error:", error); }
    });
  }
}
