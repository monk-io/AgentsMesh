import type { Locator, Page } from "@playwright/test";
import { TEST_ORG_SLUG } from "../helpers/env";
import { gotoHash, expectHashMatches } from "../helpers/nav";

/**
 * Page Object for Repositories list + detail.
 * Route: #/:org/repositories, #/:org/repositories/:id
 */
export class RepositoriesPage {
  readonly repoRows: Locator;
  readonly importButton: Locator;

  constructor(private page: Page) {
    this.repoRows = page.locator('[data-row-type="repo"], tbody tr');
    this.importButton = page.getByRole("button", { name: /import|add repo|connect/i });
  }

  async goto(): Promise<void> {
    await gotoHash(this.page, `/${TEST_ORG_SLUG}/repositories`);
  }

  async expectOnPage(): Promise<void> {
    await expectHashMatches(this.page, /\/repositories/);
  }

  async openRepo(id: string): Promise<void> {
    await gotoHash(this.page, `/${TEST_ORG_SLUG}/repositories/${id}`);
  }
}
