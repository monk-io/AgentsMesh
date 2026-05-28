// Wire-level realtime EventBus verification for channel:* events.
//
// Asserts SendChannelMessage / EditChannelMessage / DeleteChannelMessage /
// InviteChannelMembers / RemoveChannelMember publish the typed
// proto.events.v1.ChannelMessage{Event,Edited,Deleted}EventData /
// ChannelMemberChangedEventData on the wire.
//
// channel:message wire delivery is also covered by tests/mesh/channel-realtime.spec.ts
// (predates this file); we keep that as the canonical pattern and add the
// edit/delete/member coverage that was previously NONE per SSOT audit.
import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";
import { withEventSubscription } from "../../helpers/eventbus-stream";

async function ensureChannel(api: import("../../fixtures/api.fixture").ApiFixture) {
  const cc = await api.connect();
  const listed = (await cc.channel.listChannels({ orgSlug: TEST_ORG_SLUG })) as {
    items: Array<{ id: bigint | number }>;
  };
  if (listed.items.length) return { channelId: listed.items[0].id, created: false };
  const ch = (await cc.channel.createChannel({
    orgSlug: TEST_ORG_SLUG,
    name: `e2e-realtime-${Date.now().toString(36)}`,
  })) as { id: bigint | number };
  return { channelId: ch.id, created: true };
}

async function sendMarkerMessage(
  api: import("../../fixtures/api.fixture").ApiFixture,
  channelId: bigint | number,
  marker: string,
): Promise<bigint | number> {
  const cc = await api.connect();
  const msg = (await cc.channel.sendChannelMessage({
    orgSlug: TEST_ORG_SLUG, channelId, source: marker,
  })) as { id: bigint | number };
  return msg.id;
}

test.describe("Realtime · channel events (wire)", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("channel:message arrives with body + channel_id", async ({ api }) => {
    const token = api.getToken() ?? ((await api.connect()), api.getToken());
    if (!token) throw new Error("api fixture missing token");
    const { channelId } = await ensureChannel(api);
    const marker = `m-${Date.now().toString(36)}`;

    const { event } = await withEventSubscription<unknown, { body?: string; channel_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "channel:message" &&
          typeof data.body === "string" && data.body.includes(marker),
      },
      async () => { await sendMarkerMessage(api, channelId, marker); },
    );

    expect(event.data.body).toContain(marker);
    expect(Number(event.data.channel_id)).toBe(Number(channelId));
  });

  test("channel:message_edited arrives with updated body", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const { channelId } = await ensureChannel(api);
    const marker = `e-${Date.now().toString(36)}`;
    const messageId = await sendMarkerMessage(api, channelId, `before-${marker}`);
    const editedBody = `after-${marker}`;

    const { event } = await withEventSubscription<unknown, { body?: string; id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "channel:message_edited" &&
          typeof data.body === "string" && data.body.includes(editedBody),
      },
      async () => {
        await cc.channel.editChannelMessage({
          orgSlug: TEST_ORG_SLUG, channelId, messageId, source: editedBody,
        });
      },
    );

    expect(event.data.body).toContain(editedBody);
    expect(Number(event.data.id)).toBe(Number(messageId));
  });

  test("channel:message_deleted arrives with message id", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");
    const { channelId } = await ensureChannel(api);
    const marker = `d-${Date.now().toString(36)}`;
    const messageId = await sendMarkerMessage(api, channelId, marker);

    const { event } = await withEventSubscription<unknown, { id?: number | string; channel_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "channel:message_deleted" && Number(data.id) === Number(messageId),
      },
      async () => {
        await cc.channel.deleteChannelMessage({
          orgSlug: TEST_ORG_SLUG, channelId, messageId,
        });
      },
    );

    expect(Number(event.data.id)).toBe(Number(messageId));
    expect(Number(event.data.channel_id)).toBe(Number(channelId));
  });

  test("channel:member_added arrives when inviting", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");

    // Two users + a channel: invite user A (test user is already a member
    // of the seeded org's default channels, so we create a fresh channel
    // and invite the secondary dev user).
    const ch = (await cc.channel.createChannel({
      orgSlug: TEST_ORG_SLUG,
      name: `e2e-members-${Date.now().toString(36)}`,
    })) as { id: bigint | number };
    const channelId = ch.id;

    // The dev seed always has a user with id=2 as a secondary user.
    const inviteeId = 2n;

    const { event } = await withEventSubscription<unknown, { channel_id?: number | string; user_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "channel:member_added" &&
          Number(data.channel_id) === Number(channelId),
      },
      async () => {
        await cc.channel.inviteChannelMembers({
          orgSlug: TEST_ORG_SLUG, id: channelId, userIds: [inviteeId],
        });
      },
    );

    expect(Number(event.data.channel_id)).toBe(Number(channelId));
    expect(Number(event.data.user_id)).toBe(Number(inviteeId));

    await cc.channel.archiveChannel({ orgSlug: TEST_ORG_SLUG, id: channelId }).catch(() => undefined);
  });

  test("channel:member_removed arrives when removing", async ({ api }) => {
    const cc = await api.connect();
    const token = api.getToken();
    if (!token) throw new Error("api fixture missing token");

    const ch = (await cc.channel.createChannel({
      orgSlug: TEST_ORG_SLUG,
      name: `e2e-members-${Date.now().toString(36)}`,
    })) as { id: bigint | number };
    const channelId = ch.id;
    const inviteeId = 2n;

    await cc.channel.inviteChannelMembers({
      orgSlug: TEST_ORG_SLUG, id: channelId, userIds: [inviteeId],
    });

    const { event } = await withEventSubscription<unknown, { channel_id?: number | string; user_id?: number | string }>(
      {
        token, orgSlug: TEST_ORG_SLUG,
        predicate: (type, data) =>
          type === "channel:member_removed" &&
          Number(data.channel_id) === Number(channelId),
      },
      async () => {
        await cc.channel.removeChannelMember({
          orgSlug: TEST_ORG_SLUG, id: channelId, userId: inviteeId,
        });
      },
    );

    expect(Number(event.data.channel_id)).toBe(Number(channelId));
    expect(Number(event.data.user_id)).toBe(Number(inviteeId));

    await cc.channel.archiveChannel({ orgSlug: TEST_ORG_SLUG, id: channelId }).catch(() => undefined);
  });
});
