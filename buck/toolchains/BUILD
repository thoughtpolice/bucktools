load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")
load("@prelude//toolchains:cxx.bzl", "system_cxx_toolchain")
load("@prelude//toolchains:python.bzl", "system_python_bootstrap_toolchain")

system_genrule_toolchain(name = "genrule")
system_python_bootstrap_toolchain(name = "python_bootstrap")

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
    actual = select({
        "mode//:debug": ":cxx-debug",
        "mode//:release": ":cxx-release",
    }),
)

cxx_warn_flags = select({
    "config//os:macos": [
        # NOTE (aseipp): some apparently ancient zlib code in the zlib-sys crate
        # needs this, otherwise builds fail on sufficiently recent macOS systems due
        # to C2X support officially making this an error by default
        "-Wno-implicit-function-declaration",
    ],
    "DEFAULT": [],
})

cxx_link_flags = select({
    "config//os:linux": [
        # on Linux, always use mold
        "-fuse-ld=mold"
    ],

    "config//os:macos": [
        # on macOS, use the new parallel linker
        "-fuse-ld=/usr/bin/ld",
        "-ld_new",
        "-framework",
        "Security",
    ],

    # on Windows, suffer
    "config//os:windows": [
        "winhttp.lib",
        "rpcrt4.lib",
        "ole32.lib",
        "crypt32.lib",
        "secur32.lib",
        "user32.lib",
    ],
})

debug_opt_flags = select({
    "config//os:windows": ["/Od"],
    "DEFAULT": ["-Og"],
})

release_opt_flags = select({
    "config//os:windows": ["/O2"],
    "DEFAULT": ["-O3"],
})

cxx_linker = select({
    "config//os:windows": "link.exe",
    "DEFAULT": "clang++",
})

system_cxx_toolchain(
    name = "cxx-debug",
    c_flags = debug_opt_flags + cxx_warn_flags,
    cxx_flags = debug_opt_flags + cxx_warn_flags,
    linker = cxx_linker,
    link_flags = cxx_link_flags,
)

system_cxx_toolchain(
    name = "cxx-release",
    c_flags = release_opt_flags + cxx_warn_flags,
    cxx_flags = release_opt_flags + cxx_warn_flags,
    linker = cxx_linker,
    link_flags = cxx_link_flags,
)
