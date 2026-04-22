"""Electron desktop app packaging for AgentsMesh.

Bundles the renderer build (produced by electron-vite, Next.js, or a raw
Vite target), the main-process entry script, and the N-API native addon
(built via build_defs/rust/napi.bzl) into an Electron-loadable layout.

This macro is a thin wrapper: it does not replace `electron-builder` or
`electron-forge` for producing signed `.dmg`/`.exe` installers. That step
runs outside Bazel (typically in a release workflow) over the output of
`:<name>_app`. The macro's job is hermetically producing the app's
on-disk layout so the distribution step is deterministic.

Outputs:
  :<name>_app       A pkg_tar bundle ready to hand off to electron-builder.
  :<name>_dev       A js_run_devserver running Electron against the local tree.

Usage:
    load("//build_defs/web:electron.bzl", "electron_app")

    electron_app(
        name = "desktop",
        main = ":main_bundle",           # electron-vite / ts_project
        renderer = ":renderer_bundle",   # typically a Next.js export
        napi_addon = "//clients/core/crates/node-bridge:node_bridge",
        electron_version = "37.0.0",
    )
"""

load("@aspect_rules_js//js:defs.bzl", "js_run_devserver")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

def electron_app(
        name,
        main,
        renderer,
        napi_addon = None,
        visibility = ["//visibility:public"],
        **kwargs):
    """Assemble an Electron app on-disk layout.

    The produced `:<name>_app` tarball is structured as:
        /out/
            main/
                index.js            (from `main`)
            renderer/
                ... index.html + assets ... (from `renderer`)
            native/
                <addon>.node        (optional, from `napi_addon`)

    Args:
        name: Base target name.
        main: Label producing the main-process bundle.
        renderer: Label producing the renderer-process bundle (directory).
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

    # Minimal dev runner — assumes electron-vite is the underlying builder.
    # For more complex workflows, teams can wire a custom js_run_devserver
    # call in their target's BUILD.bazel.
    js_run_devserver(
        name = "{}_dev".format(name),
        args = [".", "--enable-logging"],
        chdir = native.package_name(),
        data = srcs,
        tool = "//:node_modules/electron",
        visibility = visibility,
        **kwargs
    )
