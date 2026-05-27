import { test as base, _electron as electron, type ElectronApplication, type Page } from "@playwright/test";
import { mkdtemp, rm, cp } from "node:fs/promises";
import { existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  getApiBaseUrl,
  getElectronMainPath,
  getAuthFile,
  getUserDataDir,
  isCi,
} from "../helpers/env";
import { invokeIpc } from "../helpers/ipc";
import { loadStorageFile, restoreStorage } from "../helpers/storage-state";

export interface SharedElectronFixtures {
  sharedElectronApp: ElectronApplication;
  sharedPage: Page;
  sharedUserDataDir: string;
}

export const test = base.extend<Record<string, never>, SharedElectronFixtures>({
  sharedUserDataDir: [
    async ({}, use, workerInfo) => {
      const dir = await mkdtemp(
        join(tmpdir(), `agentsmesh-e2e-shared-w${workerInfo.workerIndex}-`),
      );
      const setupDir = getUserDataDir();
      if (existsSync(setupDir)) {
        try {
          await cp(setupDir, dir, { recursive: true, preserveTimestamps: true });
        } catch {
          // Setup dir clone is a best-effort optimization; specs that
          // depend on auth state will still get it via restoreStorage +
          // authBootstrap in sharedPage below.
        }
      }
      await use(dir);
      await rm(dir, { recursive: true, force: true }).catch(() => undefined);
    },
    { scope: "worker" },
  ],

  sharedElectronApp: [
    async ({ sharedUserDataDir }, use) => {
      const ciArgs = isCi() && process.platform === "linux"
        ? ["--no-sandbox", "--disable-dev-shm-usage"]
        : [];
      const app = await electron.launch({
        args: [getElectronMainPath(), `--user-data-dir=${sharedUserDataDir}`, ...ciArgs],
        env: {
          ...process.env,
          AGENTSMESH_API_URL: getApiBaseUrl(),
          NODE_ENV: "test",
          ELECTRON_DISABLE_SECURITY_WARNINGS: "true",
        },
        timeout: isCi() ? 120_000 : 30_000,
      });
      await use(app);
      await app.close().catch(() => undefined);
    },
    { scope: "worker" },
  ],

  sharedPage: [
    async ({ sharedElectronApp }, use) => {
      const page = await sharedElectronApp.firstWindow({ timeout: isCi() ? 90_000 : 30_000 });
      await page.waitForLoadState("domcontentloaded");

      const snap = loadStorageFile(getAuthFile());
      if (snap) {
        await restoreStorage(page, snap);
        await page.reload().catch(() => undefined);
        await page.waitForLoadState("domcontentloaded");
        await invokeIpc(page, "authBootstrap");
      }
      await use(page);
    },
    { scope: "worker" },
  ],
});

export { expect } from "@playwright/test";
