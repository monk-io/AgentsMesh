// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · runner", () => {
  test("runnerAuthorizeRunner", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerAuthorizeRunner", returnType: "Array<number>" }, []);
  });

  test("runnerGetAuthStatus", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "runnerGetAuthStatus", returnType: "Array<number>" }, []);
  });
});
