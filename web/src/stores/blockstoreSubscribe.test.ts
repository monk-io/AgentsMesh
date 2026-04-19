import { beforeEach, describe, expect, it, vi } from "vitest";

import { useBlockstoreStore } from "./blockstore";
import { handleBlockstoreEvent } from "./blockstoreSubscribe";

// F5 guard: handleBlockstoreEvent must drop ops whose workspace_id was not
// seeded into lastOpId via loadSubtree. The org-wide WS broadcast delivers
// ops from every workspace the user's org hosts; filtering here is the
// line that keeps an unrelated workspace's data out of this client's store.

describe("handleBlockstoreEvent workspace-scope filter (F5)", () => {
  beforeEach(() => {
    useBlockstoreStore.getState().actions.reset();
  });

  it("applies an op for a subscribed workspace", () => {
    const wsID = "ws-subscribed";
    // Seed the marker as loadSubtree would.
    useBlockstoreStore.setState({ lastOpId: { [wsID]: 0 } });

    handleBlockstoreEvent({
      type: "blockstore:op",
      data: {
        id: 1,
        workspace_id: wsID,
        actor_type: "system",
        actor_id: 1,
        op: "createBlock",
        payload: {},
        forward: {
          id: "block-1",
          type: "paragraph",
          data: { text: "hello" },
        },
        inverse: {},
        applied_at: "2026-04-18T00:00:00Z",
      },
    } as never);

    const blocks = useBlockstoreStore.getState().blocks;
    expect(Object.keys(blocks)).toContain("block-1");
  });

  it("drops an op for an unsubscribed workspace", () => {
    const subscribed = "ws-subscribed";
    const other = "ws-other";
    useBlockstoreStore.setState({ lastOpId: { [subscribed]: 0 } });

    handleBlockstoreEvent({
      type: "blockstore:op",
      data: {
        id: 2,
        workspace_id: other, // not in lastOpId
        actor_type: "system",
        actor_id: 1,
        op: "createBlock",
        payload: {},
        forward: {
          id: "leak-block",
          type: "paragraph",
          data: { text: "should not land" },
        },
        inverse: {},
        applied_at: "2026-04-18T00:00:00Z",
      },
    } as never);

    const blocks = useBlockstoreStore.getState().blocks;
    expect(blocks["leak-block"], "op from unsubscribed workspace must be dropped").toBeUndefined();
  });

  it("drops an op when the user has no subscriptions at all (cold boot)", () => {
    useBlockstoreStore.setState({ lastOpId: {} });

    handleBlockstoreEvent({
      type: "blockstore:op",
      data: {
        id: 3,
        workspace_id: "any-workspace",
        actor_type: "system",
        actor_id: 1,
        op: "createBlock",
        payload: {},
        forward: { id: "pre-hydrate", type: "paragraph", data: {} },
        inverse: {},
        applied_at: "2026-04-18T00:00:00Z",
      },
    } as never);

    expect(useBlockstoreStore.getState().blocks["pre-hydrate"]).toBeUndefined();
  });

  it("ignores non-blockstore event types", () => {
    useBlockstoreStore.setState({ lastOpId: { "ws-subscribed": 0 } });
    const applySpy = vi.spyOn(useBlockstoreStore.getState().actions, "applyRemoteOp");
    handleBlockstoreEvent({ type: "pod:created", data: {} } as never);
    expect(applySpy).not.toHaveBeenCalled();
    applySpy.mockRestore();
  });
});
