import { randomUUID } from "crypto";

import { test, expect, orgSlug } from "./fixtures";

// Verifies the WebSocket realtime path end-to-end:
//   - Two browser pages open the /blocks page for the same workspace
//   - Page A creates a paragraph block via API
//   - Page B's DOM eventually reflects the new block, proving the WS
//     broadcast + opApply pipeline works across tabs (and by extension,
//     across browsers, machines, etc.)
// The test drives setup via API rather than UI so a failure pinpoints the
// realtime layer, not the create-flow UI.
test("multi-tab realtime sync: paragraph created in API shows up in both tabs", async ({
  browser,
  token,
  isolatedWorkspace,
  api,
}) => {
  const { id: workspaceID } = isolatedWorkspace;
  const ctxA = await browser.newContext();
  const ctxB = await browser.newContext();
  try {
    for (const ctx of [ctxA, ctxB]) {
      await ctx.addInitScript(
        ({ tok, ws }) => {
          window.localStorage.setItem(
            "agentsmesh-auth",
            JSON.stringify({
              state: {
                token: tok,
                refreshToken: null,
                user: { id: 1, email: "dev@agentsmesh.local", username: "devuser", name: "Dev User" },
                currentOrg: { id: 1, slug: "dev-org", name: "Dev Organization", role: "owner" },
                organizations: [
                  { id: 1, slug: "dev-org", name: "Dev Organization", role: "owner" },
                ],
              },
              version: 0,
            }),
          );
          void ws;
        },
        { tok: token, ws: workspaceID },
      );
    }

    const pageA = await ctxA.newPage();
    const pageB = await ctxB.newPage();
    await pageA.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);
    await pageB.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);

    // Wait for both pages to finish initial hydration (the "+ Add block"
    // button only appears once the default workspace subtree loads).
    await expect(pageA.getByRole("button", { name: "+ Add block" })).toBeVisible({
      timeout: 15_000,
    });
    await expect(pageB.getByRole("button", { name: "+ Add block" })).toBeVisible({
      timeout: 15_000,
    });

    // Fetch the root block id so we can nest the new paragraph.
    const workspaces = await api.get<{ workspaces: Array<{ id: string; root_block_id: string }> }>(
      `/api/v1/orgs/${orgSlug}/blocks/workspaces`,
    );
    const ws = workspaces.workspaces.find((w) => w.id === workspaceID)!;
    const rootID = ws.root_block_id;

    // Create a uniquely-named paragraph via API — both tabs should see it.
    const marker = `E2E-SYNC-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    const newID = randomUUID();
    await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
      workspace_id: workspaceID,
      ops: [
        {
          op: "createBlock",
          payload: {
            id: newID,
            type: "paragraph",
            data: { text: marker },
            text: marker,
          },
        },
        {
          op: "addRef",
          payload: {
            from: rootID,
            to: newID,
            rel: "nest",
            // Must stay inside BASE_CHARS (0-9, a-z) so fractionalIndex can
            // generate subsequent keys after this paragraph. A `-` would be
            // sortable but outside the base set and would break indexOfChar.
            order_key: `zzz${Date.now().toString(36)}`,
          },
        },
      ],
      idempotency_key: `e2e-sync-${newID}`,
    });

    // Both tabs should render the paragraph text within a few seconds.
    await expect(pageA.getByText(marker)).toBeVisible({ timeout: 10_000 });
    await expect(pageB.getByText(marker)).toBeVisible({ timeout: 10_000 });
  } finally {
    await ctxA.close();
    await ctxB.close();
  }
});
