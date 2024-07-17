
load("@toolchains//cxx:defs.bzl", "export_cxx_toolchain", "runnable")

cxx_warn_flags = [
    # NOTE (aseipp): some apparently ancient zlib code in the zlib-sys crate
    # needs this, otherwise builds fail on sufficiently recent macOS systems due
    # to C2X support officially making this an error by default
    "-Wno-implicit-function-declaration",
]

debug_opt_flags = [ "-Og" ]

release_opt_flags = [ "-O3" ]

cxx_link_flags = [
    "-framework",
    "Security",
]

base_cc_flags = [
    "-isystem",
    select({
        'config//cpu:arm64': "$(location toolchains//cxx/macos/clang:darwin_arm64)/include/c++/v1",
    }),
    "-isystem",
    "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include",
    "-isystem",
    select({
        'config//cpu:arm64': "$(location toolchains//cxx/macos/clang:darwin_arm64)/lib/clang/18/include/",
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

runnable(
    name = 'linker',
    cmd = [
        "/usr/bin/clang",
        "--ld-path=/usr/bin/ld",
        "-Wl,-ld_new",
    ],
)

[
    cxx_toolchain(
        name = name,
        archiver = f':toolchain[archiver]',
        assembler = f':toolchain[assembler]',
        c_compiler = f':toolchain[c_compiler]',
        cxx_compiler = f':toolchain[cxx_compiler]',
        linker = ':linker',
        nm = f':toolchain[nm]',
        objcopy_for_shared_library_interface = f':toolchain[objcopy]',
        objdump = f':toolchain[objdump]',
        ranlib = f':toolchain[ranlib]',
        strip = f':toolchain[strip]',

        archiver_type = "gnu",
        linker_type = "darwin",
        compiler_type = "clang",
        shared_library_interface_type = "disabled",
        pic_behavior = "always_enabled",

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
        'config//cpu:arm64': 'toolchains//cxx/macos/clang:darwin_arm64',
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
