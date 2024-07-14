
load(":defs.bzl", "run_test")

python_bootstrap_binary(
    name = 'check-deps',
    main = 'check-deps.py',
    visibility = [ ],
)

run_test(
    name = "test-deps-unused",
    cmd = ":check-deps",
    args = [ "check-unused" ],
)

run_test(
    name = "test-update-buck-deps",
    cmd = ":check-deps",
    args = [ "update-buck-deps" ],
)

run_test(
    name = "test-sync-cargo",
    cmd = ":check-deps",
    args = [ "sync-cargo" ],
)
