import { test, expect } from "../../fixtures";
import { LoopsPage } from "../../pages/loops.page";

// Render-layer coverage for the Loops desktop state adapter. The loops IDE
// sidebar (components/ide/sidebar/LoopsSidebarContent.tsx) lists one row per
// loop from useLoops() — dev seed has 'nightly-dependency-audit', so the
// sidebar must render at least one loop row. A drifted/stub ElectronLoopState
// that dropped the fetched loops would show the empty placeholder instead.
// Guards the same bug class as repositories-list / tickets-board render specs.
test.describe("Desktop loops · list render", () => {
  test("loops sidebar renders the seeded loop (not empty)", async ({ page }) => {
    const loops = new LoopsPage(page);
    await loops.goto();
    await loops.expectOnPage();

    await expect(
      loops.loopRows.first(),
      "no loop rows while backend has a seeded loop — loops state adapter dropped the fetch?",
    ).toBeVisible({ timeout: 10_000 });
    expect(await loops.loopRows.count()).toBeGreaterThan(0);
  });
});
