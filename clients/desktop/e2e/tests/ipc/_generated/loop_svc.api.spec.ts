// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · loop_svc", () => {
  test("loopSvcAppendCachedRuns", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcAppendCachedRuns", returnType: "void" }, []);
  });

  test("loopSvcClearCurrentLoop", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcClearCurrentLoop", returnType: "void" }, []);
  });

  test("loopSvcClearLoopRuns", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcClearLoopRuns", returnType: "void" }, []);
  });

  test("loopSvcCurrentLoopJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcCurrentLoopJson", returnType: "string" });
  });

  test("loopSvcGetLoopBySlugJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcGetLoopBySlugJson", returnType: "string" }, "");
  });

  test("loopSvcInsertLoopRun", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcInsertLoopRun", returnType: "void" }, []);
  });

  test("loopSvcLoopsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcLoopsJson", returnType: "string" });
  });

  test("loopSvcPatchLoopFromAction", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcPatchLoopFromAction", returnType: "void" }, []);
  });

  test("loopSvcPatchLoopRunStatus", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcPatchLoopRunStatus", returnType: "void" }, []);
  });

  test("loopSvcReplaceCachedLoops", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcReplaceCachedLoops", returnType: "void" }, []);
  });

  test("loopSvcReplaceCachedRuns", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcReplaceCachedRuns", returnType: "void" }, []);
  });

  test("loopSvcRunsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcRunsJson", returnType: "string" });
  });

  test("loopSvcSetCurrentLoop", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "loopSvcSetCurrentLoop", returnType: "void" }, []);
  });
});
