"""Rust → iOS XCFramework via a Bazel-invoked shell script.

This macro exposes the legacy `build-ios-xcframework.sh` as a Bazel
target so `bazel run //<pkg>:AgentsMeshCore` builds the framework and
the CI pipeline can drive everything through a single label.

The shell script stays authoritative for the implementation: cargo
build for three iOS triples, lipo the simulator slices into a fat
library, run `uniffi-bindgen` for the Swift glue, and assemble the
XCFramework with `xcodebuild`. Output lives at
`clients/core/ios-framework/AgentsMeshCore.xcframework/` — where the
SPM `binaryTarget` in `clients/ios/Packages/AgentsMeshCore` looks for
it.

Full Bazel-native replacement (Phase 3b of the migration plan)
requires:
  - a `rust_static_library` per-iOS-triple via `platform_transition`,
  - a `lipo_genrule` for the universal simulator slice,
  - `apple_static_xcframework` from the yishuiliunian/rules_apple fork
    with the correct `minimum_os_versions` attr,
  - `uniffi-bindgen-swift` wrapped as a Bazel action.

Until all four land, the sh_binary wrapper below keeps CI + local dev
calling a single Bazel label instead of hunting for the script path.

Usage:
    load("//build_defs/ios:rust_xcframework.bzl", "rust_xcframework")

    rust_xcframework(
        name = "AgentsMeshCore",
        visibility = ["//clients/ios:__subpackages__"],
    )
"""

def rust_xcframework(
        name,
        script = "//clients/core/scripts:build-ios-xcframework.sh",
        visibility = None):
    """Expose the XCFramework build as a `bazel run` target.

    Args:
        name: Target name. `bazel run //<pkg>:<name>` triggers the
            build. Also the output framework's module name.
        script: Label pointing at `build-ios-xcframework.sh`. Can be
            overridden for testing; defaults to the canonical location.
        visibility: Standard visibility.

    Tags:
        - `requires-darwin`  — won't resolve on Linux toolchains
        - `manual`           — excluded from `bazel build //...`
        - `ios`              — lets CI tag-filter this target
        - `no-sandbox`       — the script writes to workspace paths
                               outside the execroot (cargo target/,
                               clients/core/ios-framework/)
    """
    native.sh_binary(
        name = name,
        srcs = [script],
        tags = [
            "requires-darwin",
            "manual",
            "ios",
            "no-sandbox",
        ],
        visibility = visibility,
    )
