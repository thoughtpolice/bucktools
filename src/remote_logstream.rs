//! Remote Logstream API.

use crate::protos::build::bazel::remote::logstream::v1::{
    log_stream_service_server, CreateLogStreamRequest, LogStream,
};

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
