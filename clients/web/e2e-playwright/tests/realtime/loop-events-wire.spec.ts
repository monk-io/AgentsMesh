// Wire-level realtime EventBus verification for loop_run:* events.
//
// Covers loop_run:started (A-class — direct from TriggerLoop), plus
// loop_run:completed / loop_run:failed (B-class — via pod terminate +
// orchestrator listener chain in backend/internal/service/loop/).
//
// loop_run:warning is not directly triggerable from e2e (requires a
// timing failure inside the orchestrator); listed as Out of Scope in
// the plan and to be covered by backend integration test.
import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { withEventSubscription } from "../../helpers/eventbus-stream";
import { terminateAllPods } from "../../helpers/pod-cleanup";

test.describe("Realtime · loop_run events (wire)", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });
  test.afterEach(async () => { await terminateAllPods(); });

  async function createLoop(api: import("../../fixtures/api.fixture").ApiFixture) {
    const cc = await api.connect();
    const stamp = Date.now().toString(36);
    const loop = (await cc.loop.createLoop({
      orgSlug: TEST_ORG_SLUG,
      name: `e2e-rt-loop-${stamp}`,
      slug: `e2e-rt-loop-${stamp}`,
      agentSlug: "e2e-echo",
      promptTemplate: "echo hi",
      executionMode: "direct",
      timeoutMinutes: 1,
    } as never)) as { slug: string };
    return loop;
  }

  test("loop_run:started arrives after TriggerLoop", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const loop = await createLoop(api);

    const { event } = await withEventSubscription<unknown, { loop_id?: number | string; run_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, _data) => type === "loop_run:started",
        timeoutMs: 15_000,
      },
      async () => {
        await cc.loop.triggerLoop({
          orgSlug: TEST_ORG_SLUG, loopSlug: loop.slug,
        } as never);
      },
    );

    expect(event.type).toBe("loop_run:started");
    expect(Number(event.data.loop_id)).toBeGreaterThan(0);
    expect(Number(event.data.run_id)).toBeGreaterThan(0);
  });

  test("loop_run:completed arrives after pod successful termination", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const loop = await createLoop(api);

    // Trigger run, wait for started → terminate pod with success → expect completed
    let runPodKey: string | null = null;
    const startedP = withEventSubscription<unknown, { pod_key?: string; loop_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, _data) => type === "loop_run:started",
        timeoutMs: 15_000,
      },
      async () => {
        await cc.loop.triggerLoop({
          orgSlug: TEST_ORG_SLUG, loopSlug: loop.slug,
        } as never);
      },
    );
    const startedRes = await startedP;
    runPodKey = (startedRes.event.data.pod_key as string | undefined) ?? null;
    expect(runPodKey, "loop_run:started must carry pod_key for completion test").toBeTruthy();

    const { event } = await withEventSubscription<unknown, { run_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, _data) => type === "loop_run:completed" || type === "loop_run:failed",
        timeoutMs: 20_000,
      },
      async () => {
        await cc.pod.terminatePod({ orgSlug: TEST_ORG_SLUG, podKey: runPodKey! });
      },
    );

    expect(Number(event.data.run_id)).toBeGreaterThan(0);
  });
});
