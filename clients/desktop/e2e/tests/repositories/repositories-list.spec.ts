import { test, expect } from "../../fixtures";
import { InfraPage } from "../../pages/infra.page";
import { invokeIpc } from "../../helpers/ipc";

// Regression coverage for ElectronRepoState stub bug. Before the
// `same-instance` fix, packages/electron-adapter/src/state_adapters.ts
// shipped an ElectronRepoState whose `repositories_json()` returned "[]"
// and `set_repositories()` was a no-op — IPC `repositoryList` returned
// real backend data, the renderer store called rs().set_repositories(json),
// the stub dropped it, useRepositories() always read "[]". Every desktop
// surface (/infra?tab=repositories, Create Pod dialog, IDE sidebar, command
// palette, ticket filter) showed an empty list.
//
// The previously-existing ipc/_generated/repository.api.spec.ts only proves
// the IPC route is wired ("Result may be a valid response OR a typed error
// — both prove the IPC route is wired") — it never checked the cache layer.
// This spec closes that gap.
test.describe("Desktop infra · repositories list", () => {
  test("renders backend repositories (not the empty stub)", async ({ page }) => {
    // Source of truth: backend via IPC. Empty backend → nothing to assert
    // against, skip rather than flake the suite on dev-env state.
    const raw = await invokeIpc<string>(page, "repositoryList");
    const { repositories = [] } = JSON.parse(raw) as {
      repositories?: { id: number; slug: string }[];
    };
    if (repositories.length === 0) {
      test.skip(true, "backend has no repositories — nothing to render");
      return;
    }

    const infra = new InfraPage(page);
    await infra.gotoTab("repositories");
    await page.waitForLoadState("networkidle");

    // Empty-state heading is the canonical fingerprint of the regression —
    // RepoSection renders it only when `repositories.length === 0` after
    // fetch completes. If the stub is back, fetch writes nowhere and this
    // heading shows even though backend has rows.
    await expect(
      page.getByRole("heading", { name: /no repositories yet|暂无仓库/i }),
      "empty-state heading visible while backend has repos — ElectronRepoState stub regressed?",
    ).toHaveCount(0, { timeout: 10_000 });

    // First repo's slug must appear in the rendered sidebar list.
    // RepositoriesSidebarContent prints `repo.slug` per row.
    const first = repositories[0];
    await expect(
      page.getByText(first.slug, { exact: false }).first(),
      `expected repo slug "${first.slug}" in sidebar list`,
    ).toBeVisible({ timeout: 10_000 });
  });
});
