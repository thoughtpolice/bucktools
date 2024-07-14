# bucktools

A set of tools for working with [Buck2].

The tools can be found under the [`./tools/`](./tools/) directory. More
information will come in the future.

This also serves as my playground for integrating Buck2 with GitHub and CI
systems, etc. Hopefully, it can serve as a useful template for you if you want
to use Buck2, Rust, GitHub Actions, and other tools together.

> [!IMPORTANT]
>
> These tools are designed to **ONLY** work with [Buck2] and are not designed
> for the original Buck1!

[Buck2]: https://buck2.build

## Buck2 build

These tools are also built with Buck2, of course. It works on macOS, Windows,
and Linux.

Rust toolchains are provisioned automatically through `BUILD` files. You must
have a C/C++ compiler available, though that will hopefully change in the
future.

You must have [DotSlash] installed, as this repository provides various useful
tools for you to use including `buck2` itself.

Take a look in the [`./buck/bin`](./buck/bin) directory for the tools in
question.

```bash
export PATH=$PWD/buck/bin:$PATH
```

```bash
buck2 run //tools/cache-server --
```

Windows users have to invoke `dotslash buck/bin/$FILE` manually, for now, though
hopefully that will change in the future.

[DotSlash]: https://dotslash-cli.com

## Cargo build

> [!WARNING]
>
> The goal is to eventually deprecate the Cargo build, so be warned.

Tools are provided by the Nix shell. Windows works too, but you need to install
`cargo` and `rustc` yourself.

```bash
nix develop .
cargo build
```

### Testing API endpoints

`grpcurl` is provided by default under [./buck/bin/](./buck/bin/).

```bash
grpcurl -plaintext '[::1]:8080' list
grpcurl -d '{"instance_name":"test"}' -plaintext '[::1]:8080' build.bazel.remote.execution.v2.Capabilities/GetCapabilities
```

## License

MIT or Apache 2.0.
