// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · uncategorized", () => {
  test("appAutopilotControllersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotControllersJson", returnType: "string" });
  });

  test("appAutopilotInsertController", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotInsertController", returnType: "void" }, []);
  });

  test("appAutopilotIterationsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotIterationsJson", returnType: "string" }, "");
  });

  test("appAutopilotReplaceCachedControllers", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotReplaceCachedControllers", returnType: "void" }, []);
  });

  test("appAutopilotReplaceCachedIterations", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotReplaceCachedIterations", returnType: "void" }, []);
  });

  test("appAutopilotThinkingHistoryJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotThinkingHistoryJson", returnType: "string" }, "");
  });

  test("appAutopilotThinkingJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAutopilotThinkingJson", returnType: "string" }, "");
  });

  test("appAvailableRunnersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appAvailableRunnersJson", returnType: "string" });
  });

  test("appChannelApplyMessageEdited", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelApplyMessageEdited", returnType: "void" }, []);
  });

  test("appChannelClearUnread", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelClearUnread", returnType: "void" }, 0);
  });

  test("appChannelInsertChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelInsertChannel", returnType: "void" }, []);
  });

  test("appChannelInsertMessage", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelInsertMessage", returnType: "void" }, []);
  });

  test("appChannelMentionCountsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelMentionCountsJson", returnType: "string" });
  });

  test("appChannelMessagesJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelMessagesJson", returnType: "string" }, 0);
  });

  test("appChannelPatchMemberCount", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelPatchMemberCount", returnType: "void" }, []);
  });

  test("appChannelPrependCachedMessages", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelPrependCachedMessages", returnType: "void" }, []);
  });

  test("appChannelRemoveMessage", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelRemoveMessage", returnType: "void" }, 0, 0);
  });

  test("appChannelReplaceCachedChannels", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelReplaceCachedChannels", returnType: "void" }, []);
  });

  test("appChannelReplaceCachedMessages", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelReplaceCachedMessages", returnType: "void" }, []);
  });

  test("appChannelReplaceUnreadCounts", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelReplaceUnreadCounts", returnType: "void" }, []);
  });

  test("appChannelsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelsJson", returnType: "string" });
  });

  test("appChannelUnreadCountsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appChannelUnreadCountsJson", returnType: "string" });
  });

  test("appCurrentRunnerJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appCurrentRunnerJson", returnType: "string" });
  });

  test("appGetPodJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appGetPodJson", returnType: "string" }, "");
  });

  test("appPodAppendCachedPods", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodAppendCachedPods", returnType: "void" }, []);
  });

  test("appPodInsertCreated", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodInsertCreated", returnType: "void" }, []);
  });

  test("appPodMarkTerminated", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodMarkTerminated", returnType: "void" }, []);
  });

  test("appPodPatchPerpetual", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodPatchPerpetual", returnType: "void" }, []);
  });

  test("appPodRemove", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodRemove", returnType: "void" }, "");
  });

  test("appPodReplaceCachedPods", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodReplaceCachedPods", returnType: "void" }, []);
  });

  test("appPodsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appPodsJson", returnType: "string" });
  });

  test("appRunnerPatch", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnerPatch", returnType: "void" }, []);
  });

  test("appRunnerRemove", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnerRemove", returnType: "void" }, []);
  });

  test("appRunnerReplaceAvailable", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnerReplaceAvailable", returnType: "void" }, []);
  });

  test("appRunnerReplaceCached", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnerReplaceCached", returnType: "void" }, []);
  });

  test("appRunnerSetCurrent", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnerSetCurrent", returnType: "void" }, []);
  });

  test("appRunnersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appRunnersJson", returnType: "string" });
  });

  test("appSelectChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appSelectChannel", returnType: "void" }, 0);
  });

  test("appSetCurrentChannel", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appSetCurrentChannel", returnType: "void" }, 0);
  });

  test("appSetCurrentUser", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "appSetCurrentUser", returnType: "void" }, 0);
  });

  test("relayDisconnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayDisconnect", returnType: "void" }, "");
  });

  test("relayDisconnectAll", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayDisconnectAll", returnType: "void" });
  });

  test("relayForceResize", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayForceResize", returnType: "void" }, "", 0, 0);
  });

  test("relayGetPodSize", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayGetPodSize", returnType: "Array<number>" }, "");
  });

  test("relayGetStatus", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayGetStatus", returnType: "string" }, "");
  });

  test("relayIsRunnerDisconnected", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayIsRunnerDisconnected", returnType: "boolean" }, "");
  });

  test("relaySend", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relaySend", returnType: "void" }, "", "");
  });

  test("relaySendAcpCommand", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relaySendAcpCommand", returnType: "void" }, "", "");
  });

  test("relaySendResize", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relaySendResize", returnType: "void" }, "", 0, 0);
  });

  test("relayUnsubscribe", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "relayUnsubscribe", returnType: "void" }, "", "");
  });
});
