
[
    http_archive(
        name = f'{name}',
        sha256 = sha256,
        type = 'tar.xz',
        urls = [
            f'https://github.com/dzbarsky/static-clang/releases/download/v18.1.2-1/{name}.tar.xz'
        ],
        visibility = [ 'toolchains//...' ],
    ) for name, sha256 in [
        ('darwin_arm64', '2ab62541139b9b3d1ec497bafbabf968303ad8d7610db83636652ddca46218ba'),
    ]
]
