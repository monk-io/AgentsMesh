// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · env_bundle", () => {
  test("envBundleCreateEnvBundleConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleCreateEnvBundleConnect", returnType: "Array<number>" }, []);
  });

  test("envBundleDeleteEnvBundleConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleDeleteEnvBundleConnect", returnType: "Array<number>" }, []);
  });

  test("envBundleGetEnvBundleConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleGetEnvBundleConnect", returnType: "Array<number>" }, []);
  });

  test("envBundleListEnvBundlesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleListEnvBundlesConnect", returnType: "Array<number>" }, []);
  });

  test("envBundleSetPrimaryEnvBundleConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleSetPrimaryEnvBundleConnect", returnType: "Array<number>" }, []);
  });

  test("envBundleUpdateEnvBundleConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "envBundleUpdateEnvBundleConnect", returnType: "Array<number>" }, []);
  });
});
