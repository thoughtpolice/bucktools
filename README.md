## reapi-server

An experiment to build a `localhost`-only implementation of the
**[Remote Execution APIs][reapi]**, used by **[Bazel]**, **[Buck2]** and others.

[reapi]: https://github.com/bazelbuild/remote-apis
[Bazel]: https://bazel.build/
[Buck2]: https://buck2.build/

### Dotslash tools

You must have [DotSlash](https://dotslash-cli.com) installed, as this repository
provides various useful tools for you to use as DotSlash files.

```bash
export PATH=$PWD/buck/bin:$PATH
```

Windows users have to invoke `dotslash buck/bin/$FILE` manually, for now.

### Cargo build

Tools are provided by the Nix shell. Windows works too, but you need to install
`cargo` and `rustc` yourself.

```bash
nix develop .
cargo run &
```

### Buck2 build

Tools are not provisioned at all, currently; you must have `rustc` installed
somewhere and available in your `$PATH`. Buck2 works on all platforms.

```bash
buck2 run :reapi-server
```

### Testing API endpoints

`grpcurl` is provided by default under [./buck/bin/](./buck/bin/).

```bash
grpcurl -plaintext '[::1]:8080' list
grpcurl -d '{"instance_name":"test"}' -plaintext '[::1]:8080' build.bazel.remote.execution.v2.Capabilities/GetCapabilities
```

## License

MIT or Apache 2.0.
