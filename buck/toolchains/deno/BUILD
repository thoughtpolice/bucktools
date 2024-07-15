
load(":defs.bzl", "download_deno", "deno_toolchain")

DENO_VERSION = '1.45.2'

toolchain_alias(
    name = 'deno',
    actual = f':deno-{DENO_VERSION}',
)

deno_toolchain(
    name = 'deno-1.45.2',
    deno = select({
        'config//cpu:arm64': select({
            # use $(location ...) macro to get the path to the downloaded Deno binary
            'config//os:linux': [ '$(location :1.45.2-aarch64-unknown-linux-gnu)/deno' ],
            'config//os:macos': [ '$(location :1.45.2-aarch64-apple-darwin)/deno' ],
        }),
        'config//cpu:x86_64': select({
            'config//os:linux':   [ '$(location :1.45.2-x86_64-unknown-linux-gnu)/deno' ],
            'config//os:windows': [ '$(location :1.45.2-x86_64-pc-windows-msvc)/deno.exe' ],
        }),
    }),
)

download_deno(
    version = '1.45.2',
    hashes = [
        ('aarch64-unknown-linux-gnu', '180afe7c70121e2110dcf8d64bb1cb844b1202b505fcc5cd911e0bf2c93bce8a'),
        ('aarch64-apple-darwin', 'de9627d06ae4d6b9960430ad294789cfb55f327b31e6d032c0c3b1ad7b7e45c1'),
        ('x86_64-unknown-linux-gnu', 'bd92822f0b12fc0e9e050c494c43226d40b1acda6c8acc86c85843ed37f7de95'),
        ('x86_64-pc-windows-msvc', '3b94de2b9b63b9859affb290fe976f6f09338964d0b8cce3c1c42a35efe0575c'),
    ]
)
