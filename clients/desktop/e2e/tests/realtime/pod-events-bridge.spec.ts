// Validate that backend pod EventBus events reach the desktop renderer
// via the IPC ServerStream bridge.
//
// Pre-bridge: NoopEventsManager swallowed every event; sidebar never
// updated on pod create/terminate without manual reload. This spec runs
// only after the Phase 0 bridge lands.
//
// Approach: install a renderer-side spy, then create + terminate a pod
// through the IPC bridge (which calls into main → Connect-RPC → backend).
// Backend publishes `pod:created` / `pod:terminated` events; the bridge
// forwards each via webContents.send → preload onRealtimeEvent → spy.
import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";
import { invokeIpc } from "../../helpers/ipc";
import { installRealtimeSpy } from "../../helpers/realtime-spy";

test.describe("Desktop realtime · pod events bridge", () => {
  test("pod:created + pod:terminated reach the renderer via IPC", async ({ page }) => {
    await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
    // EventSubscriptionManager.connect() runs after wasm bootstrap — give
    // the underlying Rust EventSubscriptionManager time to land its
    // SubscribeRequest with the backend hub before publishing.
    await page.waitForTimeout(2_000);

    const spy = await installRealtimeSpy(page);
    try {
      // Prerequisites identical to pod-realtime.spec.ts's pattern.
      const runners = await invokeIpc<string>(page, "runnerFetchRunners");
      const runnerList = JSON.parse(runners) as { runners?: { id: number; status: string }[] };
      const onlineRunner = (runnerList.runners ?? []).find((r) => r.status === "online");
      expect(onlineRunner, "dev env must have an online runner").toBeTruthy();

      const agentsJson = await invokeIpc<string>(page, "agentListAgents");
      const agents = JSON.parse(agentsJson) as { builtin_agents?: { slug: string }[] };
      const agent = agents.builtin_agents?.[0];
      expect(agent, "dev env must have a builtin agent").toBeTruthy();

      // Create a pod and assert pod:created reaches the renderer via bridge.
      const created = await invokeIpc<string>(page, "podCreatePod", JSON.stringify({
        agent_slug: agent!.slug, runner_id: onlineRunner!.id, cols: 80, rows: 24,
      }));
      const { pod } = JSON.parse(created) as { pod: { pod_key: string } };
      expect(pod.pod_key, "podCreatePod returned a pod_key").toBeTruthy();

      try {
        const createdEvent = await spy.waitFor(
          (json) => json.includes('"type":"pod:created"') && json.includes(pod.pod_key),
          15_000,
        );
        const wireCreated = JSON.parse(createdEvent) as { data: { pod_key: string } };
        expect(wireCreated.data.pod_key).toBe(pod.pod_key);

        // Terminate and assert pod:terminated also lands.
        await invokeIpc(page, "podTerminatePod", pod.pod_key);
        const terminatedEvent = await spy.waitFor(
          (json) => json.includes('"type":"pod:terminated"') && json.includes(pod.pod_key),
          15_000,
        );
        const wireTerminated = JSON.parse(terminatedEvent) as { data: { pod_key: string } };
        expect(wireTerminated.data.pod_key).toBe(pod.pod_key);
      } finally {
        await invokeIpc(page, "podTerminatePod", pod.pod_key).catch(() => undefined);
      }
    } finally {
      await spy.dispose();
    }
  });
});
