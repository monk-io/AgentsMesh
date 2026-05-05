"""golangci_lint — run hermetic golangci-lint over a Go module.

Wraps `@multitool//tools/golangci-lint` in a `sh_binary` invokable via
`bazel run //backend:lint`. CI calls the same command instead of
`golangci-lint-action@v9`, so the linter version + flags + exclusion
rules live in `multitool.lock.json` + `<module>/.golangci.yml` —
hermetic on every developer / CI machine.

Why a binary, not a test:
- golangci-lint walks the full Go module dep graph (GOMODCACHE,
  vendored deps, generated code). The Bazel test sandbox strips
  symlinks Go's loader needs, and re-materializing the entire dep
  graph as runfiles per module is wasteful.
- `bazel run` exposes `BUILD_WORKSPACE_DIRECTORY`, so the lint runs
  against the live source tree just like the previous
  `golangci-lint-action`. Exit code propagates naturally → CI fails
  on lint errors.

Usage (from `backend/BUILD.bazel`):

    load("//build_defs/go:golangci_lint.bzl", "golangci_lint")

    golangci_lint(
        name = "lint",
        config = ".golangci.yml",
    )

Then `bazel run //backend:lint` runs golangci-lint over the entire
`backend/` module. No args needed; pass extra flags after `--`:

    bazel run //backend:lint -- --fix
"""

def golangci_lint(
        name,
        config,
        module_dir = None,
        **kwargs):
    """Run golangci-lint over a Go module.

    Args:
        name: Binary target name. Convention: `lint`.
        config: Path to the module's `.golangci.yml`, package-relative.
        module_dir: Workspace-relative dir to cd into before running
            golangci-lint. Defaults to the calling package (the
            common case — top-level module BUILD.bazel).
        **kwargs: Forwarded to `native.sh_binary` (e.g., `tags`,
            `visibility`).
    """
    if module_dir == None:
        module_dir = native.package_name()

    native.sh_binary(
        name = name,
        srcs = ["//build_defs/go:golangci_lint_runner.sh"],
        data = [
            config,
            "@multitool//tools/golangci-lint",
        ],
        args = [
            "$(rlocationpath @multitool//tools/golangci-lint)",
            module_dir,
            "$(rlocationpath :%s)" % config,
        ],
        deps = ["@bazel_tools//tools/bash/runfiles"],
        **kwargs
    )
