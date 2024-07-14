
def _run_test(ctx: AnalysisContext) -> list[Provider]:
    cmd = ctx.attrs.cmd[RunInfo].args
    cmd = cmd_args([cmd] + ctx.attrs.args)

    return [
        DefaultInfo(),
        ExternalRunnerTestInfo( type = "custom", command = [ cmd ] )
    ]

run_test = rule(
    impl = _run_test,
    attrs = {
        "cmd": attrs.dep(providers = [RunInfo]),
        "args": attrs.list(attrs.arg()),
    }
)
