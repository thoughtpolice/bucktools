//! Happy Fun Ball. Do not taunt.

// ---------------------------------------------------------------------------------------------------------------------

mod protos;
mod remote_asset;
mod remote_execution;
mod remote_logstream;

const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("reapi_descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();

    let logstream_service = remote_logstream::LogStreamingService::default();

    let asset_fetching_service = remote_asset::AssetFetchingService::default();
    let asset_pushing_service = remote_asset::AssetPushingService::default();

    let capabilities_service = remote_execution::CapabilitiesService::default();
    let cas_service = remote_execution::ContentAddressableStorageService::default();
    let action_cache_service = remote_execution::ActionCacheService::default();
    let execution_service = remote_execution::ExecutionService::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    use crate::protos::build::bazel::remote::asset::v1::{
        fetch_server::FetchServer, push_server::PushServer,
    };
    use crate::protos::build::bazel::remote::execution::v2::{
        action_cache_server::ActionCacheServer, capabilities_server::CapabilitiesServer,
        content_addressable_storage_server::ContentAddressableStorageServer,
        execution_server::ExecutionServer,
    };
    use crate::protos::build::bazel::remote::logstream::v1::log_stream_service_server::LogStreamServiceServer;

    tonic::transport::Server::builder()
        .add_service(LogStreamServiceServer::new(logstream_service))
        .add_service(FetchServer::new(asset_fetching_service))
        .add_service(PushServer::new(asset_pushing_service))
        .add_service(CapabilitiesServer::new(capabilities_service))
        .add_service(ContentAddressableStorageServer::new(cas_service))
        .add_service(ActionCacheServer::new(action_cache_service))
        .add_service(ExecutionServer::new(execution_service))
        .add_service(reflection_service)
        .serve(address)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
