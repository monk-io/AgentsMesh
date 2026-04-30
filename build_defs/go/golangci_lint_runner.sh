#!/usr/bin/env bash
# golangci_lint_runner.sh — sh_binary entrypoint that runs the
# hermetic golangci-lint binary over a Go module rooted at the
# live workspace tree (BUILD_WORKSPACE_DIRECTORY).
#
# Args:
#   $1  rlocation path to the golangci-lint binary
#   $2  workspace-relative module dir (e.g., "backend")
#   $3  rlocation path to the module's .golangci.yml
#
# Extra positional args are forwarded to `golangci-lint run` (e.g.,
# `--fix`).

set -uo pipefail

# Bazel's bash runfiles bootstrap. RUNFILES_DIR is set by sh_binary
# when invoked via `bazel run`. Fall back to RUNFILES_MANIFEST_FILE
# if only the manifest is available (e.g., on platforms without
# directory-based runfiles).
if [[ -z "${RUNFILES_DIR:-}" && -z "${RUNFILES_MANIFEST_FILE:-}" ]]; then
    if [[ -d "${0}.runfiles" ]]; then
        export RUNFILES_DIR="${0}.runfiles"
    elif [[ -f "${0}.runfiles_manifest" ]]; then
        export RUNFILES_MANIFEST_FILE="${0}.runfiles_manifest"
    fi
fi

runfiles_lib=""
if [[ -n "${RUNFILES_DIR:-}" && -f "${RUNFILES_DIR}/bazel_tools/tools/bash/runfiles/runfiles.bash" ]]; then
    runfiles_lib="${RUNFILES_DIR}/bazel_tools/tools/bash/runfiles/runfiles.bash"
elif [[ -n "${RUNFILES_MANIFEST_FILE:-}" ]]; then
    # On manifest-only platforms, the bash helper is alongside the manifest.
    runfiles_lib="$(dirname "$RUNFILES_MANIFEST_FILE")/bazel_tools/tools/bash/runfiles/runfiles.bash"
fi

if [[ -z "$runfiles_lib" || ! -f "$runfiles_lib" ]]; then
    echo "ERROR: bash runfiles helper not found." >&2
    echo "  RUNFILES_DIR=${RUNFILES_DIR:-<unset>}" >&2
    echo "  RUNFILES_MANIFEST_FILE=${RUNFILES_MANIFEST_FILE:-<unset>}" >&2
    exit 1
fi

# shellcheck disable=SC1090
source "$runfiles_lib"

linter_rloc="$1"
module_dir="$2"
config_rloc="$3"
shift 3

linter="$(rlocation "$linter_rloc")"
config="$(rlocation "$config_rloc")"

if [[ -z "$linter" || ! -x "$linter" ]]; then
    echo "ERROR: golangci-lint binary not found at rlocation '$linter_rloc'" >&2
    exit 1
fi
if [[ -z "$config" || ! -f "$config" ]]; then
    echo "ERROR: .golangci.yml not found at rlocation '$config_rloc'" >&2
    exit 1
fi

# `bazel run` sets BUILD_WORKSPACE_DIRECTORY to the workspace root.
# Without it (e.g., direct invocation), bail with a clear message —
# we deliberately don't fall back to runfiles because golangci-lint
# needs the live Go module tree (vendored deps, GOMODCACHE).
if [[ -z "${BUILD_WORKSPACE_DIRECTORY:-}" ]]; then
    echo "ERROR: BUILD_WORKSPACE_DIRECTORY not set — invoke via 'bazel run', not direct exec." >&2
    exit 1
fi

cd "$BUILD_WORKSPACE_DIRECTORY/$module_dir"

echo "::group::golangci-lint $($linter --version | head -1) on $module_dir/"
"$linter" run --config="$config" --timeout=5m "$@" ./...
status=$?
echo "::endgroup::"
exit $status
