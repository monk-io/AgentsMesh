"""Rust → iOS XCFramework with UniFFI Swift bindings.

Takes one Rust crate and produces:
  - <name>.xcframework  (static library, ios-arm64 device + universal sim)
  - <name>_swift        (Swift glue + C headers + modulemap)
  - <name>_swift_pkg    (filegroup for SwiftPM consumption)

Mirrors the hand-rolled `clients/core/scripts/build-ios-xcframework.sh`
flow but scopes to Bazel's action graph so a change in any Rust source
triggers the XCFramework to regenerate — and Swift/iOS targets depending
on it rebuild automatically.

Usage:
    load("//build_defs/ios:rust_xcframework.bzl", "rust_xcframework")

    rust_xcframework(
        name = "AgentsMeshCore",
        crate = "//clients/core/crates/ffi:ffi",
        module_name = "AgentsMeshCoreFFI",
        visibility = ["//clients/ios:__subpackages__"],
    )

Implementation notes:
  - rules_rust 0.68+ builds per-target static libs via `platform_transition`.
  - `apple_static_xcframework` from rules_apple (forked) ties the slices
    together; we pass the universal-simulator slice built via a lipo genrule.
  - `uniffi_bindgen_swift` is a custom genrule that shells out to
    `bazel run //clients/core/crates/uniffi-bindgen:uniffi-bindgen --
    generate --library ... --language swift`.

Known caveats:
  - `uniffi-bindgen-swift` 0.29 writes C headers + modulemap using the
    crate name as the module prefix. Override via `module_name` when a
    Swift package wants a different import path.
  - The fork of `rules_apple` (yishuiliunian/rules_apple) is required for
    `rules_swift@3.0.2` compatibility. See MODULE.bazel for the pin.
"""

load("@build_bazel_rules_apple//apple:apple.bzl", "apple_static_xcframework")
load("@rules_rust//rust:defs.bzl", "rust_static_library")

_IOS_TRIPLES = struct(
    device = "@rules_rust//rust/platform:aarch64-apple-ios",
    sim_arm64 = "@rules_rust//rust/platform:aarch64-apple-ios-sim",
    sim_x86_64 = "@rules_rust//rust/platform:x86_64-apple-ios",
)

def rust_xcframework(
        name,
        crate,
        module_name = None,
        visibility = None):
    """Bundle a Rust crate into an iOS XCFramework + Swift bindings.

    Args:
        name: The XCFramework name (also used as the module identifier).
        crate: Label of the `rust_library` whose `crate_type` includes
            `"staticlib"`. The crate's `uniffi::setup_scaffolding!()` call
            drives Swift binding generation.
        module_name: Override the Swift module name. Defaults to `name`.
        visibility: Standard visibility attribute.
    """
    module = module_name or name

    # 1. Build the three slices.
    for slice_name, platform in (
        ("device_staticlib", _IOS_TRIPLES.device),
        ("sim_arm64_staticlib", _IOS_TRIPLES.sim_arm64),
        ("sim_x86_64_staticlib", _IOS_TRIPLES.sim_x86_64),
    ):
        rust_static_library(
            name = "_{}_{}".format(name, slice_name),
            crate = crate,
            target_compatible_with = ["@platforms//os:ios"],
            platform = platform,
        )

    # 2. Merge the two simulator slices so we can ship a single
    #    `ios-arm64_x86_64-simulator` folder inside the XCFramework.
    native.genrule(
        name = "_{}_sim_universal".format(name),
        srcs = [
            ":_{}_sim_arm64_staticlib".format(name),
            ":_{}_sim_x86_64_staticlib".format(name),
        ],
        outs = ["_{}_sim_universal.a".format(name)],
        cmd = "lipo -create $(SRCS) -output $@",
        tools = [],
    )

    # 3. Generate Swift bindings via our uniffi-bindgen CLI target.
    #    `--library` mode parses the crate's metadata directly.
    native.genrule(
        name = "_{}_swift".format(name),
        srcs = [":_{}_device_staticlib".format(name)],
        outs = [
            "_{}_swift/{}.swift".format(name, module),
            "_{}_swift/{}FFI.h".format(name, module),
            "_{}_swift/{}FFI.modulemap".format(name, module),
        ],
        cmd = """
            $(location //clients/core/crates/uniffi-bindgen:uniffi-bindgen) \\
                generate \\
                --library $(location :_{name}_device_staticlib) \\
                --language swift \\
                --out-dir $$(dirname $(location _{name}_swift/{module}.swift))
        """.format(name = name, module = module),
        tools = ["//clients/core/crates/uniffi-bindgen:uniffi-bindgen"],
    )

    # 4. Assemble the XCFramework with device + universal-sim slices.
    apple_static_xcframework(
        name = name,
        ios = {
            "simulator": [
                "aarch64-apple-ios-sim",
                "x86_64-apple-ios",
            ],
            "device": ["aarch64-apple-ios"],
        },
        deps = [
            ":_{}_device_staticlib".format(name),
            ":_{}_sim_universal".format(name),
        ],
        public_hdrs = [":_{}_swift".format(name)],
        visibility = visibility or ["//visibility:public"],
    )

    # 5. Expose the Swift glue as a filegroup so SwiftPM / swift_library
    #    can drop it into a target module.
    native.filegroup(
        name = "{}_swift".format(name),
        srcs = [":_{}_swift".format(name)],
        visibility = visibility or ["//visibility:public"],
    )
