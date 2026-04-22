import { defineConfig, devices } from "@playwright/test";

// E2E tests run against the dev docker stack (Traefik fronts the Next.js
// frontend and the Go backend). Ports come from deploy/dev/.env and are
// worktree-scoped; WEB_PORT / HTTP_PORT change per worktree so we read
// them from the environment when set and fall back to this worktree's
// baseline otherwise.
const WEB_PORT = process.env.WEB_PORT ?? "15307";
const API_PORT = process.env.HTTP_PORT ?? "15300";

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: false, // serial: shared backend state between specs
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [["list"]],
  timeout: 60_000,
  use: {
    baseURL: `http://localhost:${WEB_PORT}`,
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
    extraHTTPHeaders: {
      // Dev environment has open CORS; our tests inject Authorization per
      // request via a shared fixture, not here.
    },
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  // Environment variables referenced by individual specs.
  metadata: {
    API_BASE: `http://localhost:${API_PORT}`,
    ORG_SLUG: "dev-org",
    DEV_USER_EMAIL: "dev@agentsmesh.local",
    DEV_USER_PASSWORD: "devpass123",
  },
});
