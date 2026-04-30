import { randomUUID } from "crypto";

import { test, expect, orgSlug } from "../../fixtures/blockstore.fixture";

// Guards the F4 fix: repeating an ApplyOps call with the same idempotency_key
// must return the FULL op_id list from the original batch, not just the head.
// Before the fix a 2-op batch returned 1 op_id on replay, leaving clients
// unable to distinguish "second op never ran" from "it ran but wasn't
// reported". Drop this spec and the contract regression is silent.

test("idempotent replay returns the full op_id batch", async ({ api, isolatedWorkspace }) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  const taskID = randomUUID();
  const key = `e2e-idem-${taskID}`;
  const ops = [
    {
      op: "createBlock",
      payload: {
        id: taskID,
        type: "task",
        data: { title: "idem test", status: "todo" },
        text: "idem test",
      },
    },
    {
      op: "addRef",
      payload: { from: rootID, to: taskID, rel: "nest", order_key: `zzy${Date.now().toString(36)}` },
    },
  ];

  const first = await api.post<{ op_ids: number[]; was_replay: boolean }>(
    `/api/v1/orgs/${orgSlug}/blocks/ops`,
    { workspace_id: workspaceID, ops, idempotency_key: key },
  );
  expect(first.was_replay).toBe(false);
  expect(first.op_ids).toHaveLength(2);

  const second = await api.post<{ op_ids: number[]; was_replay: boolean }>(
    `/api/v1/orgs/${orgSlug}/blocks/ops`,
    { workspace_id: workspaceID, ops, idempotency_key: key },
  );
  expect(second.was_replay).toBe(true);
  // Full batch must round-trip — identical ids in identical order.
  expect(second.op_ids).toEqual(first.op_ids);

  // And the underlying block must exist exactly once (idempotency isn't
  // faking the response — no duplicate row was inserted).
  const subtree = await api.get<{ blocks: Array<{ id: string }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`,
  );
  const matches = subtree.blocks.filter((b) => b.id === taskID);
  expect(matches).toHaveLength(1);
});
