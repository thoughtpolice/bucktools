use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "./protos/google/bytestream/bytestream.proto",
        "./protos/build/bazel/remote/execution/v2/remote_execution.proto",
        "./protos/build/bazel/semver/semver.proto",
    ];

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("reapi_descriptor.bin"))
        .compile(protos, &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
