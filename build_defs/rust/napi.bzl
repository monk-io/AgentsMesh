"""Rust → Node-API (.node) native addon.

Compiles a Rust crate as a `cdylib` and renames the platform-specific
dynamic library (`lib<name>.dylib` / `lib<name>.so` / `<name>.dll`) into
the Node-friendly `<name>.node` suffix that `require()` expects.

Usage:
    load("//build_defs/rust:napi.bzl", "napi_rust_library")

    napi_rust_library(
        name = "node_bridge",
        crate = "//clients/core/crates/node-bridge:node_bridge_lib",
        visibility = ["//clients/desktop:__subpackages__"],
    )

The macro does not attempt to run `@napi-rs/cli`. The Rust `build.rs`
still emits the N-API header bindings; we ship the compiled `.node`
alongside it. TypeScript declarations are expected to be hand-maintained
(or generated separately via `js_run_binary` wrapping `napi build --dts`).

Known limits:
  - Node's N-API ABI must match the Electron build being shipped. Pin
    the target `node_version` in `MODULE.bazel` to match the Electron
    release's node ABI.
  - Cross-compilation is handled by `--platforms` (e.g. `linux/amd64`
    vs `darwin/arm64`); this macro simply packages whatever slice the
    current configuration produces.
"""

load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

def napi_rust_library(
        name,
        srcs,
        deps = None,
        proc_macro_deps = None,
        crate_name = None,
        edition = "2021",
        ts_declaration = None,
        visibility = None):
    """Produce a Node-loadable `.node` file from a Rust source set.

    Args:
        name: Package name (becomes the `.node` filename).
        srcs: Rust source files for the crate.
        deps: Rust library dependencies.
        proc_macro_deps: Rust proc-macro deps (e.g. napi-derive).
        crate_name: Override for the emitted crate name. Defaults to `name`.
        edition: Rust edition. Defaults to 2021.
        ts_declaration: Optional label pointing at a hand-written `.d.ts`
            file to copy next to the `.node`. If absent, the package
            exports the addon only.
        visibility: Standard visibility attribute.
    """
    rust_shared_library(
        name = "_{}_cdylib".format(name),
        srcs = srcs,
        deps = deps or [],
        proc_macro_deps = proc_macro_deps or [],
        crate_name = crate_name or name,
        edition = edition,
    )

    # Rename the platform-specific suffix (dylib/so/dll) → .node. Node's
    # loader resolves require() paths by extension so this is the minimum
    # friction: no JS wrapper needed on the consumer side.
    native.genrule(
        name = "_{}_rename".format(name),
        srcs = [":_{}_cdylib".format(name)],
        outs = ["{}.node".format(name)],
        cmd = "cp $(SRCS) $@",
    )

    data = [":_{}_rename".format(name)]
    if ts_declaration:
        copy_to_bin(
            name = "_{}_dts".format(name),
            srcs = [ts_declaration],
        )
        data.append(":_{}_dts".format(name))

    native.filegroup(
        name = name,
        srcs = data,
        visibility = visibility or ["//visibility:public"],
    )
