
[
    http_archive(
        name = f'mold-{name}',
        sha256 = sha256,
        type = 'tar.gz',
        strip_prefix = f'mold-2.32.1-{name}',
        urls = [
            f'https://github.com/rui314/mold/releases/download/v2.32.1/mold-2.32.1-{name}.tar.gz',
        ]
    ) for name, sha256 in [
        ('aarch64-linux', '18ffc28b392fef90f3a73b4c40445925a661298f255c3031753c351ae7c7600a'),
        ('x86_64-linux', 'd9caf252d58b5f6a0fa5aa30fbe05d17b1c84f498106dc634cbce73866abdfeb'),
    ]
]
