# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

load(
    "@prelude//cxx:cxx_toolchain_types.bzl",
    "BinaryUtilitiesInfo",
    "CCompilerInfo",
    "CvtresCompilerInfo",
    "CxxCompilerInfo",
    "CxxPlatformInfo",
    "CxxToolchainInfo",
    "LinkerInfo",
    "PicBehavior",
    "RcCompilerInfo",
    "ShlibInterfacesMode",
)
load("@prelude//cxx:headers.bzl", "HeaderMode")
load("@prelude//cxx:linker.bzl", "is_pdb_generated")
load("@prelude//linking:link_info.bzl", "LinkOrdering", "LinkStyle")
load("@prelude//linking:lto.bzl", "LtoMode")
load("@prelude//toolchains/msvc:tools.bzl", "VisualStudio")
load("@prelude//utils:cmd_script.bzl", "ScriptOs", "cmd_script")

def _export_cxx_toolchain_impl(ctx: AnalysisContext) -> list[Provider]:
    toolchain = ctx.attrs.toolchain[DefaultInfo].default_outputs[0]

    archiver = toolchain.project(ctx.attrs.archiver)
    assembler = toolchain.project(ctx.attrs.assembler)
    c_compiler = toolchain.project(ctx.attrs.c_compiler)
    cxx_compiler = toolchain.project(ctx.attrs.cxx_compiler)
    linker = toolchain.project(ctx.attrs.linker)
    nm = toolchain.project(ctx.attrs.nm)
    objcopy = toolchain.project(ctx.attrs.objcopy)
    objdump = toolchain.project(ctx.attrs.objdump)
    ranlib = toolchain.project(ctx.attrs.ranlib)
    strip = toolchain.project(ctx.attrs.strip)

    return [
        DefaultInfo(
            sub_targets = {
                'archiver': [ DefaultInfo(), RunInfo(args = cmd_args(archiver)) ],
                'assembler': [ DefaultInfo(), RunInfo(args = cmd_args(assembler)) ],
                'c_compiler': [ DefaultInfo(), RunInfo(args = cmd_args(c_compiler)) ],
                'cxx_compiler': [ DefaultInfo(), RunInfo(args = cmd_args(cxx_compiler)) ],
                'linker': [ DefaultInfo(), RunInfo(args = cmd_args(linker)) ],
                'nm': [ DefaultInfo(), RunInfo(args = cmd_args(nm)) ],
                'objcopy': [ DefaultInfo(), RunInfo(args = cmd_args(objcopy)) ],
                'objdump': [ DefaultInfo(), RunInfo(args = cmd_args(objdump)) ],
                'ranlib': [ DefaultInfo(), RunInfo(args = cmd_args(ranlib)) ],
                'strip': [ DefaultInfo(), RunInfo(args = cmd_args(strip)) ],
            }
        )
    ]

export_cxx_toolchain = rule(
    impl = _export_cxx_toolchain_impl,
    attrs = {
        "toolchain": attrs.dep(),

        "archiver": attrs.string(),
        "assembler": attrs.string(),
        "c_compiler": attrs.string(),
        "cxx_compiler": attrs.string(),
        "linker": attrs.string(),
        "nm": attrs.string(),
        "objcopy": attrs.string(),
        "objdump": attrs.string(),
        "ranlib": attrs.string(),
        "strip": attrs.string(),
    },
)

runnable = rule(
    impl = lambda ctx: [ DefaultInfo(), RunInfo(args = cmd_args(ctx.attrs.cmd)) ],
    attrs = {
        "cmd": attrs.list(attrs.arg()),
    }
)
