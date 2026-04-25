"""Electron desktop app packaging for AgentsMesh.

Wraps the electron-vite build pipeline (main + preload + renderer) and
electron-builder packaging into Bazel actions. The `electron_vite_build`
macro produces the `out/` tree (main process + preload + renderer
bundles) inside `bazel-bin/`, and `electron_app` chains that into a
hermetic `pkg_tar` of the final layout that electron-builder can
consume downstream.

The legacy thin `electron_app` (just pkg_tar of pre-built artifacts)
is retained for callers that still ship a manually-built `out/`.

Usage:
    load("//build_defs/web:electron.bzl", "electron_vite_build", "electron_app")

    electron_vite_build(
        name = "out",
        srcs = glob(["src/**/*"]) + ["electron.vite.config.ts", "tsconfig.json"],
    )

    electron_app(
        name = "desktop",
        main = ":main_bundle",
        renderer = ":renderer_bundle",
        napi_addon = "//clients/core/crates/node-bridge:node_bridge",
    )
"""

load("@aspect_rules_js//js:defs.bzl", "js_run_binary", "js_run_devserver")
load("@npm//:electron-vite/package_json.bzl", electron_vite_bin = "bin")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

def electron_vite_build(
        name,
        srcs,
        config = "electron.vite.config.ts",
        out_dir = "out",
        chdir = None,
        visibility = None,
        **kwargs):
    """Run `electron-vite build` over the package as a Bazel action.

    Produces a tree artifact (`out_dir`) containing:
        main/index.js          — main-process bundle
        preload/index.js       — preload bundle
        renderer/              — renderer assets

    Args:
        name: Target name. Output tree label is `:<name>`.
        srcs: All source files electron-vite needs (TS sources +
            config + tsconfig). Pass via `glob(["src/**/*", ...])`.
        config: electron-vite config filename. Default
            `electron.vite.config.ts`.
        out_dir: Output directory name (default `out`). Bazel tree
            artifact created relative to bazel-bin/<package>/.
        chdir: CWD electron-vite runs in. Defaults to the calling
            package.
        visibility: Standard visibility.
        **kwargs: Forwarded to `js_run_binary`.
    """
    electron_vite_bin.electron_vite_binary(
        name = name + "_bin",
        visibility = ["//visibility:private"],
    )

    js_run_binary(
        name = name,
        srcs = srcs + ["//:node_modules"],
        args = ["build"],
        chdir = chdir or native.package_name(),
        out_dirs = [out_dir],
        tool = ":" + name + "_bin",
        visibility = visibility,
        **kwargs
    )

def electron_app(
        name,
        main,
        renderer,
        napi_addon = None,
        visibility = ["//visibility:public"],
        **kwargs):
    """Assemble an Electron app on-disk layout (legacy stub).

    Bundles already-built artifacts via pkg_tar; for the Bazel-native
    build pipeline use `electron_vite_build` instead.

    Args:
        name: Base target name.
        main: Label producing the main-process bundle.
        renderer: Label producing the renderer-process bundle.
        napi_addon: Optional label for the N-API native addon.
        visibility: Standard visibility.
        **kwargs: Forwarded to underlying rules.
    """
    srcs = [main, renderer]
    if napi_addon:
        srcs.append(napi_addon)

    pkg_tar(
        name = "{}_app".format(name),
        srcs = srcs,
        extension = "tar",
        package_dir = "/out",
        visibility = visibility,
    )

    js_run_devserver(
        name = "{}_dev".format(name),
        args = [".", "--enable-logging"],
        chdir = native.package_name(),
        data = srcs,
        tool = "//:node_modules/electron",
        visibility = visibility,
        **kwargs
    )
