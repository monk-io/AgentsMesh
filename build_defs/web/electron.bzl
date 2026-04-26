"""Electron desktop app packaging for AgentsMesh.

Wraps the electron-vite build pipeline (main + preload + renderer) and
electron-builder packaging into Bazel actions:

  - `electron_vite_build` runs `electron-vite build` over the package
    sources and produces the `out/` tree artifact (main + preload +
    renderer bundles) under `bazel-bin/`.
  - `electron_builder_app` consumes that tree plus the thin-shell
    `package.json` and an electron-builder config, runs
    `electron-builder` from a Bazel sandbox, and emits a `dist/` tree
    artifact containing platform installers (`.dmg`, `.exe`,
    `.AppImage`, etc.).

The legacy thin `electron_app` (just pkg_tar of pre-built artifacts)
is retained for callers that still ship a manually-built `out/`.

Usage:
    load("//build_defs/web:electron.bzl",
         "electron_vite_build", "electron_builder_app")

    electron_vite_build(
        name = "out",
        srcs = glob(["src/**/*"]) + ["electron.vite.config.ts", "tsconfig.json"],
    )

    electron_builder_app(
        name = "dist",
        out = ":out",
        package_json = "package.json",
        config = "electron-builder.yml",
    )
"""

load("@aspect_bazel_lib//lib:copy_to_directory.bzl", "copy_to_directory")
load("@aspect_rules_js//js:defs.bzl", "js_run_binary", "js_run_devserver")
load("@npm//:electron-builder/package_json.bzl", electron_builder_bin = "bin")
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

def electron_builder_app(
        name,
        out,
        package_json,
        config = "electron-builder.yml",
        extra_srcs = None,
        out_dir = "dist",
        platform = None,
        chdir = None,
        visibility = None,
        tags = None,
        **kwargs):
    """Run `electron-builder` over a pre-built `out/` tree as a Bazel action.

    Wraps the electron-builder CLI: stages the `out/` tree (produced by
    `electron_vite_build`) plus the thin-shell `package.json` and an
    electron-builder config file into a sandbox CWD, then runs
    `electron-builder` to produce platform-specific installers
    (`.dmg`, `.exe`, `.AppImage`, etc.) into a `dist_dir` tree artifact.

    Args:
        name: Target name. Output tree label is `:<name>` (the `dist/`
            tree).
        out: Label of the `electron_vite_build` target whose tree
            artifact contains `main/` + `preload/` + `renderer/`.
        package_json: Label of the thin-shell `package.json` (carries
            `name`/`version`/`main`/`build`).
        config: electron-builder config filename. Default
            `electron-builder.yml`.
        extra_srcs: Additional sources to ship into the staging
            sandbox (build/, icons, entitlements, etc.).
        out_dir: Output directory name (default `dist`). Tree
            artifact created relative to bazel-bin/<package>/.
        platform: Platform flag — one of `mac`, `win`, `linux`. If
            unset, electron-builder builds for the host platform.
        chdir: CWD electron-builder runs in. Defaults to the calling
            package.
        visibility: Standard visibility.
        tags: Bazel tags. Defaults to `["manual"]` because cross-platform
            packaging requires the matching host (macOS for `.dmg`, etc.)
            and pulls in heavy native deps; cluster CI opts in via
            `--build_tag_filters=electron_builder`.
        **kwargs: Forwarded to `js_run_binary`.
    """
    electron_builder_bin.electron_builder_binary(
        name = name + "_bin",
        # Critical: disable aspect_rules_js's `node-patches/fs.cjs`
        # interception. electron-builder's `appFileCopier.walk` uses
        # `lstat`+`readlink` recursively over the staging tree; the
        # patches' multi-hop readlink walker throws EINVAL on regular
        # files inside bazel-out (the `package.json` symlink chain is
        # deeper than node-patches expects). Unpatched fs lets
        # electron-builder walk normally.
        patch_node_fs = False,
        visibility = ["//visibility:private"],
    )

    # Stage `:out` + thin-shell package.json + config into a hermetic
    # directory of real files (not aspect_rules_js symlink chains).
    # electron-builder's `appFileCopier.walk` uses `lstat` + `readlink`
    # recursively; the multi-hop pnpm/aspect symlinks tree under
    # bazel-out trips its symlink walker (EINVAL on readlink). The
    # staging copy produces a `.app/` layout electron-builder can walk
    # without hitting node-patches/fs.cjs.
    staging_name = name + "_staging"
    copy_to_directory(
        name = staging_name,
        srcs = [out, package_json, config] + (extra_srcs or []),
        # `out` carries `out/` prefix from electron_vite_build's
        # tree-artifact root; package.json + config land at top-level.
        root_paths = [native.package_name()],
        # Mirror real file content so electron-builder's readlink
        # walker stops following hop chains.
        replace_prefixes = {},
        hardlink = "off",
        allow_overwrites = True,
        verbose = False,
    )

    args = [
        "--config",
        config,
        "--projectDir",
        staging_name,
    ]
    if platform == "mac":
        args.append("--mac")
    elif platform == "win":
        args.append("--win")
    elif platform == "linux":
        args.append("--linux")

    srcs = [
        ":" + staging_name,
        "//:node_modules",
    ]

    # Default tags include `no-sandbox`/`local` because macOS dmg creation
    # invokes `hdiutil`, which needs real filesystem operations and a
    # `/private` mount the darwin-sandbox forbids. Linux AppImage
    # creation also calls native binaries with similar constraints.
    # Bazel still tracks inputs/outputs hermetically — sandbox-off only
    # changes the *execution* sandbox, not the action graph.
    default_tags = ["manual", "electron_builder", "no-sandbox", "local"]

    js_run_binary(
        name = name,
        srcs = srcs,
        args = args,
        chdir = chdir or native.package_name(),
        out_dirs = [out_dir],
        tool = ":" + name + "_bin",
        tags = tags if tags != None else default_tags,
        visibility = visibility,
        # Disable aspect_rules_js's `node-patches/fs.cjs` interception
        # — see binary above for the rationale (electron-builder's
        # recursive lstat/readlink walk hits EINVAL on bazel-out
        # symlink chains the patches were never designed to follow).
        patch_node_fs = False,
        # electron-builder writes tons of cache state (electron binary
        # download, app-builder downloads, etc.) under $HOME — without
        # an isolated CACHE dir, the sandbox bombs out trying to create
        # `/.cache`. Point its caches at $TMPDIR.
        env = {
            "ELECTRON_BUILDER_CACHE": "$$(pwd)/.electron-builder-cache",
            "ELECTRON_CACHE": "$$(pwd)/.electron-cache",
        },
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
