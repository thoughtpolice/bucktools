## reapi-server

An experiment to build a `localhost`-only implementation of the
**[Remote Execution APIs][reapi]**, used by **[Bazel]**, **[Buck2]** and others.

For now:

```bash
nix develop .
cargo run &
grpcurl -plaintext '[::1]:8080' list
grpcurl -d '{"instance_name":"test"}' -plaintext '[::1]:8080' build.bazel.remote.execution.v2.Capabilities/GetCapabilities
```

[reapi]: https://github.com/bazelbuild/remote-apis
[Bazel]: https://bazel.build/
[Buck2]: https://buck2.build/
