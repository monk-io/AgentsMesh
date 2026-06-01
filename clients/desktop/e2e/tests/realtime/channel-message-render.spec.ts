// Desktop has no in-process Rust: the renderer mirrors a main-pushed
// runtime.state snapshot (D4+D5). This asserts a channel message posted from a
// SECOND context RENDERS live in the open channel's message list — the gap
// channel-message-bridge (IPC wiretap only) cannot catch. D1's handler
// slimming broke this on desktop until the snapshot-push → renderer mirror
// restored it.
import { test, expect } from "../../fixtures";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { ChannelsPage } from "../../pages/channels.page";

test.describe("Desktop realtime · channel message renders live", () => {
  test("message from another context appears in the open channel list", async ({ page, api }) => {
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
        name: `e2e-desktop-render-${Date.now().toString(36)}`,
      })) as { id: bigint | number };
      channelId = ch.id;
      createdId = ch.id;
    }

    const channels = new ChannelsPage(page);
    await channels.openChannel(String(channelId));
    await expect(channels.messageInput).toBeVisible({ timeout: 10_000 });
    // Let the EventSubscriptionManager connect + the initial fetches mirror the
    // runtime.state baseline before the realtime message arrives.
    await page.waitForTimeout(2_000);

    const marker = `E2E-RENDER-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;
    await cc.channel.sendChannelMessage({
      orgSlug: TEST_ORG_SLUG, channelId, source: marker,
    });

    // The message reaches the UI ONLY via realtime (posted from cc, not the
    // UI's own send path) — so this exercises the snapshot-push mirror end to end.
    await expect(
      page.getByTestId("channel-message-list").getByText(marker),
    ).toBeVisible({ timeout: 10_000 });

    if (createdId !== undefined) {
      await cc.channel.archiveChannel({ orgSlug: TEST_ORG_SLUG, id: createdId }).catch(() => undefined);
    }
  });
});
