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
        req: tonic::Request<ExecuteRequest>,
    ) -> Result<tonic::Response<Self::ExecuteStream>, tonic::Status> {
        println!("execute: {:?}", req);
        Err(tonic::Status::unimplemented("execute is not implemented"))
    }

    type WaitExecutionStream = ReceiverStream<Result<longrunning::Operation, tonic::Status>>;

    async fn wait_execution(
        &self,
        req: tonic::Request<WaitExecutionRequest>,
    ) -> Result<tonic::Response<Self::WaitExecutionStream>, tonic::Status> {
        println!("wait_execution: {:?}", req);
        Err(tonic::Status::unimplemented(
            "wait_execution is not implemented",
        ))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
