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

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("@aspect_rules_js//js:defs.bzl", "js_image_layer")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load", "oci_push")

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

    # Normalize single-repo back-compat form.
    repos = dict(repositories)
    if repository and not repos:
        repos["default"] = repository

    if repos:
        # Tag list: always `latest`, plus the stamped version + minor.
        # Duplicate values are harmless — oci_push de-dups tags and the
        # registry just re-tags the manifest.
        expand_template(
            name = name + "_tags",
            out = name + "_tags.txt",
            template = [
                "latest",
                "STAMP_VERSION",
                "STAMP_MINOR",
            ],
            stamp_substitutions = {
                "STAMP_VERSION": "{{STABLE_IMAGE_VERSION}}",
                "STAMP_MINOR": "{{STABLE_IMAGE_MINOR}}",
            },
        )

        for reg_key in sorted(repos.keys()):
            oci_push(
                name = "{}_push_{}".format(name, reg_key),
                image = ":" + name,
                remote_tags = ":" + name + "_tags",
                repository = repos[reg_key],
                visibility = visibility,
            )

        # Back-compat alias — first registry's push under the legacy
        # `:name_push` name. Pick `default` if present, else the first
        # alphabetical key.
        back_compat_key = "default" if "default" in repos else sorted(repos.keys())[0]
        native.alias(
            name = name + "_push",
            actual = ":{}_push_{}".format(name, back_compat_key),
            visibility = visibility,
        )
