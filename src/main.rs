//! Happy Fun Ball. Do not taunt.

// ---------------------------------------------------------------------------------------------------------------------

mod protos;

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
        .serve(address)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------

pub mod remote_execution {
    use tokio_stream::wrappers::ReceiverStream;

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::execution::v2::{
        execution_server, ExecuteRequest, WaitExecutionRequest,
    };

    use crate::protos::google::longrunning;

    #[derive(Debug, Default)]
    pub struct ExecutionService {}

    #[tonic::async_trait]
    impl execution_server::Execution for ExecutionService {
        type ExecuteStream = ReceiverStream<Result<longrunning::Operation, tonic::Status>>;

        async fn execute(
            &self,
            _request: tonic::Request<ExecuteRequest>,
        ) -> Result<tonic::Response<Self::ExecuteStream>, tonic::Status> {
            unimplemented!()
        }

        type WaitExecutionStream = ReceiverStream<Result<longrunning::Operation, tonic::Status>>;

        async fn wait_execution(
            &self,
            _request: tonic::Request<WaitExecutionRequest>,
        ) -> Result<tonic::Response<Self::WaitExecutionStream>, tonic::Status> {
            unimplemented!()
        }
    }

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::execution::v2::{
        action_cache_server, ActionResult, GetActionResultRequest, UpdateActionResultRequest,
    };

    #[derive(Debug, Default)]
    pub struct ActionCacheService {}

    #[tonic::async_trait]
    impl action_cache_server::ActionCache for ActionCacheService {
        async fn get_action_result(
            &self,
            _request: tonic::Request<GetActionResultRequest>,
        ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
            unimplemented!()
        }

        async fn update_action_result(
            &self,
            _request: tonic::Request<UpdateActionResultRequest>,
        ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
            unimplemented!()
        }
    }

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::execution::v2::{
        content_addressable_storage_server, BatchReadBlobsRequest, BatchReadBlobsResponse,
        BatchUpdateBlobsRequest, BatchUpdateBlobsResponse, FindMissingBlobsRequest,
        FindMissingBlobsResponse, GetTreeRequest, GetTreeResponse,
    };

    #[derive(Debug, Default)]
    pub struct ContentAddressableStorageService {}

    #[tonic::async_trait]
    impl content_addressable_storage_server::ContentAddressableStorage
        for ContentAddressableStorageService
    {
        async fn find_missing_blobs(
            &self,
            _request: tonic::Request<FindMissingBlobsRequest>,
        ) -> Result<tonic::Response<FindMissingBlobsResponse>, tonic::Status> {
            unimplemented!()
        }

        async fn batch_update_blobs(
            &self,
            _request: tonic::Request<BatchUpdateBlobsRequest>,
        ) -> Result<tonic::Response<BatchUpdateBlobsResponse>, tonic::Status> {
            unimplemented!()
        }

        async fn batch_read_blobs(
            &self,
            _request: tonic::Request<BatchReadBlobsRequest>,
        ) -> Result<tonic::Response<BatchReadBlobsResponse>, tonic::Status> {
            unimplemented!()
        }

        type GetTreeStream = ReceiverStream<Result<GetTreeResponse, tonic::Status>>;

        async fn get_tree(
            &self,
            _request: tonic::Request<GetTreeRequest>,
        ) -> Result<tonic::Response<Self::GetTreeStream>, tonic::Status> {
            unimplemented!()
        }
    }

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::execution::v2::{
        capabilities_server, GetCapabilitiesRequest, ServerCapabilities,
    };

    #[derive(Debug, Default)]
    pub struct CapabilitiesService {}

    #[tonic::async_trait]
    impl capabilities_server::Capabilities for CapabilitiesService {
        async fn get_capabilities(
            &self,
            _request: tonic::Request<GetCapabilitiesRequest>,
        ) -> Result<tonic::Response<ServerCapabilities>, tonic::Status> {
            unimplemented!()
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

pub mod remote_asset {

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::asset::v1::{
        fetch_server, FetchBlobRequest, FetchBlobResponse, FetchDirectoryRequest,
        FetchDirectoryResponse,
    };

    #[derive(Debug, Default)]
    pub struct AssetFetchingService {}

    #[tonic::async_trait]
    impl fetch_server::Fetch for AssetFetchingService {
        async fn fetch_blob(
            &self,
            _request: tonic::Request<FetchBlobRequest>,
        ) -> Result<tonic::Response<FetchBlobResponse>, tonic::Status> {
            unimplemented!()
        }

        async fn fetch_directory(
            &self,
            _request: tonic::Request<FetchDirectoryRequest>,
        ) -> Result<tonic::Response<FetchDirectoryResponse>, tonic::Status> {
            unimplemented!()
        }
    }

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::build::bazel::remote::asset::v1::{
        push_server, PushBlobRequest, PushBlobResponse, PushDirectoryRequest, PushDirectoryResponse,
    };

    #[derive(Debug, Default)]
    pub struct AssetPushingService {}

    #[tonic::async_trait]
    impl push_server::Push for AssetPushingService {
        async fn push_blob(
            &self,
            _request: tonic::Request<PushBlobRequest>,
        ) -> Result<tonic::Response<PushBlobResponse>, tonic::Status> {
            unimplemented!()
        }

        async fn push_directory(
            &self,
            _request: tonic::Request<PushDirectoryRequest>,
        ) -> Result<tonic::Response<PushDirectoryResponse>, tonic::Status> {
            unimplemented!()
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

pub mod remote_logstream {
    use crate::protos::build::bazel::remote::logstream::v1::{
        log_stream_service_server, CreateLogStreamRequest, LogStream,
    };

    #[derive(Debug, Default)]
    pub struct LogStreamingService {}

    #[tonic::async_trait]
    impl log_stream_service_server::LogStreamService for LogStreamingService {
        async fn create_log_stream(
            &self,
            _request: tonic::Request<CreateLogStreamRequest>,
        ) -> Result<tonic::Response<LogStream>, tonic::Status> {
            unimplemented!()
        }
    }

    //// ---------------------------------------------------------------------------------------------------------------

    use crate::protos::google::bytestream::{
        byte_stream_server, QueryWriteStatusRequest, QueryWriteStatusResponse, ReadRequest,
        ReadResponse, WriteRequest, WriteResponse,
    };
    use tokio_stream::wrappers::ReceiverStream;

    #[derive(Debug, Default)]
    pub struct ByteStreamService {}

    #[tonic::async_trait]
    impl byte_stream_server::ByteStream for ByteStreamService {
        type ReadStream = ReceiverStream<Result<ReadResponse, tonic::Status>>;

        async fn read(
            &self,
            _request: tonic::Request<ReadRequest>,
        ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status> {
            unimplemented!()
        }

        async fn write(
            &self,
            _request: tonic::Request<tonic::Streaming<WriteRequest>>,
        ) -> Result<tonic::Response<WriteResponse>, tonic::Status> {
            unimplemented!()
        }

        async fn query_write_status(
            &self,
            _request: tonic::Request<QueryWriteStatusRequest>,
        ) -> Result<tonic::Response<QueryWriteStatusResponse>, tonic::Status> {
            unimplemented!()
        }
    }
}
