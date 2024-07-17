
VERSION = '18.1.2-1'

[
    http_archive(
        name = f'{name}',
        sha256 = sha256,
        type = 'tar.xz',
        urls = [
            f'https://github.com/dzbarsky/static-clang/releases/download/v{VERSION}/{name}.tar.xz'
        ],
        visibility = [ 'toolchains//...' ],
    ) for name, sha256 in [
        ('linux_amd64', 'a0f47523f2e6a7afd268d8172df563856c6c6a027fb7be5cb04bbcfe8a86b932'),
        ('linux_arm64', '748167cca66051dbfb505c4717e67b17a13d8b006c8bcee1df644e41ffa53c55'),
    ]
]
