// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · pod", () => {
  test("podCurrentPodJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "podCurrentPodJson", returnType: "string" });
  });

  test("podGetPodJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "podGetPodJson", returnType: "string" }, "");
  });

  test("podPodsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "podPodsJson", returnType: "string" });
  });
});
