import { beforeEach, describe, expect, it } from "vitest";

import type { BlockOp } from "@/lib/api/blockstoreTypes";

import { useBlockstoreStore } from "./blockstore";

// Seeds a minimal state so the remote op pipeline has something to mutate.
const WS = "ws-00000000-0000-0000-0000-000000000000";
const ROOT = "00000000-0000-0000-0000-000000000001";
const CHILD = "00000000-0000-0000-0000-000000000002";

function op(partial: Partial<BlockOp>): BlockOp {
  return {
    id: 1,
    workspace_id: WS,
    actor_type: "user",
    actor_id: 1,
    op: "createBlock",
    payload: {},
    forward: {},
    inverse: {},
    applied_at: new Date().toISOString(),
    ...partial,
  } as BlockOp;
}

describe("blockstore regression", () => {
  beforeEach(() => {
    useBlockstoreStore.getState().actions.reset();
  });

  it("indexRef is idempotent — duplicate inserts do not re-add to nestChildren", () => {
    const actions = useBlockstoreStore.getState().actions;
    // Seed both blocks so the ref refers to real rows.
    actions.upsertBlock({
      id: ROOT, workspace_id: WS, type: "page", data: {}, meta: {},
      created_by: 1, created_at: "", updated_at: "",
    });
    actions.upsertBlock({
      id: CHILD, workspace_id: WS, type: "paragraph", data: {}, meta: {},
      created_by: 1, created_at: "", updated_at: "",
    });

    const ref = {
      id: 42, workspace_id: WS, from_id: ROOT, to_id: CHILD, rel: "nest",
      order_key: "m", anchor: null, meta: {}, created_by: 1,
      created_at: "", updated_at: "",
    };
    actions.upsertRef(ref);
    actions.upsertRef(ref);
    actions.upsertRef(ref);

    const children = useBlockstoreStore.getState().nestChildren[ROOT];
    expect(children).toEqual([42]);
  });

  it("applyRemoteOp createBlock → updateBlock → deleteBlock round-trip", () => {
    const actions = useBlockstoreStore.getState().actions;
    actions.applyRemoteOp(op({
      id: 1, op: "createBlock",
      target_block: CHILD,
      forward: { id: CHILD, type: "paragraph", data: { text: "hello" }, meta: {} },
    }));
    expect(useBlockstoreStore.getState().blocks[CHILD]?.data.text).toBe("hello");

    actions.applyRemoteOp(op({
      id: 2, op: "updateBlock",
      target_block: CHILD,
      forward: { id: CHILD, data: { text: "edited" } },
    }));
    expect(useBlockstoreStore.getState().blocks[CHILD]?.data.text).toBe("edited");

    actions.applyRemoteOp(op({
      id: 3, op: "deleteBlock",
      target_block: CHILD,
      forward: { id: CHILD },
    }));
    expect(useBlockstoreStore.getState().blocks[CHILD]).toBeUndefined();
  });

  it("lastOpId advances monotonically", () => {
    const actions = useBlockstoreStore.getState().actions;
    actions.applyRemoteOp(op({ id: 5, op: "createBlock", target_block: CHILD, forward: { id: CHILD, type: "paragraph", data: {}, meta: {} } }));
    expect(useBlockstoreStore.getState().lastOpId[WS]).toBe(5);
    actions.applyRemoteOp(op({ id: 8, op: "updateBlock", forward: { id: CHILD, data: { text: "x" } } }));
    expect(useBlockstoreStore.getState().lastOpId[WS]).toBe(8);
    // An out-of-order older op must not rewind the marker.
    actions.applyRemoteOp(op({ id: 3, op: "updateBlock", forward: { id: CHILD, data: { text: "stale" } } }));
    expect(useBlockstoreStore.getState().lastOpId[WS]).toBe(8);
  });

  it("requestFocus / clearPendingFocus manage a one-shot signal", () => {
    const actions = useBlockstoreStore.getState().actions;
    actions.requestFocus(CHILD);
    expect(useBlockstoreStore.getState().pendingFocusBlockID).toBe(CHILD);
    actions.clearPendingFocus();
    expect(useBlockstoreStore.getState().pendingFocusBlockID).toBeNull();
  });

  it("addRef op writes updated_at equal to applied_at (B2)", () => {
    const actions = useBlockstoreStore.getState().actions;
    const ts = "2026-04-17T10:00:00.000Z";
    actions.applyRemoteOp(op({
      id: 10, op: "addRef", applied_at: ts, target_ref: 99,
      forward: {
        id: 99, from: ROOT, to: CHILD, rel: "nest", order_key: "m",
        meta: { source: "agent" },
      },
    }));
    const ref = useBlockstoreStore.getState().refs[99];
    expect(ref?.updated_at).toBe(ts);
    expect(ref?.meta).toEqual({ source: "agent" }); // B1 round-trip
  });

  it("updateRef op refreshes updated_at and meta (B2 + R2)", () => {
    const actions = useBlockstoreStore.getState().actions;
    actions.applyRemoteOp(op({
      id: 1, op: "addRef", applied_at: "2026-04-17T10:00:00.000Z",
      forward: { id: 7, from: ROOT, to: CHILD, rel: "nest", order_key: "a" },
    }));
    actions.applyRemoteOp(op({
      id: 2, op: "updateRef", applied_at: "2026-04-17T10:05:00.000Z",
      forward: { ref_id: 7, order_key: "b", meta: { resolved: true } },
    }));
    const ref = useBlockstoreStore.getState().refs[7];
    expect(ref?.order_key).toBe("b");
    expect(ref?.meta).toEqual({ resolved: true });
    expect(ref?.updated_at).toBe("2026-04-17T10:05:00.000Z");
  });

  it("removeBlock cascades refs + clears indexes (R1)", () => {
    const actions = useBlockstoreStore.getState().actions;
    actions.upsertBlock({
      id: ROOT, workspace_id: WS, type: "page", data: {}, meta: {},
      created_by: 1, created_at: "", updated_at: "",
    });
    actions.upsertBlock({
      id: CHILD, workspace_id: WS, type: "paragraph", data: {}, meta: {},
      created_by: 1, created_at: "", updated_at: "",
    });
    actions.upsertRef({
      id: 10, workspace_id: WS, from_id: ROOT, to_id: CHILD, rel: "nest",
      order_key: "m", anchor: null, meta: {}, created_by: 1,
      created_at: "", updated_at: "",
    });
    actions.upsertRef({
      id: 11, workspace_id: WS, from_id: CHILD, to_id: ROOT, rel: "mention",
      order_key: null, anchor: null, meta: {}, created_by: 1,
      created_at: "", updated_at: "",
    });
    // Sanity: indexes populated.
    expect(useBlockstoreStore.getState().nestChildren[ROOT]).toEqual([10]);
    expect(useBlockstoreStore.getState().backlinks[ROOT]).toEqual([11]);

    actions.removeBlock(CHILD);

    const state = useBlockstoreStore.getState();
    expect(state.blocks[CHILD]).toBeUndefined();
    // Both refs (nest + mention touching CHILD) are gone.
    expect(state.refs[10]).toBeUndefined();
    expect(state.refs[11]).toBeUndefined();
    // Indexes are empty — no ghost entries.
    expect(state.nestChildren[ROOT]).toBeUndefined();
    expect(state.backlinks[ROOT]).toBeUndefined();
    expect(state.nestChildren[CHILD]).toBeUndefined();
    expect(state.backlinks[CHILD]).toBeUndefined();
  });
});
