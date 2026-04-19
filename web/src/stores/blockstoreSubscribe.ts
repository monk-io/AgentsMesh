import { reconnectRegistry } from "@/lib/realtime";
import type { RealtimeEvent } from "@/lib/realtime";
import type { BlockOp } from "@/lib/api/blockstoreTypes";
import { useBlockstoreStore } from "./blockstore";

// handleBlockstoreEvent is invoked by RealtimeProvider for every
// `blockstore:*` event received over the org-wide WebSocket.
//
// Scope guard: the backend fans out ops across the whole organization, so
// users connected to the same org see each other's workspace ops on the wire.
// We drop ops for workspaces the user has not loaded — otherwise block data
// from a workspace the user has no UI access to would still land in the
// in-memory store. The gate is "is this workspace one the user has
// subscribed to via loadSubtree or catchup", which we track via the
// lastOpId map (loadSubtree seeds an entry so the workspace becomes
// subscribed regardless of whether any op has arrived yet).
export function handleBlockstoreEvent(event: RealtimeEvent) {
  if (event.type !== "blockstore:op") return;
  const op = event.data as BlockOp;
  const state = useBlockstoreStore.getState();
  if (!(op.workspace_id in state.lastOpId)) return;
  state.actions.applyRemoteOp(op);
}

// Register on module load so every consumer of the store gets catch-up for free
// after a WebSocket reconnect. Catch-up is per-workspace; we iterate over
// whatever lastOpId entries currently exist in the store.
reconnectRegistry.register({
  name: "blockstore:catchup",
  fn: () => {
    const { lastOpId, actions } = useBlockstoreStore.getState();
    const workspaces = Object.keys(lastOpId);
    workspaces.forEach((wsID) => {
      void actions.catchup(wsID);
    });
  },
  priority: "deferred",
});
