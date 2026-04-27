import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { terminateAllPods } from "../../helpers/pod-cleanup";
import { CreatePodModal } from "../../pages/modals/create-pod.modal";

/**
 * UI regression for: CreatePod dialog must auto-close on success and the new
 * pod must appear in the workspace sidebar.
 *
 * The bug: `useCreatePodFormSubmit.ts` was reading `response.pod` from the
 * Rust pod-service result, but Rust already returns the unwrapped Pod, so
 * `submitCreatePod()` always returned `null`. `onSuccess` never fired,
 * dialog never closed, user re-clicked, second submit hit runner capacity
 * and 503'd. Pure HTTP `pod-create.api.spec.ts` did not catch this — the
 * 200 came back fine; the bug was in how the renderer interpreted the
 * payload.
 */
test.describe("CreatePod dialog UI", () => {
  test.afterEach(async () => {
    await terminateAllPods();
  });

  test("dialog auto-closes and new pod appears in sidebar", async ({ page, api }) => {
    // Pre-condition: at least one runner online + one agent available.
    const runnersRes = await api.get(
      `/api/v1/orgs/${TEST_ORG_SLUG}/runners/available`
    );
    const runners = (await runnersRes.json()).runners;
    if (!runners?.length) { test.skip(); return; }

    const agentsRes = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/agents`);
    const agents = (await agentsRes.json()).builtin_agents;
    if (!agents?.length) { test.skip(); return; }

    // Start clean so the sidebar count is deterministic.
    await terminateAllPods();

    await page.goto(`/${TEST_ORG_SLUG}/workspace`);
    await page.waitForLoadState("domcontentloaded");

    // pod_key format: `<org_id>-(standalone|ticket|channel)-<8 hex>` —
    // matches PodListItem display in the sidebar. Count before submit so
    // the assertion can detect the +1 delta.
    const podRowRegex = /^1-(standalone|ticket|channel)-[a-f0-9]{8}/;
    const before = await page.getByText(podRowRegex).count();

    // Trigger via "New Pod" — same handler whether shown in sidebar
    // (WorkspaceSidebarContent) or empty-state CTA (WorkspaceEmptyState).
    const newPodBtn = page
      .getByRole("button", { name: /new pod|create new pod|新建 pod/i })
      .first();
    await newPodBtn.click();

    const modal = new CreatePodModal(page);
    await modal.waitForOpen();
    await modal.selectAgent(agents[0].slug);
    await modal.submit();

    // Regression assertion #1: dialog must close after onSuccess fires.
    // Pre-fix this timed out — the dialog stayed open indefinitely.
    await modal.waitForClosed(15_000);

    // Regression assertion #2: new pod appears in the sidebar.
    await expect
      .poll(async () => page.getByText(podRowRegex).count(), { timeout: 10_000 })
      .toBeGreaterThan(before);
  });
});
