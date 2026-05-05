import type { Locator, Page } from "@playwright/test";
import { TEST_ORG_SLUG } from "../helpers/env";
import { gotoHash, expectHashMatches } from "../helpers/nav";

/**
 * Page Object for Workspace (Pod list + terminal panes).
 * Route: #/:org/workspace
 */
export class WorkspacePage {
  readonly createPodButton: Locator;
  readonly podList: Locator;
  readonly terminalPane: Locator;
  readonly emptyStateCta: Locator;

  constructor(private page: Page) {
    this.createPodButton = page.getByRole("button", { name: /新建 Pod|create pod|create new pod|新建/i }).first();
    this.podList = page.locator('[class*="pod-list"], [data-slot="pod-list"]').first();
    this.terminalPane = page.locator(".xterm, .xterm-viewport").first();
    this.emptyStateCta = page.getByRole("button", { name: /创建新 Pod|create new pod/i });
  }

  async goto(): Promise<void> {
    await gotoHash(this.page, `/${TEST_ORG_SLUG}/workspace`);
  }

  async expectOnPage(): Promise<void> {
    await expectHashMatches(this.page, /\/workspace/);
  }

  async openCreatePodModal(): Promise<void> {
    await this.createPodButton.click();
  }

  async selectPodByKey(podKey: string): Promise<void> {
    await this.page.getByText(podKey, { exact: false }).first().click();
  }

  async expectTerminalVisible(): Promise<void> {
    await this.terminalPane.waitFor({ state: "visible", timeout: 15_000 });
  }
}
