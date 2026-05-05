"""Package a Next.js build into an OCI image.

Wraps the `next_app()` output (the `.next/` directory + `package.json`
+ `node_modules`) into a Node-based container. The resulting image runs
`node ./node_modules/next/dist/bin/next start` on container boot.

Separate from build_defs/docker/go_oci_image.bzl because Node apps carry
their interpreter + runtime in-image, whereas Go binaries are static.

Usage:
    load("//build_defs/web:next_oci.bzl", "next_oci_image")

    next_oci_image(
        name = "image",
        start_binary = ":next_start_binary",
        exposed_ports = ["3000/tcp"],
        repositories = {
            "dockerhub":  "agentsmesh/web",
            "agentsmesh": "registry.agentsmesh.ai/agentsmesh/web",
        },
    )

    # Produces:
    #   :image                   — oci_image
    #   :image_tarball           — oci_load (docker load)
    #   :image_push_dockerhub    — oci_push to Docker Hub
    #   :image_push_agentsmesh   — oci_push to AgentsMesh registry
"""

load("@aspect_rules_js//js:defs.bzl", "js_image_layer")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load")
load("//build_defs/docker:oci_push.bzl", "oci_push_targets")

def next_oci_image(
        name,
        start_binary,
        base = "@distroless_nodejs",
        env = {},
        exposed_ports = ["3000/tcp"],
        labels = {},
        repositories = {},
        repository = None,
        visibility = ["//visibility:public"]):
    """OCI image wrapping a Next.js production build.

    Args:
        name: Target name.
        start_binary: Label of the `<name>_start_binary` from next_app.
        base: Base image, defaults to distroless Node 20.
        env: Env vars injected into the container.
        exposed_ports: Container ports to expose.
        labels: OCI labels.
        repositories: Dict mapping registry key → full repo URL. Every
            entry becomes a `:name_push_<key>` target. `bazel run` one
            of them with `--stamp --workspace_status_command=...` to
            push with the CI-provided version tag.
        repository: Back-compat single-registry form; equivalent to
            `repositories = {"default": repository}`. Prefer the dict.
        visibility: Standard visibility.
    """

    # js_image_layer packages node_modules + the built .next/ tree
    # into OCI-compatible layers with correct ownership. The layer
    # places the binary at `<root>/<package>/<name>` — mirror that here
    # when wiring the container entrypoint. The workdir stays at `/app`
    # (not the package subdir) because the binary's internal `chdir`
    # env is resolved relative to `$PWD`, so running from /app lets the
    # script cd into clients/<pkg>/ as the build-time `chdir` attr
    # expects.
    js_image_layer(
        name = name + "_layer",
        binary = start_binary,
        root = "/app",
    )

    binary_name = start_binary.split(":")[-1]
    entrypoint_path = "/app/" + native.package_name() + "/" + binary_name

    oci_image(
        name = name,
        base = base,
        entrypoint = [entrypoint_path],
        env = env,
        exposed_ports = exposed_ports,
        labels = labels,
        tars = [":" + name + "_layer"],
        visibility = visibility,
        workdir = "/app",
    )

    oci_load(
        name = name + "_tarball",
        image = ":" + name,
        repo_tags = ["agentsmesh/" + name + ":latest"],
        visibility = visibility,
    )

    oci_push_targets(
        name = name,
        image = ":" + name,
        repositories = repositories,
        repository = repository,
        visibility = visibility,
    )
