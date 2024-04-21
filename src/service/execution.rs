// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use tokio_stream::wrappers::ReceiverStream;
use tonic;

use crate::protos::build::bazel::remote::execution::v2::{
    execution_server, ExecuteRequest, WaitExecutionRequest,
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
