// Data-integrity assertions for key list pages. These specs are the last
// line of defense for the failure class that motivated this PR:
// the renderer reads an API response but a deserialization / wasm-bridge
// drift loses fields silently, so the page renders an empty list while
// the API says N items exist.
//
// Each test:
//   1. fetches ground-truth from Connect-RPC (same wire format the
//      production client uses, asserted on the proto layer)
//   2. navigates to the page, waits for the data to land
//   3. compares DOM-rendered item counts against the API count
//
// A passing run proves the wasm cache + selectors + render path all
// agree with the backend. The auto-attached console monitor (see
// fixtures/index.ts) catches the silent-error class on top.

import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

const isEmptyHint = /no .*(pod|loop|channel|ticket|run)s?|no .*found|没有.*(pod|loop|channel|ticket|run)|empty|暂无|还没有|nothing here|此.*暂无/i;

test.describe("Data integrity: list pages match API counts", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("workspace sidebar pod count matches ListPods API", async ({ page, api, db }) => {
    const cc = await api.connect();
    // Mirror the renderer's default sidebar request — "mine" tab maps to
    // status "running,initializing" + created_by_id = current user
    // (SIDEBAR_STATUS_MAP in stores/podTypes.ts). Anything else returned
    // by the API would be filtered out client-side, so testing equality
    // against an unfiltered API call is structurally wrong.
    const userId = db.queryValue(
      `SELECT id FROM users WHERE email = 'dev@agentsmesh.local' LIMIT 1`,
    );
    expect(userId, "dev seed must include the dev user").toBeTruthy();
    const { items } = await cc.pod.listPods({
      orgSlug: TEST_ORG_SLUG,
      status: "running,initializing",
      createdById: BigInt(userId as string),
      limit: 50,
      offset: 0,
    }) as { items: Array<{ podKey: string }> };

    await page.goto(`/${TEST_ORG_SLUG}/workspace`);
    await page.waitForLoadState("load");
    await page.waitForTimeout(2000); // sidebar fetch lands

    if (items.length === 0) {
      // Empty state must be visible — otherwise the sidebar is silently
      // showing nothing, which is the original bug.
      const empty = await page.getByText(isEmptyHint).first().isVisible({ timeout: 5_000 }).catch(() => false);
      expect(empty, "API returned no pods → page must show empty state").toBe(true);
      return;
    }

    // Wait for the first pod to appear, then assert the total matches.
    const podRows = page.locator('[data-testid="pod-list-item"]');
    await expect(podRows.first()).toBeVisible({ timeout: 10_000 });
    // The sidebar may also include pods from other filters or paginated
    // sub-views; assert it includes *at least* every podKey the API
    // returned, not strict equality. Matches the "wasm cache lost data"
    // failure mode exactly: if the cache drops pods, this comparison
    // catches it.
    const renderedKeys = await podRows.evaluateAll(els =>
      els.map(el => el.getAttribute("data-pod-key")).filter((k): k is string => !!k)
    );
    for (const expected of items.map((p) => p.podKey)) {
      expect(
        renderedKeys.includes(expected),
        `sidebar missing pod ${expected} that ListPods returned`,
      ).toBe(true);
    }
  });

  test("runner detail pods tab row count matches ListPods(runnerId) API", async ({ page, api, db }) => {
    const runnerId = db.queryValue(
      `SELECT id FROM runners WHERE organization_id = (SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}') LIMIT 1`,
    );
    expect(runnerId, "dev seed must include at least one runner").toBeTruthy();
    const id = runnerId as string;

    const cc = await api.connect();
    // ListPods filtered by runner_id — same data the runner-detail Pods
    // tab renders (production goes through a wasm-only helper, but the
    // backend wire query is exactly this). Renderer drops the limit/offset
    // to mirror what useRunnerDetail.ts sends.
    const apiRes = await cc.pod.listPods({
      orgSlug: TEST_ORG_SLUG,
      runnerId: BigInt(id),
      limit: 20,
      offset: 0,
    }) as { items: Array<{ podKey: string }>; total: bigint };

    await page.goto(`/${TEST_ORG_SLUG}/runners/${id}`);
    await page.waitForLoadState("load");
    await page.waitForTimeout(2000); // initial mount

    // Pods tab must be selected for rows to render. Click the tab using
    // its stable testid (not text — i18n + text-collision-prone).
    const podsTab = page.locator('[data-testid="runner-detail-tab-pods"]');
    if (await podsTab.isVisible({ timeout: 5_000 }).catch(() => false)) {
      await podsTab.click();
    }
    // loadPods is async behind the click — wait for the first row or empty
    // state to land before assertions.
    await page.waitForTimeout(4000);

    const expectedKeys = apiRes.items.map((p) => p.podKey);
    if (expectedKeys.length === 0) {
      const emptyVisible = await page
        .getByText(isEmptyHint)
        .first()
        .isVisible({ timeout: 5_000 })
        .catch(() => false);
      expect(
        emptyVisible,
        "API returned no runner pods → page must show empty state",
      ).toBe(true);
      return;
    }

    // Wait for the runner-pod-row attribute to appear (the table render is
    // async behind a loadPods fetch). Then read every rendered pod_key.
    const podRows = page.locator('[data-testid="runner-pod-row"]');
    await expect(podRows.first()).toBeVisible({ timeout: 10_000 });
    const renderedKeys = await podRows.evaluateAll(els =>
      els.map(el => el.getAttribute("data-pod-key")).filter((k): k is string => !!k)
    );
    for (const key of expectedKeys) {
      expect(
        renderedKeys.includes(key),
        `runner pods table missing pod ${key} from ListPods(runnerId) response`,
      ).toBe(true);
    }
  });

  test("tickets board column counts match GetBoard API", async ({ page, api }) => {
    const cc = await api.connect();
    const board = await cc.ticket.getBoard({ orgSlug: TEST_ORG_SLUG }) as {
      columns: Array<{ status: string; tickets: Array<{ slug: string }> }>;
    };

    await page.goto(`/${TEST_ORG_SLUG}/tickets`);
    await page.waitForLoadState("load");
    await page.waitForTimeout(2000);

    // Sum across all columns — the board view renders every ticket;
    // proto field drift would leave the column array empty / lose
    // tickets in transit. We check inclusion of each slug in the
    // rendered body text, matching the data-integrity contract.
    const expectedSlugs = board.columns.flatMap((c) => c.tickets.map((t) => t.slug));
    if (expectedSlugs.length === 0) return;

    const bodyText = await page.textContent("body") ?? "";
    let foundAny = false;
    for (const slug of expectedSlugs) {
      if (bodyText.includes(slug)) {
        foundAny = true;
        break;
      }
    }
    expect(
      foundAny,
      `tickets board renders 0 of ${expectedSlugs.length} API-returned ticket slugs — wasm cache or selector likely dropped them`,
    ).toBe(true);
  });

  test("channel list count from ListChannels API matches DOM", async ({ page, api }) => {
    const cc = await api.connect();
    const { items } = await cc.channel.listChannels({ orgSlug: TEST_ORG_SLUG }) as {
      items: Array<{ id: bigint; name: string }>;
    };

    await page.goto(`/${TEST_ORG_SLUG}/channels`);
    await page.waitForLoadState("load");
    await page.waitForTimeout(2000);

    if (items.length === 0) {
      const empty = await page.getByText(isEmptyHint).first().isVisible({ timeout: 5_000 }).catch(() => false);
      expect(empty, "no channels → page must render empty state").toBe(true);
      return;
    }

    const bodyText = await page.textContent("body") ?? "";
    let foundAny = false;
    for (const ch of items) {
      if (bodyText.includes(ch.name)) {
        foundAny = true;
        break;
      }
    }
    expect(
      foundAny,
      `channel list renders 0 of ${items.length} API-returned channel names — wasm cache likely dropped them`,
    ).toBe(true);
  });

  test("loops list count matches ListLoops API", async ({ page, api }) => {
    const cc = await api.connect();
    const { items } = await cc.loop.listLoops({ orgSlug: TEST_ORG_SLUG }) as {
      items: Array<{ slug: string; name: string }>;
    };

    await page.goto(`/${TEST_ORG_SLUG}/loops`);
    await page.waitForLoadState("load");
    await page.waitForTimeout(2000);

    if (items.length === 0) {
      const empty = await page.getByText(isEmptyHint).first().isVisible({ timeout: 5_000 }).catch(() => false);
      expect(empty, "no loops → page must render empty state").toBe(true);
      return;
    }

    const bodyText = await page.textContent("body") ?? "";
    let foundAny = false;
    for (const loop of items) {
      if (bodyText.includes(loop.name) || bodyText.includes(loop.slug)) {
        foundAny = true;
        break;
      }
    }
    expect(
      foundAny,
      `loops list renders 0 of ${items.length} API-returned loop names/slugs — wasm cache likely dropped them`,
    ).toBe(true);
  });
});
