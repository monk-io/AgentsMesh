import type { Page } from "@playwright/test";

/**
 * Wait for the renderer's wasm-core to finish bootstrapping. The hash router's
 * route guards (Connect-RPC cache populators) only run once wasm-core is ready;
 * before that, navigations bounce back to `/workspace`. We expose a boolean
 * `window.__amesh_ready__` set by `markServiceReady()` (see
 * packages/service-runtime/src/service-getters.ts) so tests can synchronize
 * with the actual readiness state instead of polling the URL hash.
 *
 * Falls back to a 200ms grace period if the renderer's main bundle is from
 * before the readiness flag landed (older snapshots in CI cache).
 */
async function waitForServicesReady(page: import("@playwright/test").Page, timeoutMs: number) {
  try {
    await page.waitForFunction(
      // The flag is mirrored to globalThis by markServiceReady() so SSR /
      // dev-chunk dual-evaluations both see it.
      () => Boolean((globalThis as { __amesh_ready__?: boolean }).__amesh_ready__),
      undefined,
      { timeout: timeoutMs },
    );
  } catch {
    // Older bundles without the flag — fall through with a short grace.
    await page.waitForTimeout(200);
  }
}

/** Navigate the Electron renderer via hash router (react-router-dom createHashRouter).
 *
 * Multi-worker contract: under `--workers > 1` the system load doubles every
 * cold-loading Electron — wasm-core init + Connect-RPC seed fetches that
 * normally complete in ~3s spread to 10-20s with 4 workers cold-launching at
 * once. The retry loop here is sized for that worst case (5 attempts × 6s =
 * 30s ceiling) and waits on the application's readiness flag before each
 * attempt so we don't burn retry budget while wasm is still loading.
 */
export async function gotoHash(page: Page, path: string): Promise<void> {
  const normalized = path.startsWith("#") ? path : `#${path.startsWith("/") ? path : `/${path}`}`;
  const substring = normalized.slice(1);

  // Wait for the renderer to finish its first paint + service registration
  // before kicking off the navigation. Cold-load under multi-worker pressure
  // can take 15-25s on macOS — this is the budget that ate the bare-bones
  // retry loop before.
  await waitForServicesReady(page, 30_000);

  const setOnce = async () => {
    await page.evaluate((h) => {
      const prevHash = window.location.hash;
      window.location.hash = h;
      if (prevHash === h) {
        window.dispatchEvent(new HashChangeEvent("hashchange", { oldURL: prevHash, newURL: h }));
      }
    }, normalized);
  };

  await setOnce();
  try {
    await page.waitForFunction(
      (sub) => window.location.hash.includes(sub),
      substring,
      { timeout: 10_000 },
    );
    return;
  } catch { /* fall through to retry loop */ }

  // Retry loop: dashboard routes can bounce back to /workspace on cold
  // boot when the Connect-RPC adapters race with the route guard. Hammer
  // the hash a few times — once the cache populator settles, the next
  // setHash sticks.
  //
  // Sized for parallel cold-load: 5 retries × 6s timeout = 30s ceiling.
  for (let i = 0; i < 5; i++) {
    // Backoff: 800ms → 1s → 1.5s → 2s → 2.5s. Total inter-retry sleep ~8s
    // which gives a slow worker time to finish its wasm init before we
    // try again.
    await page.waitForTimeout(800 + i * 300);
    await setOnce();
    try {
      await page.waitForFunction(
        (sub) => window.location.hash.includes(sub),
        substring,
        { timeout: 6_000 },
      );
      return;
    } catch { /* try again */ }
  }

  // Final attempt — let it throw with the diagnostic message.
  await page.waitForFunction(
    (sub) => window.location.hash.includes(sub),
    substring,
    { timeout: 6_000 },
  );
}

/** Wait until window.location.hash includes the substring. */
export async function waitForHash(
  page: Page,
  substring: string,
  timeout = 20_000
): Promise<void> {
  await page.waitForFunction(
    (sub) => window.location.hash.includes(sub),
    substring,
    { timeout }
  );
}

/** Return current hash path without the leading '#'. */
export async function currentRoute(page: Page): Promise<string> {
  return page.evaluate(() => window.location.hash.replace(/^#/, ""));
}

/** Wait for the hash to match the regex; returns the matched hash.
 *
 * Cold-load contract: the application's route guard can bounce the user back
 * to `/workspace` after the initial hash set if the Connect-RPC cache
 * populator is still in flight. We give up to `timeout` ms for the hash to
 * settle on a value that matches the regex, polling at 250ms intervals (the
 * default for `waitForFunction`). For navigation specs, callers pass
 * `timeout = 40_000` to cover multi-worker cold loads.
 */
export async function expectHashMatches(
  page: Page,
  re: RegExp,
  timeout = 40_000,
): Promise<string> {
  try {
    await page.waitForFunction(
      ({ src, flags }) => new RegExp(src, flags).test(window.location.hash),
      { src: re.source, flags: re.flags },
      { timeout }
    );
  } catch (e) {
    // Diagnostic: report the actual hash + console errors so the test log
    // shows the real bounce target (e.g. "stuck on /workspace expected
    // /loops"), not a generic timeout.
    const actual = await page.evaluate(() => window.location.hash).catch(() => "<page closed>");
    throw new Error(
      `expectHashMatches timeout ${timeout}ms: expected hash matching ${re}, ` +
        `got "${actual}". Possible cause: route guard bounced back to default ` +
        `route while a Connect-RPC cache populator was still in flight. ` +
        `Original error: ${e instanceof Error ? e.message : String(e)}`,
    );
  }
  return currentRoute(page);
}
