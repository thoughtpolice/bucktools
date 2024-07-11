
load(":defs.bzl", "vendored_rust_toolchain", "download_rust_toolchain")

toolchain_alias(
    name = 'rust',
    actual = select({
        "mode//:debug": ":rust-debug",
        "mode//:release": ":rust-release",
    }),
)

DEFAULT_RUST_TOOLCHAIN = 'rustc-nightly-2024-07-11'

common_rustc_flags = select({
    'config//os:windows': [
        # NOTE: on Windows, link the CRT statically so that users don't need
        # a copy of vcruntime140.dll on their machines. its distribution is...
        # complex
        "-Ctarget-feature=+crt-static",
    ],
    'DEFAULT': [],
})

vendored_rust_toolchain(
    name = "rust-debug",
    toolchain = f':{DEFAULT_RUST_TOOLCHAIN}',
    default_edition = "2021",
    rustc_flags = common_rustc_flags + [
        "-Zthreads=0",
        # XXX TODO FIXME: apply this only to tokio/console_subscriber instead of
        # blanket applying it in the future?
        "--cfg=tokio_unstable",
    ],
)

vendored_rust_toolchain(
    name = "rust-release",
    toolchain = f':{DEFAULT_RUST_TOOLCHAIN}',
    default_edition = "2021",
    rustc_flags = common_rustc_flags + [
        "-Copt-level=3",
        "-Cdebuginfo=none",
        "-Cdebug-assertions=false",
        "-Coverflow-checks=false",
        "-Clto=false",
        "-Ccodegen-units=1",
    ],
)

download_rust_toolchain(
    name = 'nightly-2024-07-11',
    channel = 'nightly',
    version = '2024-07-11',
    hashes = [
        ('aarch64-unknown-linux-gnu', 'f36ab3c836dad4b4c40212905314f3a4138e887d18eb2f61c507fd27d4ee06bd'),
        ('aarch64-apple-darwin', 'e987370555e8b6b492e475aa8c6f3127f6669d4013b85e1fdd7c676bc8f710b9'),
        ('x86_64-unknown-linux-gnu', '6d58267bc70d1afa122e5f47549d61ed17e472754be8993c6614bde3edd53b2d'),
        ('x86_64-pc-windows-msvc', 'f44dd18ecd55948c0ac636a1069e64e1787012feaa8dedf4f3daf000ffc1444b'),
    ]
)

download_rust_toolchain(
    name = 'stable-1.79.0',
    channel = 'stable',
    version = '1.79.0',
    hashes = [
        ('aarch64-unknown-linux-gnu', 'f7d3b31581331b54af97cf3162e65b8c26c8aa14d42f71c1ce9adc1078ef54e5'),
        ('aarch64-apple-darwin', 'e70a9362975b94df7dbc6e2ed5ceab4254dd32f72ba497ff4a70440ace3f729f'),
        ('x86_64-unknown-linux-gnu', '628efa8ef0658a7c4199883ee132281f19931448d3cfee4ecfd768898fe74c18'),
        ('x86_64-pc-windows-msvc', '8ee0346884d6da5ad250ee389236a446972c84b1bb5ac7e34be74c4a4e05f101'),
    ]
)
