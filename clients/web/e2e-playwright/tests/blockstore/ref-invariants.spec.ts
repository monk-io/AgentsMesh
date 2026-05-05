import { randomUUID } from "crypto";

import { test, expect, orgSlug, apiBase } from "../../fixtures/blockstore.fixture";

// Ref-graph invariants — single nest parent + cycle prevention — are
// end-to-end contracts the frontend relies on. Violations must surface as
// 409 so clients can distinguish structural conflicts from server bugs.
// Covered by backend integration tests; this adds REST-layer evidence that
// the status code mapping is wired through translateErr correctly.

async function postOps(
  token: string,
  workspaceID: string,
  ops: unknown[],
  idempotencyKey: string,
): Promise<Response> {
  return fetch(`${apiBase}/api/v1/orgs/${orgSlug}/blocks/ops`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "X-Organization-Slug": orgSlug,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ workspace_id: workspaceID, ops, idempotency_key: idempotencyKey }),
  });
}

test("addRef(nest) to a block with an existing nest parent → 409", async ({
  api,
  token,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID: root } = isolatedWorkspace;
  const parent = randomUUID();
  const child = randomUUID();
  // Setup: child nested under root.
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      { op: "createBlock", payload: { id: parent, type: "list", data: { name: "p" }, text: "p" } },
      {
        op: "addRef",
        payload: { from: root, to: parent, rel: "nest", order_key: `pa${Date.now()}` },
      },
      { op: "createBlock", payload: { id: child, type: "task", data: { title: "c" }, text: "c" } },
      {
        op: "addRef",
        payload: { from: parent, to: child, rel: "nest", order_key: `ch${Date.now()}` },
      },
    ],
    idempotency_key: `e2e-single-parent-setup-${child}`,
  });

  // Attempt: a second nest parent for `child` → must fail with 409.
  const res = await postOps(
    token,
    workspaceID,
    [
      {
        op: "addRef",
        payload: { from: root, to: child, rel: "nest", order_key: `bad${Date.now()}` },
      },
    ],
    `e2e-single-parent-bad-${child}`,
  );
  expect(res.status).toBe(409);
});

test("addRef(nest) creating a cycle → 409", async ({ api, token, isolatedWorkspace }) => {
  const { id: workspaceID, rootID: root } = isolatedWorkspace;
  const a = randomUUID();
  const b = randomUUID();
  // Setup: A contains B.
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      { op: "createBlock", payload: { id: a, type: "list", data: { name: "A" }, text: "A" } },
      { op: "addRef", payload: { from: root, to: a, rel: "nest", order_key: `ca${Date.now()}` } },
      { op: "createBlock", payload: { id: b, type: "list", data: { name: "B" }, text: "B" } },
      { op: "addRef", payload: { from: a, to: b, rel: "nest", order_key: `cb${Date.now()}` } },
    ],
    idempotency_key: `e2e-cycle-setup-${a}`,
  });

  // Independent pair to isolate the cycle check from single-parent guard.
  const c = randomUUID();
  const d = randomUUID();
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      { op: "createBlock", payload: { id: c, type: "list", data: { name: "C" }, text: "C" } },
      { op: "createBlock", payload: { id: d, type: "list", data: { name: "D" }, text: "D" } },
      { op: "addRef", payload: { from: c, to: d, rel: "nest", order_key: `cd${Date.now()}` } },
    ],
    idempotency_key: `e2e-cycle-pair-${c}`,
  });
  // D → C would form a cycle (C → D → C). Must be rejected.
  const res = await postOps(
    token,
    workspaceID,
    [{ op: "addRef", payload: { from: d, to: c, rel: "nest", order_key: `dc${Date.now()}` } }],
    `e2e-cycle-bad-${c}`,
  );
  expect(res.status).toBe(409);
});
