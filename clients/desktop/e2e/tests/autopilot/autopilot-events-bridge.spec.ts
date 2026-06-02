// Validate that autopilot EventBus events reach the desktop renderer via the
// IPC ServerStream bridge, and that the electron-adapter fetch path surfaces a
// live controller. The decision loop runs on the same runner as web; here we
// only assert the desktop-specific delivery (bridge + electron-adapter), which
// is a parallel TS impl with drift risk.
//
// `api` (backend Connect client) creates the target pod + controller out of
// band — desktop exposes no autopilot-create IPC, only autopilotFetchControllers.
import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";
import { invokeIpc } from "../../helpers/ipc";
import { installRealtimeSpy } from "../../helpers/realtime-spy";
import {
  createReadyAutopilotTarget,
  createAutopilotForPod,
} from "../../../../web/e2e-playwright/helpers/autopilot";

test.describe("Desktop realtime · autopilot events bridge", () => {
  test("autopilot:created reaches the renderer and the controller is fetchable", async ({ page, api }) => {
    test.setTimeout(120_000);
    const pod = await createReadyAutopilotTarget(api);

    await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
    await page.waitForTimeout(2_000); // let the Rust EventSubscriptionManager land its SubscribeRequest

    const spy = await installRealtimeSpy(page);
    try {
      const key = await createAutopilotForPod(api, {
        targetPodKey: pod.podKey,
        script: { decisions: [{ type: "continue", reasoning: "loop", send_input: "echo loop\n" }] },
        maxIterations: 20,
      });

      const created = await spy.waitFor(
        (json) => json.includes('"autopilot:created"') && json.includes(pod.podKey),
        20_000,
      );
      expect(created).toContain(pod.podKey);

      // electron-adapter fetch path returns the live controller (DTO contract).
      const raw = await invokeIpc<string>(page, "autopilotFetchControllers");
      expect(raw, `fetchControllers should include ${key}`).toContain(key);
    } finally {
      await spy.dispose();
      await pod.cleanup();
    }
  });
});
