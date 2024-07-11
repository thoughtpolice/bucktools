load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")

toolchain_alias(
    name = 'genrule',
    actual = 'toolchains//genrule:genrule',
)

toolchain_alias(
    name = 'python_bootstrap',
    actual = 'toolchains//python-bootstrap:python_bootstrap',
)

toolchain_alias(
    name = 'rust',
    actual = select({
        "mode//:debug": ":rust-debug",
        "mode//:release": ":rust-release",
    }),
)

system_rust_toolchain(
    name = "rust-debug",
    default_edition = "2021",
    rustc_flags = [
        "-Zthreads=0",
        # XXX TODO FIXME: apply this only to tokio/console_subscriber instead of
        # blanket applying it in the future?
        "--cfg=tokio_unstable",
    ],
)

system_rust_toolchain(
    name = "rust-release",
    default_edition = "2021",
    rustc_flags = [
        "-Copt-level=3",
        "-Cdebuginfo=none",
        "-Cdebug-assertions=false",
        "-Coverflow-checks=false",
        "-Clto=false",
        "-Ccodegen-units=1",
    ],
)

toolchain_alias(
    name = 'cxx',
    actual = 'toolchains//cxx:cxx',
)
