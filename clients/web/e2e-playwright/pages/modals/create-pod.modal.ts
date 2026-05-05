import type { Page } from "@playwright/test";

/**
 * Page Object Model for the Create Pod Modal.
 * Based on: web/src/components/ide/CreatePodModal.tsx
 *           web/src/components/pod/CreatePodForm/index.tsx
 */
export class CreatePodModal {
  constructor(private page: Page) {}

  /** Wait for the modal dialog to appear. */
  async waitForOpen(): Promise<void> {
    await this.page
      .locator('[role="dialog"]')
      .first()
      .waitFor({ state: "visible" });
  }

  /** Wait for the modal to disappear after a successful submit. */
  async waitForClosed(timeoutMs = 15_000): Promise<void> {
    await this.page
      .locator('[role="dialog"]')
      .first()
      .waitFor({ state: "hidden", timeout: timeoutMs });
  }

  /**
   * Select an agent. The form renders a native `<select id="agent-select">`
   * (web/src/components/pod/CreatePodForm/AgentSelect.tsx) — `<option>` text
   * is not in the visible DOM until the dropdown is opened, so the previous
   * `getByText(...).click()` was a silent no-op and left the Create Pod
   * button disabled.
   */
  async selectAgent(agentSlug: string): Promise<void> {
    const select = this.page
      .locator('[role="dialog"] select#agent-select')
      .first();
    await select.selectOption(agentSlug);
  }

  /** Fill the prompt text area. */
  async fillPrompt(prompt: string): Promise<void> {
    const textarea = this.page.locator("textarea").first();
    await textarea.fill(prompt);
  }

  /** Click the submit/create button. */
  async submit(): Promise<void> {
    const btn = this.page
      .locator('[role="dialog"]')
      .getByRole("button", { name: /create|创建/i });
    await btn.click();
  }

  /** Close the modal. */
  async cancel(): Promise<void> {
    const btn = this.page
      .locator('[role="dialog"]')
      .getByRole("button", { name: /cancel|取消/i });
    if (await btn.isVisible()) await btn.click();
  }
}
