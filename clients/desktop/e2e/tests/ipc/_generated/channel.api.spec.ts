// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · channel", () => {
  test("channelApplyChannelMessageEditedEvent", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelApplyChannelMessageEditedEvent", returnType: "any" });
  });

  test("channelApplyIncomingChannelMessage", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelApplyIncomingChannelMessage", returnType: "any" });
  });

  test("channelArchiveChannelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelArchiveChannelConnect", returnType: "any" });
  });

  test("channelChannelMembersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelChannelMembersJson", returnType: "string" }, 0);
  });

  test("channelChannelPodsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelChannelPodsJson", returnType: "string" }, 0);
  });

  test("channelChannelsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelChannelsJson", returnType: "string" });
  });

  test("channelClearChannelMentions", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelClearChannelMentions", returnType: "void" }, 0);
  });

  test("channelClearChannelUnread", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelClearChannelUnread", returnType: "void" }, 0);
  });

  test("channelCreateChannelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelCreateChannelConnect", returnType: "any" });
  });

  test("channelCurrentChannelJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelCurrentChannelJson", returnType: "string" });
  });

  test("channelDeleteChannelMessageConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelDeleteChannelMessageConnect", returnType: "any" });
  });

  test("channelEditChannelMessageConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelEditChannelMessageConnect", returnType: "any" });
  });

  test("channelFilterChannelsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelFilterChannelsJson", returnType: "string" }, "", false);
  });

  test("channelGetChannelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetChannelConnect", returnType: "any" });
  });

  test("channelGetChannelJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetChannelJson", returnType: "string" }, 0);
  });

  test("channelGetChannelUnreadCountsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetChannelUnreadCountsConnect", returnType: "any" });
  });

  test("channelGetLastMessageJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetLastMessageJson", returnType: "string" }, 0);
  });

  test("channelGetMentionCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetMentionCount", returnType: "number" }, 0);
  });

  test("channelGetMessagesJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetMessagesJson", returnType: "string" }, 0);
  });

  test("channelGetUnreadCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelGetUnreadCount", returnType: "number" }, 0);
  });

  test("channelIncrementMention", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelIncrementMention", returnType: "void" }, 0);
  });

  test("channelIncrementUnread", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelIncrementUnread", returnType: "void" }, 0);
  });

  test("channelInsertChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelInsertChannel", returnType: "void" }, []);
  });

  test("channelInsertChannelMessage", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelInsertChannelMessage", returnType: "any" });
  });

  test("channelListChannelMembersConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelListChannelMembersConnect", returnType: "any" });
  });

  test("channelListChannelMessagesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelListChannelMessagesConnect", returnType: "any" });
  });

  test("channelListChannelsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelListChannelsConnect", returnType: "any" });
  });

  test("channelMarkChannelReadConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelMarkChannelReadConnect", returnType: "any" });
  });

  test("channelMentionCountsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelMentionCountsJson", returnType: "string" });
  });

  test("channelPatchChannelMemberCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelPatchChannelMemberCount", returnType: "void" }, []);
  });

  test("channelPrependCachedChannelMessages", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelPrependCachedChannelMessages", returnType: "any" });
  });

  test("channelRemoveChannelMember", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelRemoveChannelMember", returnType: "void" }, 0, 0);
  });

  test("channelRemoveMessage", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelRemoveMessage", returnType: "any" });
  });

  test("channelReplaceCachedChannelMessages", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelReplaceCachedChannelMessages", returnType: "any" });
  });

  test("channelReplaceCachedChannels", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelReplaceCachedChannels", returnType: "void" }, []);
  });

  test("channelReplaceChannelMembers", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelReplaceChannelMembers", returnType: "void" }, []);
  });

  test("channelReplaceChannelPods", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelReplaceChannelPods", returnType: "void" }, []);
  });

  test("channelReplaceChannelUnreadCounts", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelReplaceChannelUnreadCounts", returnType: "any" });
  });

  test("channelSelectChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelSelectChannel", returnType: "string" }, 0);
  });

  test("channelSendChannelMessageConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelSendChannelMessageConnect", returnType: "any" });
  });

  test("channelSetCurrentChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelSetCurrentChannel", returnType: "void" }, 0);
  });

  test("channelSetCurrentUserId", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelSetCurrentUserId", returnType: "void" }, 0);
  });

  test("channelSortedChannelIdsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelSortedChannelIdsJson", returnType: "string" }, "", false);
  });

  test("channelTotalMentionCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelTotalMentionCount", returnType: "number" });
  });

  test("channelTotalUnreadCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelTotalUnreadCount", returnType: "number" });
  });

  test("channelUnarchiveChannelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelUnarchiveChannelConnect", returnType: "any" });
  });

  test("channelUnreadCountsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelUnreadCountsJson", returnType: "string" });
  });

  test("channelUpdateChannelConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "channelUpdateChannelConnect", returnType: "any" });
  });
});
