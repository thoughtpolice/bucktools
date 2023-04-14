//! Remote Execution Asset API.

use crate::protos::build::bazel::remote::asset::v1::{
    fetch_server, FetchBlobRequest, FetchBlobResponse, FetchDirectoryRequest,
    FetchDirectoryResponse,
};

use crate::protos::build::bazel::remote::asset::v1::{
    push_server, PushBlobRequest, PushBlobResponse, PushDirectoryRequest, PushDirectoryResponse,
};

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

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
