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
        req: tonic::Request<ReadRequest>,
    ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status> {
        tracing::warn!(
            service = "ByteStream",
            method = "read",
            request = format!("{:?}", req)
        );
        Err(tonic::Status::unimplemented("read is not implemented"))
    }

    async fn write(
        &self,
        req: tonic::Request<tonic::Streaming<WriteRequest>>,
    ) -> Result<tonic::Response<WriteResponse>, tonic::Status> {
        tracing::warn!(
            service = "ByteStream",
            method = "write",
            request = format!("{:?}", req)
        );
        Err(tonic::Status::unimplemented("write is not implemented"))
    }

    async fn query_write_status(
        &self,
        req: tonic::Request<QueryWriteStatusRequest>,
    ) -> Result<tonic::Response<QueryWriteStatusResponse>, tonic::Status> {
        tracing::warn!(
            service = "ByteStream",
            method = "query_write_status",
            request = format!("{:?}", req)
        );
        Err(tonic::Status::unimplemented(
            "query_write_status is not implemented",
        ))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
