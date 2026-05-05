import { test, expect, orgSlug } from "../../fixtures/blockstore.fixture";

// Chart block smoke test: confirms the full add-chart flow is wired.
// - Slash menu shows the "Chart / Bar" entry
// - Clicking it dispatches a createBlock op for type=chart with bar seed data
// - The backend persists the block with data.type === "bar" and a non-empty
//   series array (matches ChartPreview's expectations)
// - The DOM lands a chart block id, so BlockRenderer dispatched to ChartRenderer
test("Chart / Bar option creates a chart block and renders it", async ({
  authenticatedPage,
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  await authenticatedPage.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);
  const addBtn = authenticatedPage.getByRole("button", { name: "+ Add block" });
  await expect(addBtn).toBeVisible({ timeout: 15_000 });
  await addBtn.click();

  const barOption = authenticatedPage.getByRole("button", { name: "Chart / Bar" });
  await expect(barOption).toBeVisible();
  const opsPromise = authenticatedPage.waitForResponse(
    (r) => r.url().includes("/blocks/ops") && r.request().method() === "POST",
    { timeout: 15_000 },
  );
  await barOption.click();
  const opsRes = await opsPromise;
  expect(opsRes.status(), `ApplyOps failed: ${await opsRes.text()}`).toBeLessThan(300);

  const subtreePath = `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/subtree?root=${rootID}`;

  // Poll the subtree for a freshly-minted chart block (backend may lag WS).
  const latest = await pollForLatestChart(api, subtreePath);
  expect(latest.data.type).toBe("bar");
  expect(Array.isArray(latest.data.series)).toBe(true);
  expect((latest.data.series as unknown[]).length).toBeGreaterThan(0);

  // ChartRenderer wraps the preview in a block-scoped container so we should
  // see the stored title in the DOM (seed uses "Sample bar chart").
  await expect(authenticatedPage.getByText(/Sample bar chart/).last()).toBeVisible({
    timeout: 10_000,
  });
});

interface ChartBlock {
  id: string;
  type: string;
  data: Record<string, unknown>;
  updated_at?: string;
}

async function pollForLatestChart(
  api: { get<T>(path: string): Promise<T> },
  subtreePath: string,
): Promise<ChartBlock> {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const res = await api.get<{ blocks: ChartBlock[] }>(subtreePath);
    const charts = res.blocks.filter((b) => b.type === "chart");
    if (charts.length > 0) return charts[charts.length - 1];
    await new Promise((r) => setTimeout(r, 200));
  }
  throw new Error("chart block did not appear in subtree within 10s");
}

async function rootBlockID(
  api: { get<T>(path: string): Promise<T> },
  workspaceID: string,
): Promise<string> {
  const res = await api.get<{ workspaces: Array<{ id: string; root_block_id: string }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces`,
  );
  return res.workspaces.find((w) => w.id === workspaceID)!.root_block_id;
}
