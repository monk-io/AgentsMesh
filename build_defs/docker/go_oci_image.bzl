"""OCI image macro for AgentsMesh Go services.

Adapted from `~/Works/AIO/AIO/DevOps/BuildSystem/Server/oci.bzl`. Produces
one oci_image + local `_tarball` (for `docker load`), and a `_push` target
for any service that specifies `repository`.

Usage:
    load("//build_defs/docker:go_oci_image.bzl", "go_oci_image")

    go_oci_image(
        name = "image",
        binary = ":server",
        exposed_ports = ["8080/tcp"],
        repository = "registry.corp.agentsmesh.ai/backend",
    )

Replaces:
    ci/backend.Dockerfile, ci/runner.Dockerfile, ci/relay.Dockerfile
"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load", "oci_push")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

def go_oci_image(
        name,
        binary,
        base = "@distroless_static",
        env = {},
        exposed_ports = [],
        labels = {},
        repository = None,
        visibility = ["//visibility:public"]):
    """Bundle a Go binary into an OCI image.

    Outputs:
        :<name>              oci_image (base = distroless/static)
        :<name>_tarball      oci_load target (`bazel run` → docker load)
        :<name>_push         oci_push (only if `repository` is set)

    The binary lives at `/app/<binary_name>` inside the image and runs as
    the non-root user baked into distroless.

    Args:
        name: Target name.
        binary: Label of the `go_binary` to embed.
        base: OCI base image, defaults to distroless/static.
        env: Environment variables set inside the container.
        exposed_ports: Ports to expose on the image.
        labels: OCI labels to add to the image.
        repository: Remote registry repo URL (optional); enables `_push`.
        visibility: Standard visibility.
    """
    binary_name = binary.split(":")[-1]

    pkg_tar(
        name = name + "_layer",
        srcs = [binary],
        package_dir = "/app",
    )

    oci_image(
        name = name,
        base = base,
        entrypoint = ["/app/" + binary_name],
        env = env,
        exposed_ports = exposed_ports,
        labels = labels,
        tars = [":" + name + "_layer"],
        user = "nonroot",
        visibility = visibility,
        workdir = "/app",
    )

    oci_load(
        name = name + "_tarball",
        image = ":" + name,
        repo_tags = ["agentsmesh/" + binary_name + ":latest"],
        visibility = visibility,
    )

    if repository:
        # Tags file expanded at build time. BUILD_EMBED_LABEL comes from
        # `--embed_label=<sha>` in CI; falls back to "0.0.0" locally.
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
