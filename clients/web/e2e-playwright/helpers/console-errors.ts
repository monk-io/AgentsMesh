import type { Page } from "@playwright/test";
import { expect } from "@playwright/test";

export function collectConsoleErrors(page: Page): string[] {
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") errors.push(msg.text());
  });
  return errors;
}

export function assertNoWasmErrors(errors: string[]) {
  const critical = errors.filter(
    (e) =>
      (e.includes("missing field") ||
      e.includes("is not valid JSON") ||
      e.includes("Failed to fetch board") ||
      e.includes("Failed to fetch topology") ||
      e.includes("Failed to load runner") ||
      e.includes("Failed to load repository") ||
      e.includes("Failed to load ticket")) &&
      !e.includes("Failed to load resource")
  );
  expect(critical).toHaveLength(0);
}
