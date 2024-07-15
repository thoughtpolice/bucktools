
TOOLS = {
    'buck2-cache-server': '//tools/cache-server:cache-server',
    'buck2-smoltar':      '//tools/smoltar:smoltar',
}

filegroup(
    name = 'bucktools',
    srcs = {
        f'{name}.exe': tgt for name, tgt in TOOLS.items()
    },
)
