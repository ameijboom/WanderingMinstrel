load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "minstrel",
    srcs = glob(["src/**/*.rs"]),
    crate_features = [],
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True),
)

pkg_tar(
    name = "base-layer",
    srcs = [":minstrel"],
)

oci_image(
    name = "api-image",
    base = "@distroless_cc",
    entrypoint = ["/minstrel"],
    tars = [
        ":base-layer",
    ],
)

oci_tarball(
    name = "base.tar",
    image = ":base-image",
    repo_tags = ["minstrel:latest"],
    visibility = ["//visibility:public"],
)
