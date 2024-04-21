// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use tokio_stream::wrappers::ReceiverStream;
use tonic;

use crate::protos::google::bytestream::{
    byte_stream_server, QueryWriteStatusRequest, QueryWriteStatusResponse, ReadRequest,
    ReadResponse, WriteRequest, WriteResponse,
};

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

// ---------------------------------------------------------------------------------------------------------------------
