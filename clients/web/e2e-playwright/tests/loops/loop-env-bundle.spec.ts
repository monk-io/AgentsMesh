import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

/**
 * Loop ↔ EnvBundle binding end-to-end.
 *
 * Covers the I4 contract: creating a Loop with `usedEnvBundles = ["<name>", ...]`
 * persists the ordered list, GetLoop round-trips it, UpdateLoop clears it via
 * empty list, and unknown-bundle names still create the Loop (eval is warn-only
 * at run-time).
 *
 * EnvBundle CRUD is still REST (`/api/v1/users/env-bundles`); Loop CRUD has
 * fully migrated to Connect-RPC (`proto.loop.v1.LoopService/*`).
 *
 * Pod-level KV injection is left to higher-tier integration tests since it
 * requires a Pod to actually launch and read its env.
 */
test.describe("Loop ↔ EnvBundle binding", () => {
  test.beforeEach(async () => {
    clearAuthRateLimit();
  });

  test("Loop persists usedEnvBundles (multi) and round-trips on GetLoop", async ({ api }) => {
    const cc = await api.connect();
    const ts = Date.now();
    const bundleAName = `e2e-loop-A-${ts}`;
    const bundleBName = `e2e-loop-B-${ts}`;

    const createBundle = async (name: string) =>
      api.post(`/api/v1/users/env-bundles`, {
        agent_slug: "claude-code",
        name,
        kind: "credential",
        data: { ANTHROPIC_API_KEY: "sk-test-e2e" },
      });

    const bundleAId = (await (await createBundle(bundleAName)).json()).bundle?.id;
    const bundleBId = (await (await createBundle(bundleBName)).json()).bundle?.id;
    expect(bundleAId).toBeTruthy();
    expect(bundleBId).toBeTruthy();

    let slug: string | undefined;
    try {
      const created = await cc.loop.createLoop({
        orgSlug: TEST_ORG_SLUG,
        name: `E2E Loop Bundle ${ts}`,
        agentSlug: "claude-code",
        promptTemplate: "echo bound",
        usedEnvBundles: [bundleAName, bundleBName],
      }) as { slug: string; usedEnvBundles: string[] };
      slug = created.slug;
      expect(slug).toBeTruthy();
      // Order preserved exactly.
      expect(created.usedEnvBundles).toEqual([bundleAName, bundleBName]);

      const fetched = await cc.loop.getLoop({
        orgSlug: TEST_ORG_SLUG,
        loopSlug: slug,
      }) as { usedEnvBundles: string[] };
      expect(fetched.usedEnvBundles).toEqual([bundleAName, bundleBName]);
    } finally {
      if (slug) {
        await cc.loop.deleteLoop({ orgSlug: TEST_ORG_SLUG, loopSlug: slug }).catch(() => null);
      }
      await api.delete(`/api/v1/users/env-bundles/${bundleAId}`);
      await api.delete(`/api/v1/users/env-bundles/${bundleBId}`);
    }
  });

  test("UpdateLoop with usedEnvBundles={names:[]} clears the binding", async ({ api }) => {
    const cc = await api.connect();
    const ts = Date.now();
    const bundleName = `e2e-clear-${ts}`;

    const bundleRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: bundleName,
      kind: "credential",
      data: { ANTHROPIC_API_KEY: "sk-test-e2e-clear" },
    });
    expect([200, 201]).toContain(bundleRes.status);
    const bundleId = (await bundleRes.json()).bundle?.id;

    let slug: string | undefined;
    try {
      const created = await cc.loop.createLoop({
        orgSlug: TEST_ORG_SLUG,
        name: `E2E Loop Clear ${ts}`,
        agentSlug: "claude-code",
        promptTemplate: "echo bound",
        usedEnvBundles: [bundleName],
      }) as { slug: string };
      slug = created.slug;
      expect(slug).toBeTruthy();

      // Wrapper present with empty `names` explicitly clears the binding.
      await cc.loop.updateLoop({
        orgSlug: TEST_ORG_SLUG,
        loopSlug: slug,
        usedEnvBundles: { names: [] },
      });

      const after = await cc.loop.getLoop({
        orgSlug: TEST_ORG_SLUG,
        loopSlug: slug,
      }) as { usedEnvBundles: string[] };
      // Backend returns [] (not null) for an empty array column.
      expect(after.usedEnvBundles).toEqual([]);
    } finally {
      if (slug) {
        await cc.loop.deleteLoop({ orgSlug: TEST_ORG_SLUG, loopSlug: slug }).catch(() => null);
      }
      await api.delete(`/api/v1/users/env-bundles/${bundleId}`);
    }
  });

  test("Loop with unknown bundle name is still creatable (warn-only at run-time)", async ({ api }) => {
    const cc = await api.connect();
    const ts = Date.now();
    // Use a name we know does NOT exist; the AgentFile eval contract is
    // tolerant of dangling references (USE_ENV_BUNDLE skips silently when
    // the name isn't in ctx.EnvBundles).
    const created = await cc.loop.createLoop({
      orgSlug: TEST_ORG_SLUG,
      name: `E2E Loop Dangling ${ts}`,
      agentSlug: "claude-code",
      promptTemplate: "echo dangling",
      usedEnvBundles: [`nonexistent-bundle-${ts}`],
    }) as { slug: string };
    const slug = created.slug;
    expect(slug).toBeTruthy();
    await cc.loop.deleteLoop({ orgSlug: TEST_ORG_SLUG, loopSlug: slug }).catch(() => null);
  });
});
