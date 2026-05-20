import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { terminateAllPods } from "../../helpers/pod-cleanup";
import { clearAuthRateLimit } from "../../helpers/redis";
import { CreatePodModal } from "../../pages/modals/create-pod.modal";

/**
 * Pod creation × EnvBundle UI flow.
 *
 * The Pod create dialog renders two independent pickers inside
 * AdvancedOptions:
 *   - `<select>` for API credential (kind='credential', single-select)
 *   - checkbox list for runtime bundles (kind='runtime', ordered multi-select)
 *
 * On submit the form merges them as: credential first, then runtime in
 * selection order — one `USE_ENV_BUNDLE "..."` line per bundle in the
 * agentfile_layer.
 *
 * We don't have a persisted `pods.agentfile_layer` column — the merged
 * layer is built per-request and shipped to Runner. So we verify the wire
 * contract via Playwright route interception: the create-pod POST body
 * carries the expected agentfile_layer with the expected lines in the
 * expected order.
 */
test.describe("Pod create — EnvBundle binding UI", () => {
  test.beforeEach(async () => {
    clearAuthRateLimit();
  });

  test.afterEach(async () => {
    await terminateAllPods();
  });

  test("Pod create dialog attaches credential first then runtime in order", async ({
    page,
    api,
    db,
  }) => {
    const runnersRes = await api.get(
      `/api/v1/orgs/${TEST_ORG_SLUG}/runners/available`
    );
    const runners = (await runnersRes.json()).runners;
    if (!runners?.length) {
      test.skip();
      return;
    }
    const agentsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/agents`);
    const claudeCode = (await agentsRes.json()).builtin_agents?.find(
      (a: { slug: string }) => a.slug === "claude-code"
    );
    if (!claudeCode) {
      test.skip();
      return;
    }

    const stamp = Date.now();
    const credName = `E2E PodUI Cred ${stamp}`;
    const runtimeName = `E2E PodUI Runtime ${stamp}`;
    db.cleanup(
      `DELETE FROM env_bundles WHERE name LIKE 'E2E PodUI %'`
    );
    const credRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: credName,
      kind: "credential",
      data: { ANTHROPIC_API_KEY: "sk-ant-e2e-multi" },
    });
    expect([200, 201]).toContain(credRes.status);
    const credId = (await credRes.json()).bundle?.id;
    const runtimeRes = await api.post(`/api/v1/users/env-bundles`, {
      agent_slug: "claude-code",
      name: runtimeName,
      kind: "runtime",
      data: { CLAUDE_LOG_LEVEL: "debug" },
    });
    expect([200, 201]).toContain(runtimeRes.status);
    const runtimeId = (await runtimeRes.json()).bundle?.id;

    // Capture the agentfile_layer the form submits.
    let capturedLayer: string | undefined;
    await page.route("**/api/v1/orgs/*/pods", async (route) => {
      if (route.request().method() === "POST") {
        try {
          const body = route.request().postDataJSON() ?? {};
          if (typeof body.agentfile_layer === "string") {
            capturedLayer = body.agentfile_layer;
          }
        } catch {
          // body wasn't JSON — ignore, let the request through anyway
        }
      }
      await route.continue();
    });

    await terminateAllPods();

    try {
      await page.goto(`/${TEST_ORG_SLUG}/workspace`);
      await page.waitForLoadState("domcontentloaded");

      const newPodBtn = page
        .getByRole("button", { name: /new pod|create new pod|新建 pod/i })
        .first();
      await newPodBtn.click();

      const modal = new CreatePodModal(page);
      await modal.waitForOpen();
      await modal.selectAgent("claude-code");

      await modal.expandAdvancedOptions();

      // Credential picker is a <select id="credential-bundle-select">,
      // runtime is a checkbox list. Verify both seeded bundles surface in
      // their respective pickers before submitting.
      const dialog = page.locator('[role="dialog"]');
      const credSelect = dialog.locator('select#credential-bundle-select');
      await expect(credSelect).toBeVisible({ timeout: 10_000 });
      // Credential <option> for our seed must exist.
      const credOption = credSelect.locator('option', { hasText: credName });
      await expect(credOption).toHaveCount(1);
      // Runtime checkbox label visible.
      await expect(dialog.locator('label', { hasText: runtimeName })).toBeVisible();

      // Select credential, then runtime — merged order on submit should
      // be credential first then runtime.
      await modal.selectCredential(credName);
      await modal.selectRuntimeBundles([runtimeName]);

      await modal.submit();
      await modal.waitForClosed(15_000);

      // Two USE_ENV_BUNDLE lines: credential first, runtime after.
      const layer = capturedLayer ?? "";
      const useLines = layer
        .split("\n")
        .filter((l) => l.startsWith("USE_ENV_BUNDLE"));
      expect(useLines).toEqual([
        `USE_ENV_BUNDLE "${credName}"`,
        `USE_ENV_BUNDLE "${runtimeName}"`,
      ]);
    } finally {
      if (credId) await api.delete(`/api/v1/users/env-bundles/${credId}`);
      if (runtimeId) await api.delete(`/api/v1/users/env-bundles/${runtimeId}`);
      db.cleanup(`DELETE FROM env_bundles WHERE name LIKE 'E2E PodUI %'`);
    }
  });

  test("no-bundle selection omits USE_ENV_BUNDLE from agentfile_layer", async ({
    page,
    api,
    db,
  }) => {
    const runnersRes = await api.get(
      `/api/v1/orgs/${TEST_ORG_SLUG}/runners/available`
    );
    const runners = (await runnersRes.json()).runners;
    if (!runners?.length) {
      test.skip();
      return;
    }
    const agentsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/agents`);
    const claudeCode = (await agentsRes.json()).builtin_agents?.find(
      (a: { slug: string }) => a.slug === "claude-code"
    );
    if (!claudeCode) {
      test.skip();
      return;
    }

    // The Pod multi-select auto-checks every primary bundle, so any
    // stray primary for claude-code would flip the assertion below.
    // Purge them up-front to keep the empty-selection path testable.
    db.cleanup(
      `DELETE FROM env_bundles WHERE agent_slug = 'claude-code' AND kind_primary = TRUE`
    );

    let capturedLayer: string | undefined;
    await page.route("**/api/v1/orgs/*/pods", async (route) => {
      if (route.request().method() === "POST") {
        try {
          const body = route.request().postDataJSON() ?? {};
          if (typeof body.agentfile_layer === "string") {
            capturedLayer = body.agentfile_layer;
          } else {
            // Absent agentfile_layer also counts as "no USE_ENV_BUNDLE".
            capturedLayer = "";
          }
        } catch {
          capturedLayer = "";
        }
      }
      await route.continue();
    });

    await terminateAllPods();

    await page.goto(`/${TEST_ORG_SLUG}/workspace`);
    await page.waitForLoadState("domcontentloaded");

    const newPodBtn = page
      .getByRole("button", { name: /new pod|create new pod|新建 pod/i })
      .first();
    await newPodBtn.click();

    const modal = new CreatePodModal(page);
    await modal.waitForOpen();
    await modal.selectAgent("claude-code");
    // Leave the default empty selection alone (we purged primaries above).
    await modal.submit();
    await modal.waitForClosed(15_000);

    expect(capturedLayer ?? "").not.toContain("USE_ENV_BUNDLE");
  });
});
