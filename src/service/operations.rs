// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use tonic;

use crate::protos::google::longrunning::{
    operations_server, CancelOperationRequest, DeleteOperationRequest, GetOperationRequest,
    ListOperationsRequest, ListOperationsResponse, Operation, WaitOperationRequest,
};

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct OperationsService {}

#[tonic::async_trait]
impl operations_server::Operations for OperationsService {
    async fn list_operations(
        &self,
        req: tonic::Request<ListOperationsRequest>,
    ) -> Result<tonic::Response<ListOperationsResponse>, tonic::Status> {
        println!("list_operations: {:?}", req);
        Err(tonic::Status::unimplemented(
            "list_operations is not implemented",
        ))
    }

    async fn get_operation(
        &self,
        req: tonic::Request<GetOperationRequest>,
    ) -> Result<tonic::Response<Operation>, tonic::Status> {
        println!("get_operation: {:?}", req);
        Err(tonic::Status::unimplemented(
            "get_operation is not implemented",
        ))
    }

    async fn delete_operation(
        &self,
        req: tonic::Request<DeleteOperationRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        println!("delete_operation: {:?}", req);
        Err(tonic::Status::unimplemented(
            "delete_operation is not implemented",
        ))
    }

    async fn cancel_operation(
        &self,
        req: tonic::Request<CancelOperationRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        println!("cancel_operation: {:?}", req);
        Err(tonic::Status::unimplemented(
            "cancel_operation is not implemented",
        ))
    }

    async fn wait_operation(
        &self,
        req: tonic::Request<WaitOperationRequest>,
    ) -> Result<tonic::Response<Operation>, tonic::Status> {
        println!("wait_operation: {:?}", req);
        Err(tonic::Status::unimplemented(
            "wait_operation is not implemented",
        ))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
