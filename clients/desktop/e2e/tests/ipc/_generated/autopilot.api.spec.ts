// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · autopilot", () => {
  test("autopilotAppendIteration", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotAppendIteration", returnType: "void" }, []);
  });

  test("autopilotControllersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotControllersJson", returnType: "string" });
  });

  test("autopilotCurrentControllerJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotCurrentControllerJson", returnType: "string" });
  });

  test("autopilotGetControllerByPodKeyJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotGetControllerByPodKeyJson", returnType: "string" }, "");
  });

  test("autopilotGetIterationsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotGetIterationsJson", returnType: "string" }, "");
  });

  test("autopilotGetThinkingHistoryJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotGetThinkingHistoryJson", returnType: "string" }, "");
  });

  test("autopilotGetThinkingJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotGetThinkingJson", returnType: "string" }, "");
  });

  test("autopilotInsertController", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotInsertController", returnType: "void" }, []);
  });

  test("autopilotPatchController", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotPatchController", returnType: "void" }, []);
  });

  test("autopilotRemoveControllerProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotRemoveControllerProto", returnType: "void" }, []);
  });

  test("autopilotReplaceCachedControllers", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotReplaceCachedControllers", returnType: "void" }, []);
  });

  test("autopilotReplaceCachedIterations", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotReplaceCachedIterations", returnType: "void" }, []);
  });

  test("autopilotSetCurrentControllerProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotSetCurrentControllerProto", returnType: "void" }, []);
  });

  test("autopilotUpdateThinkingProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "autopilotUpdateThinkingProto", returnType: "void" }, []);
  });
});
