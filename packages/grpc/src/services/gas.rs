use std::{future::Future, pin::Pin};

use tonic::{Request, Response, Status};

use crate::pb::{
    self, gas_server::Gas, BlockRangeReply, BlockRangeRequest, BlockUpdate, SubscriptionRequest,
};

#[derive(Default)]
pub struct GasService {}
type GasResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl<'a> Gas for GasService {
    type SubscribeStream = tonic::Streaming<BlockUpdate>;
    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> GasResult<Self::SubscribeStream> {
        Err(Status::unimplemented("mimi"))
    }

    async fn blocks(&self, request: Request<BlockRangeRequest>) -> GasResult<BlockRangeReply> {
        Err(Status::unimplemented("mimi"))
    }
}
