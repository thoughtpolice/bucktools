
DenoToolchain = provider(fields = {
    "deno": provider_field(typing.Any),
})

def _deno_toolchain_impl(ctx: AnalysisContext) -> list[Provider]:
    deno = cmd_args(ctx.attrs.deno)
    return [
        DefaultInfo(),
        DenoToolchain(deno = deno),
    ]

deno_toolchain = rule(
    impl = _deno_toolchain_impl,
    attrs = {
        "deno": attrs.list(attrs.arg()),
    },
    is_toolchain_rule = True,
)

def _deno_binary_impl(ctx: AnalysisContext) -> list[Provider]:
    deno = ctx.attrs._deno_toolchain[DenoToolchain].deno

    unstable_features = map(lambda x: f'--unstable-{x}', ctx.attrs.unstable_features)
    permissions = map(lambda x: f'--allow-{x}', ctx.attrs.permissions)

    output = ctx.actions.declare_output("{}.exe".format(ctx.label.name))
    ctx.actions.run(
        cmd_args([
            deno,
            "compile",
            "--output",
            output.as_output(),
        ] + unstable_features
          + permissions
        + [
            ctx.attrs.main,
        ]),
        category = "deno_compile",
    )

    return [
        DefaultInfo(
            default_output = output,
        ),
        RunInfo(
            args = cmd_args([
                deno,
                ctx.attrs.type,
                unstable_features,
                permissions,
                ctx.attrs.main,
            ])
        )
    ]

deno_binary = rule(
    impl = _deno_binary_impl,
    attrs = {
        "srcs": attrs.list(attrs.source(), default = []),
        "main": attrs.source(),
        "type": attrs.enum(["run", "serve"]),
        "unstable_features": attrs.list(attrs.string(), default = []),
        "permissions": attrs.list(attrs.string(), default = []),
        "_deno_toolchain": attrs.toolchain_dep(default = "toolchains//:deno", providers = [DenoToolchain]),
    }
)

def download_deno(version: str, hashes: list[(str, str)]):
    for triple, sha256 in hashes:
        url = f'https://github.com/denoland/deno/releases/download/v{version}/deno-{triple}.zip'
        native.http_archive(
            name = f'{version}-{triple}',
            sha256 = sha256,
            type = 'zip',
            urls = [ url ],
            visibility = [],
        )

    native.alias(
        name = f'{version}.zip',
        actual = select({
            'config//cpu:arm64': select({
                'config//os:linux': f':{version}-aarch64-unknown-linux-gnu',
                'config//os:macos': f':{version}-aarch64-apple-darwin',
            }),
            'config//cpu:x86_64': select({
                'config//os:linux': f':{version}-x86_64-unknown-linux-gnu',
                'config//os:windows': f':{version}-x86_64-pc-windows-msvc',
            }),
        }),
    )
