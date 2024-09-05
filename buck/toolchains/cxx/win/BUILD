
load("@prelude//toolchains:cxx.bzl", "system_cxx_toolchain")

cxx_warn_flags = [ ]

debug_opt_flags = [ "/Od" ]

release_opt_flags = [ "/O2" ]

cxx_link_flags = [
    "winhttp.lib",
    "rpcrt4.lib",
    "ole32.lib",
    "crypt32.lib",
    "secur32.lib",
    "user32.lib",
]

cxx_linker = "link.exe"

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
