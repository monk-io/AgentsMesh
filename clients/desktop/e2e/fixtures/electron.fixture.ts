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

export interface ElectronFixtures {
  electronApp: ElectronApplication;
  /** The Electron main window as a Playwright Page. */
  page: Page;
  /** Path of the saved auth storage (may not exist in setup project). */
  authFile: string;
  /** Skip applying saved localStorage (use for fresh-login tests). */
  skipAuthRestore: boolean;
  /**
   * Per-test isolated Electron userData directory.
   *
   * **Why per-test**: Chromium inside Electron holds an exclusive
   * `SingletonLock` + LevelDB lock on `<userData>/Local State`. Two
   * concurrent Electron instances pointing at the same dir deadlock
   * within ~5s and the second's `firstWindow()` times out. Per-test
   * isolation is the only way to safely run `--workers > 1`.
   *
   * **Why clone from setup dir**: global.setup.ts logs in and persists
   * Rust auth state to `getUserDataDir()/agentsmesh-files/auth_session.json`
   * (via the FileStorage backend in node-bridge). The renderer's
   * localStorage snapshot lives in `getAuthFile()` and is restored via
   * `restoreStorage()`, but the Rust side is on-disk. If we hand the
   * fixture a fresh empty tmpdir, `authBootstrap` finds no session →
   * router guards bounce to `/login`. Cloning the setup-completed dir
   * preserves the Rust auth state without re-logging-in per test.
   *
   * Industry references:
   * - VS Code smoke test runner clones a pre-authenticated user-data-dir
   *   per test (`test/smoke/src/areas/launcher.ts`).
   * - Microsoft Playwright + Electron docs: "if your app persists state
   *   on disk, copy the master profile to a tmpdir per test".
   */
  userDataDir: string;
}

/**
 * Launch Electron with AGENTSMESH_API_URL + NODE_ENV=test + isolated userData.
 * If a saved storage snapshot exists (from global.setup.ts), inject it into localStorage.
 */
export const test = base.extend<ElectronFixtures>({
  authFile: async ({}, use) => {
    await use(getAuthFile());
  },

  userDataDir: async ({}, use, testInfo) => {
    const dir = await mkdtemp(
      join(tmpdir(), `agentsmesh-e2e-w${testInfo.workerIndex}-`),
    );
    // Clone the setup-completed userData (Rust auth state + FileStorage
    // sessions) into this test's fresh tmpdir. `fs.cp` with recursive +
    // preserveTimestamps gives us an exact copy without subprocess shell
    // out. The setup dir might not exist if a non-auth-dependent spec
    // runs before setup — fall through silently and let `authBootstrap`
    // discover an empty state.
    const setupDir = getUserDataDir();
    if (existsSync(setupDir)) {
      try {
        await cp(setupDir, dir, { recursive: true, preserveTimestamps: true });
      } catch {
        // EBUSY / partial-clone is acceptable — fall through and let the
        // fixture's `restoreStorage + authBootstrap` rehydrate from the
        // file-level auth snapshot.
      }
    }
    await use(dir);
    await rm(dir, { recursive: true, force: true }).catch(() => undefined);
  },

  skipAuthRestore: [false, { option: true }],

  electronApp: async ({ userDataDir }, use) => {
    const ciArgs = isCi() && process.platform === "linux"
      ? ["--no-sandbox", "--disable-dev-shm-usage"]
      : [];
    const app = await electron.launch({
      args: [getElectronMainPath(), `--user-data-dir=${userDataDir}`, ...ciArgs],
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

  page: async ({ electronApp, authFile, skipAuthRestore }, use) => {
    const page = await electronApp.firstWindow({ timeout: isCi() ? 90_000 : 30_000 });
    await page.waitForLoadState("domcontentloaded");

    const snap = skipAuthRestore ? null : loadStorageFile(authFile);
    if (snap) await restoreStorage(page, snap);

    await page.evaluate(() => localStorage.setItem("app_locale", "en"));

    await page.reload().catch(() => undefined);
    await page.waitForLoadState("domcontentloaded");

    if (snap) {
      await invokeIpc(page, "authBootstrap");
    }

    await use(page);
  },
});

export { expect } from "@playwright/test";
