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
    async fn get_action_result(
        &self,
        _request: tonic::Request<GetActionResultRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        unimplemented!()
    }

    async fn update_action_result(
        &self,
        _request: tonic::Request<UpdateActionResultRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        unimplemented!()
    }
}

// ---------------------------------------------------------------------------------------------------------------------
