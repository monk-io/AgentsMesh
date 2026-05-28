// Multi-tab UI propagation for channel:member_added / :member_removed.
//
// FIXME(follow-up): Requires the same prerequisite as the edit/delete
// multi-tab spec — stable channel selection across tabs (sidebar item
// testid or URL-based routing). See channel-edit-delete-multitab.spec.ts.
//
// Wire-level coverage already exists in tests/realtime/channel-events-wire.spec.ts
// (channel:member_added/removed proto frames). The UI assertion this would
// add is the member-count badge or member-list re-render in tab B after
// tab A's invite/remove RPC.
//
// Tracking: same follow-up issue as channel-edit-delete-multitab.
import { test } from "../../fixtures/index";

test.describe("Channel members · multi-tab UI propagation", () => {
  test.fixme("tab A invite/remove → tab B member count updates", async () => {
    // See file-level comment.
  });
});
