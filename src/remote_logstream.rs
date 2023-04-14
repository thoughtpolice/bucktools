//! Remote Logstream API.

use crate::protos::build::bazel::remote::logstream::v1::{
    log_stream_service_server, CreateLogStreamRequest, LogStream,
};

use crate::protos::google::bytestream::{
    byte_stream_server, QueryWriteStatusRequest, QueryWriteStatusResponse, ReadRequest,
    ReadResponse, WriteRequest, WriteResponse,
};

use tokio_stream::wrappers::ReceiverStream;

// ---------------------------------------------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------------------------------------------

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
