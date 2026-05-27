// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · binding", () => {
  test("bindingAcceptBindingConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingAcceptBindingConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingApproveScopesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingApproveScopesConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingCheckBindingConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingCheckBindingConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingGetBoundPodsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingGetBoundPodsConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingGetPendingBindingsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingGetPendingBindingsConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingListBindingsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingListBindingsConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingRejectBindingConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingRejectBindingConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingRequestBindingConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingRequestBindingConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingRequestScopesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingRequestScopesConnect", returnType: "Uint8Array" }, []);
  });

  test("bindingUnbindConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "bindingUnbindConnect", returnType: "Uint8Array" }, []);
  });
});
