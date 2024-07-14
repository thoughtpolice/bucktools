
alias(
    name = 'smoltar',
    actual = ':buck2-smoltar',
    visibility = [ 'PUBLIC' ],
)

rust_binary(
    name = 'buck2-smoltar',
    srcs = [ 'main.rs' ],
    deps = [
        # CARGO-SYNC-START: dependencies
        'third-party//rust:flate2',
        'third-party//rust:tar',
        # CARGO-SYNC-END
    ],
    visibility = [ 'PUBLIC' ],
)
