#!/usr/bin/env python3
"""One-shot helper to mark legacy build scripts / Dockerfiles deprecated.

Prepends a block-comment header noting the Bazel target that replaces
each file. Skip paths that already carry the header.

Usage:
    python3 scripts/mark_deprecated.py
"""
import pathlib

MAPPINGS = {
    "ci/backend.Dockerfile":   ("//backend/cmd/server:image",    "#"),
    "ci/runner.Dockerfile":    ("//runner/cmd/runner:image",     "#"),
    "ci/relay.Dockerfile":     ("//relay/cmd/relay:image",       "#"),
    "ci/web.Dockerfile":       ("//clients/web:image",           "#"),
    "ci/web-admin.Dockerfile": ("//clients/web-admin:image",     "#"),
    "ci/build-onpremise.sh":   ("//deploy/onpremise:bundle",     "#"),
    "ci/pack-onpremise.sh":    ("//deploy/onpremise:bundle",     "#"),
    "clients/core/scripts/build-ios-xcframework.sh":
        ("bazel build //clients/core/crates/ffi:AgentsMeshCore", "#"),
    "clients/ios/Makefile":
        ("//clients/ios:AgentsMesh (build) and //clients/ios:AgentsMesh_xcodeproj (IDE)", "#"),
    "clients/ios/project.yml":
        ("//clients/ios:AgentsMesh_xcodeproj (generated via rules_xcodeproj)", "#"),
}

MARKER = "DEPRECATED --- Bazel migration"

def stamp(path: str, bazel_target: str, comment: str) -> None:
    p = pathlib.Path(path)
    if not p.exists():
        print(f"skip (missing): {path}")
        return
    text = p.read_text()
    if MARKER in text:
        print(f"skip (already stamped): {path}")
        return
    # Preserve shebang if present.
    lines = text.splitlines(keepends=True)
    shebang = ""
    if lines and lines[0].startswith("#!"):
        shebang = lines[0]
        rest = "".join(lines[1:])
    else:
        rest = text
    header = (
        f"{comment} {MARKER}\n"
        f"{comment} Replacement: {bazel_target}\n"
        f"{comment} Kept until .github/workflows/bazel.yml is authoritative, then delete.\n"
        f"{comment}\n"
    )
    p.write_text(shebang + header + rest)
    print(f"stamped: {path}")


if __name__ == "__main__":
    for path, (target, comment) in MAPPINGS.items():
        stamp(path, target, comment)
