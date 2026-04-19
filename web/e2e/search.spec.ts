import { randomUUID } from "crypto";

import { test, expect, orgSlug } from "./fixtures";

// Verifies the semantic search path end-to-end inside a pristine workspace:
//   1. Create two paragraph blocks with very different content
//   2. Wait for async embeddings (HashEmbedder in dev) to flush
//   3. Search for distinctive text in each — expect the right block to rank
//      above the other.
//
// Using `isolatedWorkspace` means the corpus contains only our two blocks,
// so top_k=10 is plenty and rank-based assertions stay stable. The earlier
// version of this spec shared the default workspace and accumulated
// hundreds of stale kiwi/rocket blocks over many runs, eventually pushing
// our freshly-written block out of the top hits.
test("semantic search ranks paragraphs by query relevance", async ({
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;

  const kiwiMarker = `kiwi-fruit-unique-${Date.now()}`;
  const rocketMarker = `rocket-engine-unique-${Date.now()}`;
  const kiwiID = randomUUID();
  const rocketID = randomUUID();

  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          id: kiwiID,
          type: "paragraph",
          data: { text: kiwiMarker },
          text: `${kiwiMarker} ripe green fruit with fuzzy skin from new zealand orchard`,
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: kiwiID, rel: "nest", order_key: `zzz${Date.now().toString(36)}1` },
      },
      {
        op: "createBlock",
        payload: {
          id: rocketID,
          type: "paragraph",
          data: { text: rocketMarker },
          text: `${rocketMarker} liquid hydrogen thrust chamber combustion nozzle stage`,
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: rocketID, rel: "nest", order_key: `zzz${Date.now().toString(36)}2` },
      },
    ],
    idempotency_key: `e2e-search-${kiwiID}`,
  });

  // Wait until both IDs are indexed. Embeddings ride a 256-buffered channel;
  // on an idle dev stack that normally flushes within a couple of ticks.
  await expect
    .poll(
      async () => {
        const res = await api.post<{ hits: Array<{ block_id: string }> }>(
          `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/search`,
          { query: "kiwi fruit orchard zealand", top_k: 20 },
        );
        return res.hits.some((h) => h.block_id === kiwiID);
      },
      { timeout: 15_000, message: "kiwi block should be indexed" },
    )
    .toBe(true);

  // Ranking check: the fruit query should surface kiwi strictly above rocket.
  const kiwiHits = await api.post<{ hits: Array<{ block_id: string; score: number }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/search`,
    { query: "kiwi fruit orchard zealand", top_k: 10 },
  );
  const kiwiRank = kiwiHits.hits.findIndex((h) => h.block_id === kiwiID);
  const rocketRankInKiwiQuery = kiwiHits.hits.findIndex((h) => h.block_id === rocketID);
  expect(kiwiRank).toBeGreaterThanOrEqual(0);
  if (rocketRankInKiwiQuery >= 0) expect(kiwiRank).toBeLessThan(rocketRankInKiwiQuery);

  const rocketHits = await api.post<{ hits: Array<{ block_id: string; score: number }> }>(
    `/api/v1/orgs/${orgSlug}/blocks/workspaces/${workspaceID}/search`,
    { query: "rocket thrust combustion nozzle engine", top_k: 10 },
  );
  const rocketRank = rocketHits.hits.findIndex((h) => h.block_id === rocketID);
  const kiwiRankInRocketQuery = rocketHits.hits.findIndex((h) => h.block_id === kiwiID);
  expect(rocketRank).toBeGreaterThanOrEqual(0);
  if (kiwiRankInRocketQuery >= 0) expect(rocketRank).toBeLessThan(kiwiRankInRocketQuery);
});
