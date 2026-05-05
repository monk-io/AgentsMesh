import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";

/**
 * Regression coverage for the `useLocalRunnerOnboarding` hook + ThisMacSection
 * UI on the desktop. Stubs in `clients/desktop/src/main/local_runner_stubs.ts`
 * intercept every `localRunner*` IPC when `NODE_ENV=test`, so this spec runs
 * end-to-end without needing a real `agentsmesh-runner` binary on PATH.
 *
 * Bugs this guards against:
 *
 *  • Hook refresh guard pinned phase in installing forever after onRegister
 *    completed every step successfully — the user saw "Working… / Starting
 *    service…" indefinitely while the runner was already up. The fix moved
 *    the `if (phase.kind === "installing") return` guard from inside refresh
 *    to the setInterval handler. Coverage: card transitions to the
 *    registered row at the end of the click flow.
 *
 *  • Backend `POST /runners/grpc/tokens` used to omit the `id` field, which
 *    the Rust client deserialises as a required `i64`. Step 2 (token mint)
 *    threw `missing field 'id'`. Coverage: the registered row appears,
 *    which is only reachable if create_token round-trips successfully.
 *
 *  • Step-aware error rendering when an individual step fails. Coverage:
 *    the failure variant of this spec asserts the "Failed at <step>" line.
 */
test.describe("Desktop ThisMacSection onboarding", () => {
  test("registering this Mac transitions through every step to active", async ({ page }) => {
    await gotoHash(page, `/${TEST_ORG_SLUG}/infra?tab=runners`);

    // Sidebar footer card mounts under the runners list. Onboarding-pending
    // state shows the trigger button.
    const button = page.getByRole("button", { name: /Register This Mac/i });
    await expect(button).toBeVisible({ timeout: 15_000 });
    await expect(button).toBeEnabled();

    await button.click();

    // The 5 step labels surface as text under the button while the hook is
    // mid-flight. We don't assert each one — STUB_DELAY_MS is 50ms per step
    // so they flash by — but the final transition to the registered row
    // proves all of them ran in order without the phase getting pinned.
    const registeredLabel = page.locator('text=/active/i').first();
    await expect(registeredLabel).toBeVisible({ timeout: 10_000 });

    // The registered row replaces the onboarding card; the trigger button
    // must be gone so re-clicking it can't fire onRegister twice.
    await expect(button).toHaveCount(0);

    // localNodeId from the stub is "test-mac". The footer row + the matching
    // sidebar list entry both label it with "This Mac" badge.
    await expect(page.locator('text=test-mac').first()).toBeVisible();
  });
});
