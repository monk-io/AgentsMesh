// AUTO-GENERATED — do not edit by hand. Regenerate: pnpm --filter desktop e2e:gen
import { test } from "../../../fixtures/electron-shared.fixture";
import { invokeIpcContract } from "../../../helpers/ipc-contract";

test.describe.configure({ mode: "serial" });

test.describe("IPC · extension", () => {
  test("extensionCreateSkillRegistryConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionCreateSkillRegistryConnect", returnType: "Array<number>" }, []);
  });

  test("extensionDeleteSkillRegistryConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionDeleteSkillRegistryConnect", returnType: "Array<number>" }, []);
  });

  test("extensionInstallCustomMcpServerConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionInstallCustomMcpServerConnect", returnType: "Array<number>" }, []);
  });

  test("extensionInstallMcpFromMarketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionInstallMcpFromMarketConnect", returnType: "Array<number>" }, []);
  });

  test("extensionInstallSkillFromGithubConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionInstallSkillFromGithubConnect", returnType: "Array<number>" }, []);
  });

  test("extensionInstallSkillFromMarketConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionInstallSkillFromMarketConnect", returnType: "Array<number>" }, []);
  });

  test("extensionInstallSkillFromUploadedFileConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionInstallSkillFromUploadedFileConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListMarketMcpServersConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListMarketMcpServersConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListMarketSkillsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListMarketSkillsConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListRepoMcpServersConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListRepoMcpServersConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListRepoSkillsConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListRepoSkillsConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListSkillRegistriesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListSkillRegistriesConnect", returnType: "Array<number>" }, []);
  });

  test("extensionListSkillRegistryOverridesConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionListSkillRegistryOverridesConnect", returnType: "Array<number>" }, []);
  });

  test("extensionPresignSkillUploadConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionPresignSkillUploadConnect", returnType: "Array<number>" }, []);
  });

  test("extensionSyncSkillRegistryConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionSyncSkillRegistryConnect", returnType: "Array<number>" }, []);
  });

  test("extensionTogglePlatformRegistryConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionTogglePlatformRegistryConnect", returnType: "Array<number>" }, []);
  });

  test("extensionUninstallMcpServerConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionUninstallMcpServerConnect", returnType: "Array<number>" }, []);
  });

  test("extensionUninstallSkillConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionUninstallSkillConnect", returnType: "Array<number>" }, []);
  });

  test("extensionUpdateMcpServerConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionUpdateMcpServerConnect", returnType: "Array<number>" }, []);
  });

  test("extensionUpdateSkillConnect", async ({ sharedPage }) => {
    await invokeIpcContract(sharedPage, { method: "extensionUpdateSkillConnect", returnType: "Array<number>" }, []);
  });
});
