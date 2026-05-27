// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · blockstore", () => {
  test("blockstoreApplyRemoteOp", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreApplyRemoteOp", returnType: "void" }, []);
  });

  test("blockstoreBacklinksJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreBacklinksJson", returnType: "string" });
  });

  test("blockstoreBlocksJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreBlocksJson", returnType: "string" });
  });

  test("blockstoreCatchup", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreCatchup", returnType: "void" }, "");
  });

  test("blockstoreEnsureDefaultWorkspace", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreEnsureDefaultWorkspace", returnType: "string" });
  });

  test("blockstoreGetBlockJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreGetBlockJson", returnType: "string | null" }, "");
  });

  test("blockstoreLastOpId", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreLastOpId", returnType: "number" }, "");
  });

  test("blockstoreLastOpIdsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreLastOpIdsJson", returnType: "string" });
  });

  test("blockstoreListBacklinksJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreListBacklinksJson", returnType: "string" }, "");
  });

  test("blockstoreListChildrenJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreListChildrenJson", returnType: "string" }, "");
  });

  test("blockstoreListWorkspaces", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreListWorkspaces", returnType: "string" });
  });

  test("blockstoreLoadSubtree", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreLoadSubtree", returnType: "void" }, "", "");
  });

  test("blockstoreLoadTypeDefs", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreLoadTypeDefs", returnType: "void" }, "");
  });

  test("blockstoreNestChildrenJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreNestChildrenJson", returnType: "string" });
  });

  test("blockstoreProjectLocalOps", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreProjectLocalOps", returnType: "void" }, []);
  });

  test("blockstoreRefsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreRefsJson", returnType: "string" });
  });

  test("blockstoreReplaceWorkspaces", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreReplaceWorkspaces", returnType: "void" }, []);
  });

  test("blockstoreSetLastOpId", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreSetLastOpId", returnType: "void" }, "", 0);
  });

  test("blockstoreTypeDefsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreTypeDefsJson", returnType: "string" }, "");
  });

  test("blockstoreUpsertBlocks", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreUpsertBlocks", returnType: "void" }, []);
  });

  test("blockstoreUpsertRefs", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreUpsertRefs", returnType: "void" }, []);
  });

  test("blockstoreUpsertWorkspace", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreUpsertWorkspace", returnType: "void" }, []);
  });

  test("blockstoreWorkspacesJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "blockstoreWorkspacesJson", returnType: "string" });
  });
});
