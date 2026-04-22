import { randomUUID } from "crypto";

import { test, expect, orgSlug } from "./fixtures";

// Kanban view smoke test. One view-layout spec covers the shared plumbing
// (useViewBlocks, groupBlocks, ViewRenderer dispatch) that table/timeline/
// tree/gallery all rely on — a break in any of those surfaces here.
// Drag-drop between columns is intentionally out of scope; dnd-kit pointer
// simulation is fragile and the semantic move op is already covered in
// block-crud.spec.ts.

test("kanban view groups tasks by status and renders per-column add buttons", async ({
  authenticatedPage,
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  const viewID = randomUUID();
  const todoTaskID = randomUUID();
  const doneTaskID = randomUUID();
  const todoTitle = `kanban-todo-${Date.now()}`;
  const doneTitle = `kanban-done-${Date.now()}`;

  // Seed: one view block, two tasks (one per status), each nested under root.
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          id: viewID,
          type: "view",
          data: {
            source_type: "task",
            layout: "kanban",
            group_by: "status",
            title: `Kanban probe ${Date.now()}`,
          },
          text: "kanban probe",
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: viewID, rel: "nest", order_key: `kv${Date.now()}` },
      },
      {
        op: "createBlock",
        payload: {
          id: todoTaskID,
          type: "task",
          data: { title: todoTitle, status: "todo" },
          text: todoTitle,
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: todoTaskID, rel: "nest", order_key: `kt1${Date.now()}` },
      },
      {
        op: "createBlock",
        payload: {
          id: doneTaskID,
          type: "task",
          data: { title: doneTitle, status: "done" },
          text: doneTitle,
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: doneTaskID, rel: "nest", order_key: `kt2${Date.now()}` },
      },
    ],
    idempotency_key: `e2e-kanban-seed-${viewID}`,
  });

  await authenticatedPage.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);

  // The view renders a container with "todo"/"done" column headers and a
  // `+ status:todo` add button per column (from the KanbanView source).
  await expect(authenticatedPage.getByRole("button", { name: "+ status:todo" }).last()).toBeVisible({
    timeout: 15_000,
  });
  await expect(authenticatedPage.getByRole("button", { name: "+ status:done" }).last()).toBeVisible();

  // Each task must appear at least once. `last()` guards against the
  // pre-existing test-data pollution accumulated across prior E2E runs.
  await expect(authenticatedPage.getByText(todoTitle).last()).toBeVisible();
  await expect(authenticatedPage.getByText(doneTitle).last()).toBeVisible();

  // Clicking "+ status:todo" in this view adds a task with status=todo.
  // Wait for the applyOps POST that carries the new createBlock.
  const addPromise = authenticatedPage.waitForResponse(
    (r) => r.url().includes("/blocks/ops") && r.request().method() === "POST",
    { timeout: 10_000 },
  );
  // Target the last-rendered `+ status:todo` button — the newly created
  // kanban is guaranteed to be last in document order among prior e2e
  // detritus.
  await authenticatedPage.getByRole("button", { name: "+ status:todo" }).last().click();
  const addRes = await addPromise;
  expect(addRes.status()).toBeLessThan(300);
});

async function rootBlockID(
  api: { get<T>(path: string): Promise<T> },
  workspaceID: string,
): Promise<string> {
  const res = await api.get<{ workspaces: Array<{ id: string; root_block_id: string }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces`,
  );
  return res.workspaces.find((w) => w.id === workspaceID)!.root_block_id;
}
