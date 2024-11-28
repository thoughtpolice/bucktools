
toolchain_alias(
    name = 'cxx',
    actual = select({
        "mode//:debug": ":cxx-debug",
        "mode//:release": ":cxx-release",
    }),
)

toolchain_alias(
    name = 'cxx-debug',
    actual = select({
        'config//os:linux': 'toolchains//cxx/linux:cxx-debug',
        'config//os:macos': 'toolchains//cxx/macos:cxx-debug',
        'config//os:windows': 'toolchains//cxx/win:cxx-debug',
    })
)

toolchain_alias(
    name = 'cxx-release',
    actual = select({
        'config//os:linux': 'toolchains//cxx/linux:cxx-release',
        'config//os:macos': 'toolchains//cxx/macos:cxx-release',
        'config//os:windows': 'toolchains//cxx/win:cxx-release',
    })
)
