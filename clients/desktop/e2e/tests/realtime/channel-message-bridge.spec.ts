// Validate that backend channel:message EventBus events reach the
// desktop renderer via the IPC ServerStream bridge.
//
// Sends a message through the channelSendMessage IPC alias (main→Connect)
// and asserts the same message arrives back at the renderer via the
// realtime stream within a few seconds.
//
// This is the desktop-side counterpart of clients/web/e2e-playwright/
// tests/mesh/channel-realtime.spec.ts (which validates wire delivery
// to a Node-side stream subscriber, not a full client).
import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { gotoHash } from "../../helpers/nav";
import { invokeIpc } from "../../helpers/ipc";
import { installRealtimeSpy } from "../../helpers/realtime-spy";

test.describe("Desktop realtime · channel:message bridge", () => {
  test("sending a message reaches the renderer via IPC bridge", async ({ page, api }) => {
    await gotoHash(page, `/${TEST_ORG_SLUG}/workspace`);
    await page.waitForTimeout(2_000); // EventSubscriptionManager bootstrap

    const spy = await installRealtimeSpy(page);
    try {
      const cc = await api.connect();
      const listed = (await cc.channel.listChannels({ orgSlug: TEST_ORG_SLUG })) as {
        items: Array<{ id: bigint | number }>;
      };

      let channelId: bigint | number;
      let createdId: bigint | number | undefined;
      if (listed.items.length) {
        channelId = listed.items[0].id;
      } else {
        const ch = (await cc.channel.createChannel({
          orgSlug: TEST_ORG_SLUG,
          name: `e2e-desktop-bridge-${Date.now().toString(36)}`,
        })) as { id: bigint | number };
        channelId = ch.id;
        createdId = ch.id;
      }

      const marker = `E2E-DESKTOP-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;
      await cc.channel.sendChannelMessage({
        orgSlug: TEST_ORG_SLUG, channelId, source: marker,
      });

      const event = await spy.waitFor(
        (json) => json.includes('"type":"channel:message"') && json.includes(marker),
        15_000,
      );
      // Validate the wire payload structure — bridge must forward the raw
      // event JSON unmodified. RealtimeEvent.data is a nested JSON object
      // (the Rust crate serializes `serde_json::Value` inline), so the
      // outer JSON.parse yields `data` as an object, not a string.
      const wire = JSON.parse(event) as { type: string; data: { body: string; channel_id: number | string } };
      expect(wire.type).toBe("channel:message");
      expect(wire.data.body).toContain(marker);
      expect(Number(wire.data.channel_id)).toBe(Number(channelId));

      if (createdId !== undefined) {
        await cc.channel.archiveChannel({ orgSlug: TEST_ORG_SLUG, id: createdId }).catch(() => undefined);
      }
    } finally {
      await spy.dispose();
    }
  });
});
