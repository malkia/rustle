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
    sha256 = "55968c5377d9d9f4a5c61780c8a041d478eaac26d984d19fd589afaf12b353dc",
    strip_prefix = "rules_rust-05bd7d1d1bd34225a6614fc131267181aee2b61e",
    url = "https://github.com/bazelbuild/rules_rust/archive/05bd7d1d1bd34225a6614fc131267181aee2b61e.tar.gz",
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories", "rust_repository_set")
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_skylib_workspace()

rust_repositories()

# This allows us to build (succesfully) on Windows, yet bazel throws that "not all outputs were created or valid"
# Most likely due to it expecting the file to be created without extension (.exe), while it does put it.
rust_repository_set(
    name = "rust_windows_x86_64",
    exec_triple = "x86_64-pc-windows-msvc",
    extra_target_triples = [], #["x86_64-unknown-linux-gnu"],
    version = "nightly",
    iso_date = "2019-08-22",
)

# This allows us to build (succesfully) on Windows, yet bazel throws that "not all outputs were created or valid"
# Most likely due to it expecting the file to be created without extension (.exe), while it does put it.
rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = [], #["x86_64-unknown-linux-gnu"],
    version = "nightly",
    iso_date = "2019-09-01",
)

bazel_version(name = "bazel_version")
