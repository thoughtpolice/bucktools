// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use tonic;

use crate::protos::build::bazel::remote::execution::v2::{
    action_cache_server, ActionResult, GetActionResultRequest, UpdateActionResultRequest,
};

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ActionCacheService {}

#[tonic::async_trait]
impl action_cache_server::ActionCache for ActionCacheService {
    #[tracing::instrument]
    async fn get_action_result(
        &self,
        _req: tonic::Request<GetActionResultRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        Err(tonic::Status::unimplemented(
            "get_action_result is not implemented",
        ))
    }

    #[tracing::instrument]
    async fn update_action_result(
        &self,
        _req: tonic::Request<UpdateActionResultRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        Err(tonic::Status::unimplemented(
            "update_action_result is not implemented",
        ))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
