
load("@toolchains//cxx:defs.bzl", "export_cxx_toolchain")

cxx_warn_flags = [ ]

debug_opt_flags = [ "-Og" ]

release_opt_flags = [ "-O3" ]

cxx_link_flags = select({
    # on Linux, always use mold
    'config//cpu:arm64': [
        "--ld-path=$(location toolchains//cxx/linux/mold:mold-aarch64-linux)/bin/mold",
    ],
    'config//cpu:x86_64': [
        "--ld-path=$(location toolchains//cxx/linux/mold:mold-x86_64-linux)/bin/mold",
    ],
})

base_cc_flags = [
    "-isystem",
    "/usr/include/c++/11",
    "-isystem",
    "/usr/include",
    "-isystem",
    select({
        'config//cpu:arm64': "$(location toolchains//cxx/linux/clang:linux_arm64)/lib/clang/18/include",
        'config//cpu:x86_64': "$(location toolchains//cxx/linux/clang:linux_amd64)/lib/clang/18/include",
    }),
    "-D_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_FAST"
]

toolchain_alias(
    name = 'cxx',
    actual = select({
        "mode//:debug": ":cxx-debug",
        "mode//:release": ":cxx-release",
    }),
)

[
    cxx_toolchain(
        name = name,
        archiver = f':toolchain[archiver]',
        assembler = f':toolchain[assembler]',
        c_compiler = f':toolchain[c_compiler]',
        cxx_compiler = f':toolchain[cxx_compiler]',
        linker = f':toolchain[linker]',
        nm = f':toolchain[nm]',
        objcopy_for_shared_library_interface = f':toolchain[objcopy]',
        objdump = f':toolchain[objdump]',
        ranlib = f':toolchain[ranlib]',
        strip = f':toolchain[strip]',

        archiver_type = "gnu",
        linker_type = "gnu",
        compiler_type = "clang",
        shared_library_interface_type = "disabled",

        c_compiler_flags = vals.c_flags,
        cxx_compiler_flags = vals.cxx_flags,
        linker_flags = vals.link_flags,
    ) for (name, vals) in ({
        'cxx-debug': struct(
            c_flags = base_cc_flags + debug_opt_flags + cxx_warn_flags,
            cxx_flags = base_cc_flags + debug_opt_flags + cxx_warn_flags,
            link_flags = cxx_link_flags,
        ),
        'cxx-release': struct(
            c_flags = base_cc_flags + release_opt_flags + cxx_warn_flags,
            cxx_flags = base_cc_flags + release_opt_flags + cxx_warn_flags,
            link_flags = cxx_link_flags,
        ),
    }).items()
]

export_cxx_toolchain(
    name = f'toolchain',
    toolchain = select({
        'config//cpu:arm64': 'toolchains//cxx/linux/clang:linux_arm64',
        'config//cpu:x86_64': 'toolchains//cxx/linux/clang:linux_amd64',
    }),

    archiver = 'bin/llvm-ar',
    assembler = 'bin/clang',
    c_compiler = 'bin/clang',
    cxx_compiler = 'bin/clang++',
    linker = 'bin/clang',
    nm = 'bin/llvm-nm',
    objcopy = 'bin/llvm-objcopy',
    objdump = 'bin/llvm-objdump',
    ranlib = 'bin/llvm-ranlib',
    strip = 'bin/llvm-strip',
)
