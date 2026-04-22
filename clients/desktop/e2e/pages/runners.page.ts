import type { Locator, Page } from "@playwright/test";
import { TEST_ORG_SLUG } from "../helpers/env";
import { gotoHash, expectHashMatches } from "../helpers/nav";

/**
 * Page Object for Runners list + detail.
 * Route: #/:org/runners, #/:org/runners/:id
 */
export class RunnersPage {
  readonly runnerRows: Locator;
  readonly addRunnerButton: Locator;
  readonly statusBadges: Locator;

  constructor(private page: Page) {
    this.runnerRows = page.locator('[data-row-type="runner"], tbody tr');
    this.addRunnerButton = page.getByRole("button", { name: /add runner|register runner|新建/i });
    this.statusBadges = page.locator('[data-status]');
  }

  async goto(): Promise<void> {
    await gotoHash(this.page, `/${TEST_ORG_SLUG}/runners`);
  }

  async expectOnPage(): Promise<void> {
    await expectHashMatches(this.page, /\/runners/);
  }

  async openRunner(id: string): Promise<void> {
    await gotoHash(this.page, `/${TEST_ORG_SLUG}/runners/${id}`);
  }
}
