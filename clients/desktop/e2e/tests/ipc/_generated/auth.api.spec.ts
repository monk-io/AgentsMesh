// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · auth", () => {
  test("authApplySessionProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authApplySessionProto", returnType: "void" }, []);
  });

  test("authBootstrap", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authBootstrap", returnType: "string" });
  });

  test("authBootstrapProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authBootstrapProto", returnType: "Array<number>" });
  });

  test("authClearSession", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authClearSession", returnType: "void" });
  });

  test("authFetchOrganizations", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authFetchOrganizations", returnType: "string" });
  });

  test("authFetchOrganizationsProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authFetchOrganizationsProto", returnType: "Array<number>" });
  });

  test("authGetCurrentOrgJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetCurrentOrgJson", returnType: "string | undefined | null" });
  });

  test("authGetCurrentUserJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetCurrentUserJson", returnType: "string | undefined | null" });
  });

  test("authGetCurrentUserProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetCurrentUserProto", returnType: "Array<number> | undefined | null" });
  });

  test("authGetExpiresAt", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetExpiresAt", returnType: "number | undefined | null" });
  });

  test("authGetOrganizationsJson", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetOrganizationsJson", returnType: "string" });
  });

  test("authGetToken", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authGetToken", returnType: "string | undefined | null" });
  });

  test("authIsAuthenticated", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authIsAuthenticated", returnType: "boolean" });
  });

  test("authLogin", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authLogin", returnType: "string" }, "", "");
  });

  test("authLoginProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authLoginProto", returnType: "Array<number>" }, "", "");
  });

  test("authLogout", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authLogout", returnType: "void" });
  });

  test("authRefreshToken", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authRefreshToken", returnType: "string" });
  });

  test("authRefreshTokenProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authRefreshTokenProto", returnType: "Array<number>" });
  });

  test("authSetCurrentOrgProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authSetCurrentOrgProto", returnType: "void" }, []);
  });

  test("authSetOrganizationsProto", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authSetOrganizationsProto", returnType: "void" }, []);
  });

  test("authSwitchOrg", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "authSwitchOrg", returnType: "void" }, "");
  });
});
