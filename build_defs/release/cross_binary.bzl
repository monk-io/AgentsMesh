"""cross_binary — declare a Go binary cross-compiled for a (goos, goarch) pair.

Wraps `rules_go`'s `go_binary` with `goos`/`goarch` attributes so a
single `go_library` can be built for every GOOS/GOARCH combination
the release pipeline needs. Equivalent to GoReleaser's `builds:`
matrix entry.

Usage:

    load("//build_defs/release:cross_binary.bzl", "cross_binary")

    cross_binary(
        name = "runner_linux_amd64",
        embed = [":runner_lib"],
        goos = "linux",
        goarch = "amd64",
        out = "agentsmesh-runner",
    )

The binary is hermetically built by rules_go's bundled Go SDK. CGO is
disabled by default (matches GoReleaser config `CGO_ENABLED=0`).
"""

load("@rules_go//go:def.bzl", "go_binary")

# (goos, goarch, archive_format) tuples that GoReleaser used to emit.
# Single source of truth — `release_bundle.bzl` iterates this.
SUPPORTED_PLATFORMS = [
    ("linux", "amd64", "tar.gz"),
    ("linux", "arm64", "tar.gz"),
    ("darwin", "amd64", "tar.gz"),
    ("darwin", "arm64", "tar.gz"),
    ("windows", "amd64", "zip"),
    ("windows", "arm64", "zip"),
]

def cross_binary(name, embed, goos, goarch, out, **kwargs):
    """Cross-compile a go_binary for one platform.

    Args:
        name: Target name. Convention: `<binary>_<goos>_<goarch>`.
        embed: List of `go_library` targets the binary embeds.
        goos: GOOS — `linux` / `darwin` / `windows`.
        goarch: GOARCH — `amd64` / `arm64`.
        out: Output filename. Windows automatically appends `.exe`
            via rules_go.
        **kwargs: Forwarded to `go_binary` (e.g., `gc_linkopts`).
    """
    gc_linkopts = kwargs.pop("gc_linkopts", [])

    # `-s -w` matches GoReleaser ldflags — strip symbol + DWARF tables
    # (~30% smaller binary, no impact on runtime).
    gc_linkopts = list(gc_linkopts) + ["-s", "-w"]

    go_binary(
        name = name,
        embed = embed,
        goos = goos,
        goarch = goarch,
        out = out,
        pure = "on",
        gc_linkopts = gc_linkopts,
        **kwargs
    )
