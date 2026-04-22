import { randomUUID } from "crypto";

import { test, expect, orgSlug } from "./fixtures";

// Core CRUD op coverage. createBlock gets exercised in every other spec; the
// rest (updateBlock, deleteBlock, updateRef a.k.a. moveChild) don't. Each has
// subtle contracts (soft-delete, stale-write rejection, nest parent swap)
// worth end-to-end evidence — a regression anywhere shows here as a subtree
// shape mismatch.

async function getSubtreeBlock(
  api: { get<T>(path: string): Promise<T> },
  workspaceID: string,
  rootID: string,
  blockID: string,
): Promise<{ data: Record<string, unknown>; text: string | null } | undefined> {
  const res = await api.get<{
    blocks: Array<{ id: string; data: Record<string, unknown>; text: string | null }>;
  }>(`/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`);
  return res.blocks.find((b) => b.id === blockID);
}

async function createTask(
  api: { post<T>(path: string, body: unknown): Promise<T> },
  workspaceID: string,
  rootID: string,
  title: string,
): Promise<string> {
  const id = randomUUID();
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          id,
          type: "task",
          data: { title, status: "todo" },
          text: title,
        },
      },
      {
        op: "addRef",
        payload: {
          from: rootID,
          to: id,
          rel: "nest",
          order_key: `zzy${Date.now().toString(36)}${Math.random().toString(36).slice(2, 5)}`,
        },
      },
    ],
    idempotency_key: `e2e-crud-seed-${id}`,
  });
  return id;
}

test("updateBlock round-trips new data and text", async ({ api, isolatedWorkspace }) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  const taskID = await createTask(api, workspaceID, rootID, "before-update");

  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "updateBlock",
        payload: {
          id: taskID,
          data: { title: "after-update", status: "done" },
          text: "after-update",
        },
      },
    ],
    idempotency_key: `e2e-crud-update-${taskID}`,
  });

  const after = await getSubtreeBlock(api, workspaceID, rootID, taskID);
  expect(after, "block should still exist after update").toBeDefined();
  expect(after!.data.title).toBe("after-update");
  expect(after!.data.status).toBe("done");
  expect(after!.text).toBe("after-update");
});

test("deleteBlock removes the block from subtree queries", async ({ api, isolatedWorkspace }) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  const taskID = await createTask(api, workspaceID, rootID, "soft-delete-me");

  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [{ op: "deleteBlock", payload: { id: taskID } }],
    idempotency_key: `e2e-crud-delete-${taskID}`,
  });

  const after = await getSubtreeBlock(api, workspaceID, rootID, taskID);
  expect(after, "soft-deleted block should not surface in subtree").toBeUndefined();
});

test("updateRef moves a block under a new parent", async ({ api, isolatedWorkspace }) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  // Two containers A and B both nested under root; a leaf starts under A
  // and then gets moved under B via updateRef.
  const containerA = randomUUID();
  const containerB = randomUUID();
  const leaf = randomUUID();

  const setupRes = await api.post<{ op_ids: number[] }>(
    `/api/v1/orgs/${orgSlug}/blocks/ops`,
    {
      workspace_id: workspaceID,
      ops: [
        {
          op: "createBlock",
          payload: { id: containerA, type: "list", data: { name: "A" }, text: "A" },
        },
        {
          op: "addRef",
          payload: { from: rootID, to: containerA, rel: "nest", order_key: `zzxa${Date.now()}` },
        },
        {
          op: "createBlock",
          payload: { id: containerB, type: "list", data: { name: "B" }, text: "B" },
        },
        {
          op: "addRef",
          payload: { from: rootID, to: containerB, rel: "nest", order_key: `zzxb${Date.now()}` },
        },
        {
          op: "createBlock",
          payload: { id: leaf, type: "task", data: { title: "moving", status: "todo" }, text: "moving" },
        },
        {
          op: "addRef",
          payload: { from: containerA, to: leaf, rel: "nest", order_key: `aa${Date.now()}` },
        },
      ],
      idempotency_key: `e2e-crud-move-setup-${leaf}`,
    },
  );
  expect(setupRes.op_ids.length).toBe(6);

  // Find the ref id connecting containerA → leaf via the subtree response
  // (the /children endpoint returns Blocks, not Refs — no ref_id exposed).
  const subtree = await api.get<{
    refs: Array<{ id: number; from_id: string; to_id: string; rel: string }>;
  }>(`/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`);
  const leafRef = subtree.refs.find(
    (r) => r.rel === "nest" && r.from_id === containerA && r.to_id === leaf,
  );
  expect(leafRef, "leaf should be attached under containerA").toBeDefined();

  // Move: updateRef switches from=A to from=B with a new order_key.
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "updateRef",
        payload: { ref_id: leafRef!.id, from: containerB, order_key: `bb${Date.now()}` },
      },
    ],
    idempotency_key: `e2e-crud-move-${leaf}`,
  });

  const after = await api.get<{
    refs: Array<{ id: number; from_id: string; to_id: string; rel: string }>;
  }>(`/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`);
  const leafRefAfter = after.refs.find(
    (r) => r.rel === "nest" && r.to_id === leaf,
  );
  expect(leafRefAfter, "moved ref should still exist").toBeDefined();
  expect(leafRefAfter!.from_id).toBe(containerB);
});
