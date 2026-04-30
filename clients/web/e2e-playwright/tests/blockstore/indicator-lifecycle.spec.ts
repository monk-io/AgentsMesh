import { test, expect, orgSlug } from "../../fixtures/blockstore.fixture";

// Tier 1 闭环 E2E: Agent defines an indicator via MCP → indicator appears as a
// new option in the slash menu → clicking it creates a typed record → server
// persists it with the right type_key → RecordEditor renders the schema's
// columns (select options + text input). The canonical "is definition 2
// working end-to-end" check; a failure pinpoints which seam broke.
test("indicator.define → slash menu → RecordEditor lifecycle", async ({
  authenticatedPage,
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID } = isolatedWorkspace;
  const typeKey = `e2e_bug_${Date.now()}`;
  const uniqueLabel = `Bug Report ${typeKey.slice(-10)}`;

  // Surface page-level errors so a silently-thrown insertChild doesn't masquerade
  // as a "missing record" timeout further down the test. We filter out generic
  // "Failed to load resource" noise since those can come from unrelated
  // workspace state (e.g. previously-created private blocks the ACL now
  // correctly rejects); only real runtime errors should fail the test.
  const consoleErrors: string[] = [];
  authenticatedPage.on("console", (msg) => {
    if (msg.type() !== "error") return;
    const text = msg.text();
    if (/Failed to load resource/.test(text)) return;
    consoleErrors.push(text);
  });
  authenticatedPage.on("pageerror", (err) => {
    consoleErrors.push(`pageerror: ${err.message}`);
  });

  // 1. Define the indicator by writing a block_type_def directly via the
  //    REST ops endpoint. (The /blocks/mcp/* REST shim was removed — agents
  //    now reach this path through the Runner MCP gRPC bridge; this test
  //    simulates the final backend state the bridge produces.)
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          type: "block_type_def",
          data: {
            type_key: typeKey,
            label: uniqueLabel,
            description: "A defect observed in production",
            revision: 1,
            default_view: "kanban",
            columns: [
              { key: "title", type: "text", required: true, label: "Title" },
              {
                key: "severity",
                type: "select",
                required: true,
                options: [
                  { value: "P0" },
                  { value: "P1" },
                  { value: "P2" },
                  { value: "P3" },
                ],
              },
              { key: "fixed", type: "boolean", default: false },
            ],
          },
          text: typeKey,
        },
      },
    ],
    idempotency_key: `e2e-indicator-define-${typeKey}`,
  });

  // Backend should expose the new type in its type-defs endpoint.
  const typeDefs = await api.get<{ blocks: Array<{ data: { type_key: string } }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/type-defs`,
  );
  expect(typeDefs.blocks.some((b) => b.data.type_key === typeKey)).toBe(true);

  // 2. Open blocks page. The "+ Add block" menu should list our new type.
  await authenticatedPage.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);
  const addBlockBtn = authenticatedPage.getByRole("button", { name: "+ Add block" });
  await expect(addBlockBtn).toBeVisible({ timeout: 15_000 });
  await addBlockBtn.click();

  const bugOption = authenticatedPage.getByRole("button", { name: new RegExp(uniqueLabel) });
  await expect(bugOption).toBeVisible();
  // Arm the response watcher BEFORE clicking so we catch the POST synchronously
  // rather than racing against the next render cycle.
  const opsPromise = authenticatedPage.waitForResponse(
    (r) => r.url().includes("/blocks/ops") && r.request().method() === "POST",
    { timeout: 15_000 },
  );
  await bugOption.click();
  const opsRes = await opsPromise;
  expect(opsRes.status(), `ApplyOps failed: ${await opsRes.text()}`).toBeLessThan(300);

  // 3. Wait for the record to show up in the subtree. Querying via API is more
  // deterministic than DOM polling because WS propagation has its own timing.
  const rootID = await rootBlockID(api, workspaceID);
  const subtreePath = `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`;
  await expect
    .poll(
      async () => {
        const res = await api.get<{ blocks: Array<{ type: string }> }>(subtreePath);
        return res.blocks.filter((b) => b.type === typeKey).length;
      },
      { timeout: 10_000, message: `record of type ${typeKey} should appear in subtree` },
    )
    .toBeGreaterThan(0);

  expect(consoleErrors, consoleErrors.join("\n")).toEqual([]);

  // 4. RecordEditor renders the schema. Severity has a <select> with P0..P3
  // — the uniqueLabel chip is our anchor since it renders once per record.
  const typeChip = authenticatedPage.getByText(uniqueLabel).last();
  await expect(typeChip).toBeVisible({ timeout: 10_000 });
  await typeChip.scrollIntoViewIfNeeded();
  const bugEditor = typeChip.locator("xpath=ancestor::*[starts-with(@id, 'block-')]");
  const severitySelect = bugEditor.locator("select").first();
  await expect(severitySelect).toBeVisible();
  const severityOptions = await severitySelect.locator("option").allTextContents();
  expect(severityOptions).toEqual(expect.arrayContaining(["P0", "P1", "P2", "P3"]));

  const titleInput = bugEditor.locator('input[type="text"]').first();
  await titleInput.fill("Checkout total wrong when coupon applied");
  await authenticatedPage.waitForTimeout(800); // updateBlockData debounce round-trip

  // 5. Confirm the record data persisted via API.
  const after = await api.get<{ blocks: Array<{ type: string; data: Record<string, unknown> }> }>(
    subtreePath,
  );
  const records = after.blocks.filter((b) => b.type === typeKey);
  expect(records.length).toBeGreaterThan(0);
  expect(records[records.length - 1].data.severity).toBe("P0");
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
