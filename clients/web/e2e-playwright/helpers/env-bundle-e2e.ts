import type { Page } from "@playwright/test";
import { execSync } from "node:child_process";
import { TEST_ORG_SLUG, getComposeProject } from "./env";
import { pollUntil } from "./retry";
import { CreatePodModal } from "../pages/modals/create-pod.modal";
import type { ApiFixture } from "../fixtures/api.fixture";

const RUNNER_CONTAINER = `${getComposeProject()}-runner-1`;

/**
 * Drive the Pod create dialog end-to-end and return the new pod's key.
 *
 * Owns the boilerplate that's identical across EnvBundle e2e specs:
 *   - navigate to /workspace (so the New-Pod button is mountable)
 *   - intercept POST /pods (via waitForResponse, no listener accumulation)
 *   - open dialog, select agent, expand advanced, apply bundle selection,
 *     submit, wait for close
 *   - poll backend until pod status reaches "running"
 *
 * Throws if the POST returns non-2xx so the caller doesn't have to check
 * an error-state flag.
 */
export async function createPodAndWaitRunning(args: {
  page: Page;
  api: ApiFixture;
  agentSlug: string;
  /** Empty string = "Use Agent default auth"; undefined = leave picker alone. */
  selectCredentialName?: string;
  selectRuntimeBundleNames?: string[];
  /**
   * Optional escape hatch: run extra interactions against the modal after
   * the default credential/runtime picks but before submit. Useful for
   * exercising switch-paths (e.g. picking credential A then credential B).
   */
  customizeModal?: (modal: CreatePodModal) => Promise<void>;
  /** Status-poll timeout (default 30s). */
  statusTimeoutMs?: number;
}): Promise<string> {
  const {
    page,
    api,
    agentSlug,
    selectCredentialName,
    selectRuntimeBundleNames,
    customizeModal,
    statusTimeoutMs = 30_000,
  } = args;

  await page.goto(`/${TEST_ORG_SLUG}/workspace`);
  await page.waitForLoadState("domcontentloaded");

  // waitForResponse filters by URL+method, so unrelated responses (favicon,
  // CSS, etc.) don't consume the listener — addresses a `page.once`
  // foot-gun where the FIRST response (any URL) would burn the handle.
  const podCreatePromise = page.waitForResponse(
    (r) =>
      r.request().method() === "POST" &&
      r.url().endsWith(`/api/v1/orgs/${TEST_ORG_SLUG}/pods`),
    { timeout: 20_000 },
  );

  const newPodBtn = page
    .getByRole("button", { name: /new pod|create new pod|新建 pod/i })
    .first();
  await newPodBtn.click();

  const modal = new CreatePodModal(page);
  await modal.waitForOpen();
  await modal.selectAgent(agentSlug);
  await modal.expandAdvancedOptions();
  if (selectCredentialName !== undefined) {
    await modal.selectCredential(selectCredentialName);
  }
  if (selectRuntimeBundleNames !== undefined) {
    await modal.selectRuntimeBundles(selectRuntimeBundleNames);
  }
  if (customizeModal) {
    await customizeModal(modal);
  }
  await modal.submit();

  const podRes = await podCreatePromise;
  const body = await podRes.json().catch(() => ({}));
  if (!podRes.ok()) {
    throw new Error(`Pod create failed: HTTP ${podRes.status()}: ${JSON.stringify(body)}`);
  }
  const createdPodKey = body?.pod?.pod_key;
  if (typeof createdPodKey !== "string" || !createdPodKey) {
    throw new Error(`Pod create response missing pod_key: ${JSON.stringify(body)}`);
  }

  await modal.waitForClosed(15_000);

  // Status enum is the backend `agentpod.Status*` set ({initializing,
  // running, paused, disconnected}). "running" is the only value where
  // the child process has been spawned and env is set.
  await pollUntil(
    async () => {
      const r = await api.get(`/api/v1/orgs/${TEST_ORG_SLUG}/pods/${createdPodKey}`);
      if (!r.ok) return false;
      const data = await r.json();
      return data?.pod?.status === "running";
    },
    {
      maxAttempts: Math.ceil(statusTimeoutMs / 1000),
      intervalMs: 1000,
      label: `pod ${createdPodKey} → running`,
    },
  );

  return createdPodKey;
}

/**
 * Read the env dump file that the e2e-echo agent writes on startup
 * (`/tmp/e2e-echo-env-dump-<pid>`). Polls until any matching file is
 * non-empty or the timeout fires; returns the concatenated content.
 *
 * Attempts immediately on entry so the typical case (file already there)
 * doesn't pay the 500ms backoff at all.
 */
export async function readEnvDumpFromRunner(timeoutMs = 30_000): Promise<string> {
  const deadline = Date.now() + timeoutMs;
  let lastErr: string | undefined;
  while (true) {
    try {
      const out = execSync(
        `docker exec ${RUNNER_CONTAINER} sh -c 'cat /tmp/e2e-echo-env-dump-* 2>/dev/null || true'`,
        { encoding: "utf-8" },
      ).trim();
      if (out.length > 0) return out;
    } catch (err) {
      lastErr = (err as Error).message;
    }
    if (Date.now() >= deadline) break;
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(
    `env dump file did not appear in ${RUNNER_CONTAINER} within ${timeoutMs}ms` +
      (lastErr ? ` (last error: ${lastErr})` : ""),
  );
}

/** Wipe any stale dump files from prior runs. */
export function clearRunnerDumps(): void {
  try {
    execSync(
      `docker exec ${RUNNER_CONTAINER} sh -c 'rm -f /tmp/e2e-echo-env-dump-* 2>/dev/null || true'`,
      { encoding: "utf-8" },
    );
  } catch {
    // Container may not be up yet — best effort.
  }
}
