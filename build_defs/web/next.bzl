"""Next.js macro for AgentsMesh — ported from AIO with minimal tweaks.

For each `next_app()` call, produces three targets:
  :<name>            js_run_binary running `next build` (outputs `.next/`)
  :<name>_dev        devserver running `next dev`
  :<name>_start      devserver running `next start` against the build

Used by clients/web and clients/web-admin. The Electron renderer bundle is
produced separately via build_defs/web/electron.bzl (which also calls
next_app under the hood).

Usage:
    load("//build_defs/web:next.bzl", "next_app")

    next_app(
        name = "next",
        srcs = [":src", "//clients/core/crates/wasm:wasm_pkg"],
        data = [
            "next.config.ts",
            "package.json",
            "//:node_modules/next",
            ...
        ],
        next_js_binary = "//:node_modules/next/package:next_bin",
        next_bin = "./node_modules/.bin/next",
    )
"""

load("@aspect_rules_js//js:defs.bzl", "js_binary", "js_run_binary", "js_run_devserver")

def next_app(
        name,
        srcs,
        data,
        next_js_binary,
        next_build_out = ".next",
        **kwargs):
    """Build / dev / start targets for a Next.js app.

    Args:
        name: Base target name (`:name` is the production build).
        srcs: Application source files (usually `ts_project` outputs).
        data: Runtime deps (node_modules, config files).
        next_js_binary: `js_binary` pointing at Next's entry.
        next_build_out: Build artifact dir, defaults to `.next`.
        **kwargs: Tags + visibility forwarded to all targets.
    """
    tags = kwargs.pop("tags", [])

    js_run_binary(
        name = name,
        args = ["build"],
        chdir = native.package_name(),
        mnemonic = "NextJsBuild",
        out_dirs = [next_build_out],
        progress_message = "Building Next.js application",
        srcs = srcs + data,
        tags = tags,
        tool = next_js_binary,
        **kwargs
    )

    js_run_devserver(
        name = "{}_dev".format(name),
        args = ["dev"],
        chdir = native.package_name(),
        data = srcs + data,
        grant_sandbox_write_permissions = True,
        tags = tags,
        tool = next_js_binary,
        **kwargs
    )

    js_run_devserver(
        name = "{}_start".format(name),
        args = ["start"],
        chdir = native.package_name(),
        data = data + [name],
        tags = tags,
        tool = next_js_binary,
        **kwargs
    )

    # The OCI image layer needs a plain `js_binary`, not a devserver.
    js_binary(
        name = "{}_start_binary".format(name),
        chdir = native.package_name(),
        data = data + [name],
        entry_point = "start.js",
        tags = tags,
        **kwargs
    )
