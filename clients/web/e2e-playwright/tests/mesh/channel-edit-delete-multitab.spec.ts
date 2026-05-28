// Multi-tab UI propagation for channel:message_edited / :message_deleted.
//
// FIXME(follow-up): Channel selection is store-based (selectedChannelId via
// useChannelStore), not URL-based. Driving two tabs into the same channel
// requires either:
//   (a) a stable testid on the sidebar ChannelListItem so we can click +
//       wait for state hydration deterministically; or
//   (b) a URL-routing refactor in clients/web/src/app/(dashboard)/[org]/
//       channels/ to make channel id a query param.
//
// Wire-level coverage already exists in tests/realtime/channel-events-wire.spec.ts
// (asserts the proto-typed wire frames arrive). What this spec would add is
// the renderer-side propagation: handlers → useChannelMessageStore → React
// re-render of the message bubble after edit/delete.
//
// Tracking: follow-up issue "test(e2e-web): channel message edit/delete
// multi-tab UI propagation" — depends on (a) above.
import { test } from "../../fixtures/index";

test.describe("Channel message edit/delete · multi-tab UI propagation", () => {
  test.fixme("tab A edit/delete → tab B message list updates", async () => {
    // See file-level comment.
  });
});
