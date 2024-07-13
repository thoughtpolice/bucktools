
[
    http_archive(
        name = name,
        sha256 = sha256,
        type = 'tar.bz2',
        urls = [
            'https://toolchains.bootlin.com/downloads/releases/toolchains/{}/tarballs/{}.tar.bz2'.format(
                name.split("--")[0],
                name
            )
        ],
        strip_prefix = name,
        visibility = [ 'toolchains//...' ],
    ) for name, sha256 in [
        ('aarch64--glibc--bleeding-edge-2024.02-1', '09a5a8a59403e3df7a557014d0f9463e8e366c5431656252f0ce3fc00ecfc050'),
        ('x86-64-v3--glibc--bleeding-edge-2024.02-1', '27bb18f261a412fa752f1c019ef4a9250653f16229f2ffb41eb584396787c68d'),
    ]
]
