use tokio_stream::wrappers::ReceiverStream;

use crate::protos::build::bazel::remote::execution::v2::{
    execution_server, ExecuteRequest, WaitExecutionRequest,
};

use crate::protos::build::bazel::remote::execution::v2::{
    capabilities_server, GetCapabilitiesRequest, ServerCapabilities,
};

use crate::protos::build::bazel::remote::execution::v2::{
    action_cache_server, ActionResult, GetActionResultRequest, UpdateActionResultRequest,
};

use crate::protos::build::bazel::remote::execution::v2::{
    content_addressable_storage_server, BatchReadBlobsRequest, BatchReadBlobsResponse,
    BatchUpdateBlobsRequest, BatchUpdateBlobsResponse, FindMissingBlobsRequest,
    FindMissingBlobsResponse, GetTreeRequest, GetTreeResponse,
};

use crate::protos::google::longrunning;

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

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
