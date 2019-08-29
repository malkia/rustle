load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

# Download bazel_skylib, unless present already.
maybe(
    http_archive,
    name = "bazel_skylib",
    sha256 = "1dde365491125a3db70731e25658dfdd3bc5dbdfd11b840b3e987ecf043c7ca0",
    url = "https://github.com/bazelbuild/bazel-skylib/releases/download/0.9.0/bazel_skylib-0.9.0.tar.gz",
)

# Download io_bazel_rules_rust, unless present already.
maybe(
    http_archive,
    name = "io_bazel_rules_rust",
    sha256 = "abd177ff2d066072e8929c8c2c9b22aeb59b4cabdcee34e8b4555f233729707f",
    strip_prefix = "rules_rust-minimum-bazel-22",
    url = "https://github.com/bazelbuild/rules_rust/archive/minimum-bazel-22.tar.gz",
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_skylib_workspace()
rust_repositories()
bazel_version(name = "bazel_version")
