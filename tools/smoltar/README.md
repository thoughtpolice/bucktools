# smoltar: a small tar extractor

> [!NOTE]
>
> Originally source location: <https://github.com/thoughtpolice/smoltar>

In order to bootstrap hermetic toolchains in [Buck2], a standalone Python
toolchain is needed. [python-build-standalone] is perfect for this, except for a
minor detail:

It's actually kind of annoying to extract a tar file. Well, it's annoying to do
it in a way that:

1. Works on all operating systems (including Windows 10/11)
2. Creates the output directory for you (a convenience)
3. Without writing out temporary shell scripts to do it for you (also a
   convenience)

The first one is the biggest problem (apparently `tar.exe` apparently appears to
exist on some versions of Windows and not others), but even macOS userspace and
GNU coreutils `tar` have differences too.

Ultimately all we need in Buck is to extract these Python tarballs to help get
things started. This project exists only to solve this one problem: portable
static binaries that work on all operating systems, can extract tar files to a
directory, and can create that directory beforehand.

[Buck2]: https://buck2.build
[python-build-standalone]: https://github.com/indygreg/python-build-standalone
