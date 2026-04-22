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
        next_target = ":next",
        start_binary = ":next_start_binary",
        exposed_ports = ["3000/tcp"],
        repository = "registry.corp.agentsmesh.ai/web",
    )
"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("@aspect_rules_js//js:defs.bzl", "js_image_layer")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load", "oci_push")

def next_oci_image(
        name,
        start_binary,
        base = "@node_slim",
        env = {},
        exposed_ports = ["3000/tcp"],
        labels = {},
        repository = None,
        visibility = ["//visibility:public"]):
    """OCI image wrapping a Next.js production build.

    Args:
        name: Target name.
        start_binary: Label of the `<name>_start_binary` from next_app.
        base: Base image, defaults to Node 20-slim.
        env: Env vars injected into the container.
        exposed_ports: Container ports to expose.
        labels: OCI labels.
        repository: Remote registry (optional).
        visibility: Standard visibility.
    """

    # js_image_layer packages node_modules + the built .next/ tree
    # into OCI-compatible layers with correct ownership.
    js_image_layer(
        name = name + "_layer",
        binary = start_binary,
        root = "/app",
    )

    oci_image(
        name = name,
        base = base,
        entrypoint = ["/app/" + start_binary.split(":")[-1]],
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

    if repository:
        expand_template(
            name = name + "_tags",
            out = name + "_tags.txt",
            stamp_substitutions = {"0.0.0": "{{BUILD_EMBED_LABEL}}"},
            template = ["latest", "0.0.0"],
        )

        oci_push(
            name = name + "_push",
            image = ":" + name,
            remote_tags = ":" + name + "_tags",
            repository = repository,
            visibility = visibility,
        )
