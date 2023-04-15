//! Protobuf files, reflected as a Rust module hierarchy. These basically map
//! 1-to-1 with the actual protobuf namespace.ea

// ---------------------------------------------------------------------------------------------------------------------

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod bytestream {
        tonic::include_proto!("google.bytestream");
    }
    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod build {
    pub mod bazel {
        pub mod remote {
            pub mod asset {
                pub mod v1 {
                    tonic::include_proto!("build.bazel.remote.asset.v1");
                }
            }
            pub mod execution {
                pub mod v2 {
                    tonic::include_proto!("build.bazel.remote.execution.v2");
                }
            }
            pub mod logstream {
                pub mod v1 {
                    tonic::include_proto!("build.bazel.remote.logstream.v1");
                }
            }
        }

        pub mod semver {
            tonic::include_proto!("build.bazel.semver");
        }
    }
}
