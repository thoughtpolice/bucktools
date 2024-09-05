
load("@toolchains//deno:defs.bzl", "deno_binary")

deno_binary(
    name = 'upload-logs',
    main = 'upload.ts',
    type = 'run',
    permissions = [ 'read', 'net' ],
    unstable_features = [ 'kv' ],
    visibility = [ 'PUBLIC' ],
)

deno_binary(
    name = 'log-server',
    main = 'server.ts',
    type = 'serve',
    permissions = [ 'net' ],
    unstable_features = [ 'kv' ],
    visibility = [ 'PUBLIC' ],
)
