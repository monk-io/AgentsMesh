import { test, expect } from "../../fixtures";
import { invokeIpc } from "../../helpers/ipc";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";

/**
 * Desktop terminal DATA PLANE round-trip after the relay-SSOT migration.
 *
 * Desktop's relay path differs from web: the Rust RelayConnectionPool runs in
 * the MAIN process (node-bridge), PTY bytes are coalesced and pushed to the
 * renderer over the `relay:*` IPC bridge (clients/desktop/src/main/relay.ts),
 * and ElectronRelayManager re-fans the single per-pod stream to the xterm
 * subscriber. This proves that whole chain end-to-end:
 *   xterm ↔ relayConnection adapter ↔ ElectronRelayManager ↔ IPC ↔ main Rust
 *   pool ↔ relay WS ↔ runner PTY ↔ e2e-echo (pty mode).
 *
 * pty_runtime.go writes "ready" on spawn, then echoes each stdin line as
 * "got: <line>". The e2e-echo agent (migration 000151) defaults to pty mode.
 */
test.describe("Desktop terminal round-trip (relay SSOT)", () => {
  test("attaches, streams pty output, and round-trips typed input via the main-process pool", async ({ page }) => {
    const runners = await invokeIpc<string>(page, "runnerFetchRunners");
    const runnerList = JSON.parse(runners) as { runners?: { id: number; status: string }[] } | { id: number; status: string }[];
    const onlineRunner = (Array.isArray(runnerList) ? runnerList : runnerList.runners ?? [])
      .find((r) => r.status === "online");
    expect(onlineRunner, "dev env must have an online runner").toBeTruthy();

    // pty is the e2e-echo default mode — no agentfile layer needed.
    const created = await invokeIpc<string>(page, "podCreatePod", JSON.stringify({
      agent_slug: "e2e-echo",
      runner_id: onlineRunner!.id,
      cols: 120,
      rows: 32,
    }));
    const pod = (JSON.parse(created) as { pod: { pod_key: string } }).pod;
    expect(pod.pod_key, "podCreatePod returned a pod_key").toBeTruthy();

    // Let the pty pod reach "running" before the workspace reload's
    // fetchSidebarPods snapshots it — usePodStatus reuses that cached status
    // (it only self-fetches when the pod is absent), so a stale "initializing"
    // would keep AgentPanel from subscribing the terminal.
    await page.waitForTimeout(6000);

    try {
      await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
      await page.reload();
      await page.waitForLoadState("domcontentloaded");
      await invokeIpc(page, "authBootstrap");
      await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);

      const sidebarPod = page.locator(
        `[data-testid="pod-list-item"][data-pod-key="${pod.pod_key}"]`,
      );
      await expect(sidebarPod, "new pod must appear in sidebar").toBeVisible({ timeout: 30_000 });
      await sidebarPod.click();

      // OUTPUT: the terminal pane self-fetches pod status (usePodStatus) and
      // subscribes once running; the daemon replays the buffered "ready" on
      // attach. Generous window covers fetch + realtime status flip + subscribe.
      const term = page.locator(".xterm");
      await expect(term, "pty 'ready' must stream through the main-process pool to xterm").toContainText(
        "ready",
        { timeout: 45_000 },
      );

      // INPUT: typed line → ElectronRelayManager → IPC → main pool → relay → PTY.
      await page.locator(".xterm-helper-textarea").focus();
      await page.keyboard.type("relay-roundtrip");
      await page.keyboard.press("Enter");

      await expect(term, "pty echo must round-trip back through the bridge to xterm").toContainText(
        "got: relay-roundtrip",
        { timeout: 20_000 },
      );
    } finally {
      await invokeIpc<void>(page, "podTerminatePod", pod.pod_key).catch(() => undefined);
    }
  });
});
