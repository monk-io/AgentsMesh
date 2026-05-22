import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

/**
 * Loop ↔ EnvBundle binding end-to-end.
 *
 * Covers the I4 contract: creating a Loop with `used_env_bundles = ["<name>", ...]`
 * persists the ordered list, GET round-trips it, PUT clears it via empty array,
 * and unknown-bundle names still create the Loop (eval is warn-only at run-time).
 *
 * Pod-level KV injection is left to higher-tier integration tests since it
 * requires a Pod to actually launch and read its env.
 */
test.describe("Loop ↔ EnvBundle binding", () => {
  test.beforeEach(async () => {
    clearAuthRateLimit();
  });

  test("Loop persists used_env_bundles (multi) and round-trips on GET", async ({ api }) => {
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

    try {
      const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
        name: `E2E Loop Bundle ${ts}`,
        agent_slug: "claude-code",
        prompt_template: "echo bound",
        used_env_bundles: [bundleAName, bundleBName],
      });
      expect([200, 201]).toContain(createRes.status);
      const created = await createRes.json();
      const slug = created.loop?.slug;
      expect(slug).toBeTruthy();
      // Order preserved exactly.
      expect(created.loop?.used_env_bundles).toEqual([bundleAName, bundleBName]);

      const getRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`);
      expect(getRes.status).toBe(200);
      const fetched = await getRes.json();
      expect(fetched.loop?.used_env_bundles).toEqual([bundleAName, bundleBName]);

      await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`);
    } finally {
      await api.delete(`/api/v1/users/env-bundles/${bundleAId}`);
      await api.delete(`/api/v1/users/env-bundles/${bundleBId}`);
    }
  });

  test("PUT with used_env_bundles=[] clears the binding", async ({ api }) => {
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
      const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
        name: `E2E Loop Clear ${ts}`,
        agent_slug: "claude-code",
        prompt_template: "echo bound",
        used_env_bundles: [bundleName],
      });
      expect([200, 201]).toContain(createRes.status);
      slug = (await createRes.json()).loop?.slug;
      expect(slug).toBeTruthy();

      // Empty array explicitly clears the binding.
      const updateRes = await api.put(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`, {
        used_env_bundles: [],
      });
      expect(updateRes.status).toBe(200);

      const after = await (await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`)).json();
      // Backend returns [] (not null) for an empty array column.
      expect(after.loop?.used_env_bundles).toEqual([]);
    } finally {
      if (slug) await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`);
      await api.delete(`/api/v1/users/env-bundles/${bundleId}`);
    }
  });

  test("Loop with unknown bundle name is still creatable (warn-only at run-time)", async ({ api }) => {
    const ts = Date.now();
    // Use a name we know does NOT exist; the AgentFile eval contract is
    // tolerant of dangling references (USE_ENV_BUNDLE skips silently when
    // the name isn't in ctx.EnvBundles).
    const createRes = await api.post(`/api/v1/orgs/${TEST_ORG_SLUG}/loops`, {
      name: `E2E Loop Dangling ${ts}`,
      agent_slug: "claude-code",
      prompt_template: "echo dangling",
      used_env_bundles: [`nonexistent-bundle-${ts}`],
    });
    expect([200, 201]).toContain(createRes.status);
    const slug = (await createRes.json()).loop?.slug;
    expect(slug).toBeTruthy();
    await api.delete(`/api/v1/orgs/${TEST_ORG_SLUG}/loops/${slug}`);
  });
});
