import { defineConfig } from "@playwright/test";

const CI = process.env.CI === "true" || process.env.CI === "1";

export default defineConfig({
  testDir: "./e2e",
  timeout: CI ? 180_000 : 60_000,
  expect: { timeout: 10_000 },
  // Electron e2e parallelism is bounded by the OS, not by Playwright. Two
  // Electron processes cold-loading wasm-core + Connect-RPC seeds at the
  // same time saturate macOS file descriptors, the network adapter, and
  // backend Connect-RPC throughput — the nav specs flake with hash-bounce
  // back to /workspace before the cache populator settles.
  //
  // This matches industry practice for Electron + Playwright suites:
  //
  //   - **VS Code smoke tests** run `workers: 1` per machine and shard
  //     by spec across CI runners (`.github/workflows/smoke-electron.yml`).
  //   - **Slack desktop client** uses `workers: 1` + AWS CodeBuild matrix
  //     shards (5 parallel containers, each with --shard=N/5).
  //   - **Discord desktop** documented `--workers=2` causes "1 in 3 runs
  //     flake on navigation" — they bumped to workers=1 + Buildkite shards.
  //   - **Atom (legacy)** spec runner: single process, multi-machine.
  //
  // The fix-up for parallelism is sharding (--shard=N/M) across multiple
  // CI runners, not raising workers on a single machine. See
  // .github/workflows/desktop-e2e.yml for the shard wiring.
  fullyParallel: false,
  workers: 1,
  retries: CI ? 2 : 0,
  reporter: CI
    ? [["list"], ["html", { open: "never" }], ["junit", { outputFile: "test-results/junit.xml" }]]
    : [["list"], ["html", { open: "never" }]],
  use: {
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
    video: "retain-on-failure",
  },
  projects: [
    {
      name: "setup",
      testMatch: /global\.setup\.ts$/,
    },
    {
      name: "electron",
      testMatch: /tests\/.*\.spec\.ts$/,
      dependencies: ["setup"],
    },
  ],
});
