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
        _request: tonic::Request<ListOperationsRequest>,
    ) -> Result<tonic::Response<ListOperationsResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn get_operation(
        &self,
        _request: tonic::Request<GetOperationRequest>,
    ) -> Result<tonic::Response<Operation>, tonic::Status> {
        unimplemented!()
    }

    async fn delete_operation(
        &self,
        _request: tonic::Request<DeleteOperationRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        unimplemented!()
    }

    async fn cancel_operation(
        &self,
        _request: tonic::Request<CancelOperationRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        unimplemented!()
    }

    async fn wait_operation(
        &self,
        _request: tonic::Request<WaitOperationRequest>,
    ) -> Result<tonic::Response<Operation>, tonic::Status> {
        unimplemented!()
    }
}

// ---------------------------------------------------------------------------------------------------------------------
