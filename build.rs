fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "./protos/google/bytestream/bytestream.proto",
        "./protos/build/bazel/remote/asset/v1/remote_asset.proto",
        "./protos/build/bazel/remote/execution/v2/remote_execution.proto",
        "./protos/build/bazel/remote/logstream/v1/remote_logstream.proto",
        "./protos/build/bazel/semver/semver.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(protos, &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
