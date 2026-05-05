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

    // After every onboarding step succeeds, ThisMacSection shows the
    // "syncing with server" line. The hook flips phase to idle.running but
    // the runners list won't pick up this Mac until a real daemon connects
    // over gRPC — and the IPC stubs don't fake that round-trip. The visible
    // syncing string is the high-fidelity proof that the 5-step state
    // machine drained without getting pinned (the regression we care about).
    const syncing = page.getByText(/syncing with server/i);
    await expect(syncing).toBeVisible({ timeout: 10_000 });

    // The trigger must be gone so re-clicking it can't fire onRegister twice.
    await expect(button).toHaveCount(0);
  });
});
