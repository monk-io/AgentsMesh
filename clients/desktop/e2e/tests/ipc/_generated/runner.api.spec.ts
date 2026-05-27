// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · runner", () => {
  test("runnerAuthorizeRunner", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerAuthorizeRunner", returnType: "Uint8Array" }, []);
  });

  test("runnerAvailableRunnersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerAvailableRunnersJson", returnType: "string" });
  });

  test("runnerCurrentRunnerJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerCurrentRunnerJson", returnType: "string" });
  });

  test("runnerGetAuthStatus", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerGetAuthStatus", returnType: "Uint8Array" }, []);
  });

  test("runnerGetRunnerJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerGetRunnerJson", returnType: "string" }, 0);
  });

  test("runnerPatchCachedRunner", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerPatchCachedRunner", returnType: "void" }, []);
  });

  test("runnerRemoveCachedRunner", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerRemoveCachedRunner", returnType: "void" }, []);
  });

  test("runnerReplaceAvailableRunners", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerReplaceAvailableRunners", returnType: "void" }, []);
  });

  test("runnerReplaceCachedRunners", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerReplaceCachedRunners", returnType: "void" }, []);
  });

  test("runnerRunnersJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerRunnersJson", returnType: "string" });
  });

  test("runnerSetCurrentRunnerProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerSetCurrentRunnerProto", returnType: "void" }, []);
  });

  test("runnerUpdateRunnerStatus", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerUpdateRunnerStatus", returnType: "void" }, 0, "");
  });
});
