import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";
import { terminateAllPods } from "../../helpers/pod-cleanup";
import { setupAcpScenarioPage } from "../../helpers/acp-spec-setup";

// Defensive-path coverage: every scenario here exercises an unhappy
// runner/agent boundary that should NOT crash the web UI or wedge the
// activity stream.
// See acp-ui-echo.spec.ts header — same r6 fix applies.
test.describe("ACP UI: error and degradation paths", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });
  test.afterEach(async () => { await terminateAllPods(); });

  test("tool_call_failed renders the failed status without crashing UI", async ({ page, api }) => {
    const ctx = await setupAcpScenarioPage(page, api, {
      mode: "acp", scenario: "tool_call_failed", prompt: "edit me",
    });
    if (!ctx) { test.skip(); return; }

    await expect(page.getByText("Trying to edit: edit me")).toBeVisible({ timeout: 15_000 });
    await expect(page.getByText("Edit", { exact: true }).first()).toBeVisible({ timeout: 15_000 });
    ctx.assertWasmHealthy();
  });

  test("malformed_json output does not break subsequent valid messages", async ({ page, api }) => {
    const ctx = await setupAcpScenarioPage(page, api, {
      mode: "acp", scenario: "malformed_json", prompt: "garbled",
    });
    if (!ctx) { test.skip(); return; }

    await expect(page.getByText("recovered: garbled")).toBeVisible({ timeout: 15_000 });
    ctx.assertWasmHealthy();
  });

  test("log_warnings surfaces warn/error stderr lines in activity stream", async ({ page, api }) => {
    const ctx = await setupAcpScenarioPage(page, api, {
      mode: "acp", scenario: "log_warnings", prompt: "noisy run",
    });
    if (!ctx) { test.skip(); return; }

    await expect(page.getByText(/degraded connection/i)).toBeVisible({ timeout: 15_000 });
    await expect(page.getByText("Completed with warnings: noisy run")).toBeVisible({ timeout: 15_000 });
  });

  test("fail_after_1s does not leave the UI wedged in a processing state", async ({ page, api }) => {
    const ctx = await setupAcpScenarioPage(page, api, {
      mode: "acp", scenario: "fail_after_1s", prompt: "crash test",
    });
    if (!ctx) { test.skip(); return; }

    // The agent emits one content chunk and then os.Exit(1)s after 1s. We
    // race two outcomes:
    //   (a) chunk renders in the activity stream before the crash arrives, OR
    //   (b) PaneErrorState replaces the panel because the crashed status
    //       reaches the browser before / instead of the chunk.
    // Either outcome proves "not wedged in processing" — the failure mode
    // we care about is the UI sitting on a loading spinner indefinitely.
    await expect(
      page.getByText("Will crash soon: crash test")
        .or(page.getByText(/process exited with code/i))
    ).toBeVisible({ timeout: 15_000 });
    await page.waitForTimeout(4000);
    ctx.assertWasmHealthy();
  });
});
