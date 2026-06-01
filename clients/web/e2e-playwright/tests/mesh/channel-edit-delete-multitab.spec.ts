// Multi-tab UI propagation for channel:message_edited + :message_deleted.
//
// Both tabs open the same channel via sidebar click; tab A edits/deletes
// a message; tab B's message list updates without manual refresh.
//
// Wire-level coverage already in tests/realtime/channel-events-wire.spec.ts;
// this spec proves the renderer-side propagation chain works (handler →
// useChannelMessageStore → React).
import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";
import { TEST_ORG_SLUG } from "../../helpers/env";

test.describe("Channel message edit/delete · multi-tab UI propagation", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("tab A edit + delete → tab B message list reflects both changes", async ({ context, api }) => {
    const cc = await api.connect();

    const stamp = Date.now().toString(36);
    const ch = (await cc.channel.createChannel({
      orgSlug: TEST_ORG_SLUG,
      name: `e2e-edit-${stamp}`,
    })) as { id: bigint | number };
    const channelId = ch.id;
    const channelIdStr = String(channelId);

    const seedBody = `seed-${stamp}`;
    const seedMsg = (await cc.channel.sendChannelMessage({
      orgSlug: TEST_ORG_SLUG, channelId, source: seedBody,
    })) as { id: bigint | number };

    const tabA = await context.newPage();
    const tabB = await context.newPage();
    await Promise.all([
      tabA.goto(`/${TEST_ORG_SLUG}/channels`),
      tabB.goto(`/${TEST_ORG_SLUG}/channels`),
    ]);

    // Sidebar mounts client-side after wasm bootstrap. Use toHaveCount
    // (not toBeVisible) because the channel may be outside the viewport
    // in a long list. realtime channel:member_added handler triggers
    // fetchChannels({includeArchived:true}) so the just-created channel
    // arrives in the sidebar without manual refresh.
    const channelSelector = `[data-testid="channel-list-item"][data-channel-id="${channelIdStr}"]`;
    await Promise.all([
      expect(tabA.locator(channelSelector)).toHaveCount(1, { timeout: 30_000 }),
      expect(tabB.locator(channelSelector)).toHaveCount(1, { timeout: 30_000 }),
    ]);

    await tabA.locator(channelSelector).scrollIntoViewIfNeeded();
    await tabB.locator(channelSelector).scrollIntoViewIfNeeded();
    await Promise.all([
      tabA.locator(channelSelector).click(),
      tabB.locator(channelSelector).click(),
    ]);

    const seedMessageSelector = `[data-message-id="${String(seedMsg.id)}"]`;
    await Promise.all([
      expect(tabA.locator(seedMessageSelector)).toContainText(seedBody, { timeout: 15_000 }),
      expect(tabB.locator(seedMessageSelector)).toContainText(seedBody, { timeout: 15_000 }),
    ]);

    // EventSubscriptionManager.connect() runs async after wasm bootstrap;
    // 1500ms settle window so both tabs are registered with the backend
    // hub before publish. Pattern from tests/blockstore/multi-tab-sync.spec.ts.
    await tabA.waitForTimeout(1500);

    const editedBody = `edited-${stamp}`;
    await cc.channel.editChannelMessage({
      orgSlug: TEST_ORG_SLUG, channelId, messageId: seedMsg.id, source: editedBody,
    });

    await Promise.all([
      expect(tabA.locator(seedMessageSelector)).toContainText(editedBody, { timeout: 10_000 }),
      expect(tabB.locator(seedMessageSelector)).toContainText(editedBody, { timeout: 10_000 }),
    ]);

    await cc.channel.deleteChannelMessage({
      orgSlug: TEST_ORG_SLUG, channelId, messageId: seedMsg.id,
    });

    await Promise.all([
      expect(tabA.locator(seedMessageSelector)).toHaveCount(0, { timeout: 10_000 }),
      expect(tabB.locator(seedMessageSelector)).toHaveCount(0, { timeout: 10_000 }),
    ]);

    await tabA.close();
    await tabB.close();
    await cc.channel.archiveChannel({ orgSlug: TEST_ORG_SLUG, id: channelId }).catch(() => undefined);
  });
});
