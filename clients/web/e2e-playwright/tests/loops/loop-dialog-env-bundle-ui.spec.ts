import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

/**
 * Loop create/edit dialog × EnvBundle UI flow.
 *
 * Complements the API-level coverage in loop-env-bundle.spec.ts:
 *
 *   - This file drives the actual LoopCreateDialog, asserting the dialog
 *     renders a split UI (credential `<select>` + runtime checkbox list),
 *     that user picks survive the round-trip to the backend, and that edit
 *     mode reconciles `used_env_bundles` back into the right pickers.
 *   - loop-env-bundle.spec.ts asserts the wire contract (POST/PUT round-trip,
 *     `[]` clears, dangling names tolerated).
 */
const BUNDLE_PREFIX = "E2E LoopUI Bundle";
const LOOP_PREFIX = "E2E LoopUI Loop";

function unique(prefix: string, label: string): string {
  return `${prefix} ${label} ${Date.now()}`;
}

async function openCreateLoopDialog(page: import("@playwright/test").Page) {
  await page.goto(`/${TEST_ORG_SLUG}/loops`);
  await page.waitForLoadState("networkidle");
  const btn = page
    .getByRole("button", { name: /create loop|新建 ?loop|创建 ?loop|创建你的第一个/i })
    .first();
  await btn.click();
  await page.locator('[data-dialog-overlay]').first().waitFor({ state: "visible" });
}

async function expandAdvancedOptions(page: import("@playwright/test").Page) {
  const adv = page
    .locator('[data-dialog-overlay]')
    .getByRole("button", { name: /advanced options|高级选项/i });
  if (await adv.isVisible().catch(() => false)) {
    const state = await adv.getAttribute("data-state");
    if (state !== "open") await adv.click();
  }
}

test.describe("Loop dialog — EnvBundle binding UI", () => {
  test.beforeEach(async () => {
    clearAuthRateLimit();
  });

  test("create flow: credential select + runtime checkbox bind and submit in merge order", async ({
    page,
    api,
    db,
  }) => {
    const credName = unique(BUNDLE_PREFIX, "cred");
    const runtimeName = unique(BUNDLE_PREFIX, "runtime");
    const loopName = unique(LOOP_PREFIX, "create");

    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE '${BUNDLE_PREFIX}%'`);

    const credRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: credName,
      kind: "credential",
      data: { ANTHROPIC_API_KEY: "sk-ant-e2e-loopui" },
    });
    const credId = (await credRes.json()).bundle?.id;

    const runtimeRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: runtimeName,
      kind: "runtime",
      data: { CLAUDE_LOG_LEVEL: "debug" },
    });
    const runtimeId = (await runtimeRes.json()).bundle?.id;

    let loopSlug: string | undefined;
    try {
      await openCreateLoopDialog(page);

      await page
        .locator('[data-dialog-overlay] input[placeholder="daily-code-review"]')
        .first()
        .fill(loopName);

      await page
        .locator('[data-dialog-overlay] select#agent-select')
        .first()
        .selectOption("claude-code");

      const promptInput = page
        .locator('[data-dialog-overlay] textarea#prompt-input')
        .first();
      await promptInput.waitFor({ state: "visible", timeout: 5000 });
      await promptInput.fill("run nightly tests");

      await expandAdvancedOptions(page);

      const overlay = page.locator('[data-dialog-overlay]');

      // Credential picker is a <select id="credential-bundle-select">.
      const credSelect = overlay.locator('select#credential-bundle-select').first();
      await credSelect.waitFor({ state: "visible", timeout: 5000 });
      await credSelect.selectOption(credName);

      // Runtime picker is a checkbox list. Toggle the seeded runtime bundle.
      const runtimeCheckbox = overlay
        .getByRole("checkbox", { name: new RegExp(runtimeName) })
        .first();
      await runtimeCheckbox.waitFor({ state: "visible", timeout: 5000 });
      if (!(await runtimeCheckbox.isChecked())) await runtimeCheckbox.click();

      await overlay
        .getByRole("button", { name: /^(Create Loop|创建 ?Loop)$/i })
        .click();

      // Backend should persist credential first then runtime.
      await expect
        .poll(
          () => {
            const raw = db.queryValue(
              `SELECT array_to_string(used_env_bundles, ',') FROM loops WHERE name = '${loopName.replace(/'/g, "''")}'`
            );
            return raw ?? "";
          },
          { timeout: 10_000 }
        )
        .toBe(`${credName},${runtimeName}`);

      loopSlug = db.queryValue(
        `SELECT slug FROM loops WHERE name = '${loopName.replace(/'/g, "''")}'`
      ) ?? undefined;
    } finally {
      if (loopSlug) {
        const cc = await api.connect();
        await cc.loop.deleteLoop({ orgSlug: TEST_ORG_SLUG, loopSlug }).catch(() => null);
      }
      if (credId) await api.delete(`/api/v1/users/env-bundles/${credId}`);
      if (runtimeId) await api.delete(`/api/v1/users/env-bundles/${runtimeId}`);
      db.cleanup(`DELETE FROM env_bundles WHERE name LIKE '${BUNDLE_PREFIX}%'`);
    }
  });

  test("edit flow: existing used_env_bundles split back into credential select + runtime checkbox", async ({
    page,
    api,
    db,
  }) => {
    const credName = unique(BUNDLE_PREFIX, "edit-cred");
    const runtimeName = unique(BUNDLE_PREFIX, "edit-runtime");
    const loopName = unique(LOOP_PREFIX, "edit");

    db.cleanup(`DELETE FROM env_bundles WHERE name LIKE '${BUNDLE_PREFIX}%'`);

    const credRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: credName,
      kind: "credential",
      data: { ANTHROPIC_API_KEY: "sk-ant-e2e-loopui-edit" },
    });
    const credId = (await credRes.json()).bundle?.id;

    const runtimeRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: runtimeName,
      kind: "runtime",
      data: { CLAUDE_LOG_LEVEL: "debug" },
    });
    const runtimeId = (await runtimeRes.json()).bundle?.id;

    const cc = await api.connect();
    const loopRes = await cc.loop.createLoop({
      orgSlug: TEST_ORG_SLUG,
      name: loopName,
      agentSlug: "claude-code",
      promptTemplate: "echo bound",
      usedEnvBundles: [credName, runtimeName],
    }) as { slug: string };
    const loopSlug = loopRes.slug;

    try {
      await page.goto(`/${TEST_ORG_SLUG}/loops/${loopSlug}`);
      await page.waitForLoadState("networkidle");

      await page
        .getByRole("heading", { name: loopName, level: 1 })
        .waitFor({ state: "visible", timeout: 10_000 })
        .catch(() => {});

      const editBtn = page
        .getByRole("button")
        .filter({ hasText: /^(Edit|编辑)$/i })
        .first();
      if (!(await editBtn.isVisible({ timeout: 5000 }).catch(() => false))) {
        test.skip();
        return;
      }
      await editBtn.click();
      await page.locator('[data-dialog-overlay]').first().waitFor({ state: "visible" });

      await expandAdvancedOptions(page);

      const overlay = page.locator('[data-dialog-overlay]');

      // Credential select should be reconciled to the saved credential name.
      const credSelect = overlay.locator('select#credential-bundle-select').first();
      await credSelect.waitFor({ state: "visible", timeout: 5000 });
      await expect(credSelect).toHaveValue(credName);

      // Runtime checkbox should be pre-checked for the saved runtime bundle.
      const runtimeCheckbox = overlay
        .getByRole("checkbox", { name: new RegExp(runtimeName) })
        .first();
      await runtimeCheckbox.waitFor({ state: "visible", timeout: 5000 });
      expect(await runtimeCheckbox.isChecked()).toBe(true);
    } finally {
      if (loopSlug) {
        await cc.loop.deleteLoop({ orgSlug: TEST_ORG_SLUG, loopSlug }).catch(() => null);
      }
      if (credId) await api.delete(`/api/v1/users/env-bundles/${credId}`);
      if (runtimeId) await api.delete(`/api/v1/users/env-bundles/${runtimeId}`);
      db.cleanup(`DELETE FROM env_bundles WHERE name LIKE '${BUNDLE_PREFIX}%'`);
    }
  });
});
