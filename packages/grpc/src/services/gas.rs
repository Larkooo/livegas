use std::{collections::HashMap, pin::Pin, sync::Arc};

use ethers::{providers::Middleware, types::BlockNumber};
use tokio::sync::{oneshot, Mutex};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::{
    pb::{
        self, gas_server::Gas, BlockRangeReply, BlockRangeRequest, BlockUpdate, Network,
        SubscriptionRequest,
    },
    provider::service::{BlockReceiver, Command, CommandSender, ProviderService},
};

pub struct GasService {
    pub providers:
        Arc<Mutex<HashMap<Network, (Arc<Mutex<ProviderService>>, CommandSender, BlockReceiver)>>>,
}

type GasResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Gas for GasService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockUpdate, Status>> + Send>>;
    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> GasResult<Self::SubscribeStream> {
        // Req payload
        let request = request.into_inner();
        // Block stream
        let providers = self.providers.clone();
        let provider = providers
            .lock()
            .await
            .get(&request.network())
            .ok_or_else(|| Status::invalid_argument("Unsupported network"))?
            .clone();

        let (tx, rx) = tokio::sync::mpsc::channel(128);

        // Spawn a task to forward the block updates to the client
        tokio::spawn(async move {
            while let Some(block) = provider.2.lock().await.recv().await {
                if let Err(e) = tx.send(block).await {
                    tracing::error!("Failed to send block update: {}", e);
                    break;
                }
            }
        });

        // Return the receiver as a stream
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::SubscribeStream
        ))
    }

    async fn blocks(&self, request: Request<BlockRangeRequest>) -> GasResult<BlockRangeReply> {
        let req = request.into_inner();
        let network = req.network(); // Ensure correct mapping to your database's network representation
        let start_block = req.start_block;
        let end_block = req.end_block;

        // start_block and end_block have to be multiples of 100
        // if start_block % 100 != 0 || end_block % 100 != 0 {
        //     return Err(Status::invalid_argument(
        //         "start_block and end_block have to be multiples of 100",
        //     ));
        // }

        let (res_tx, res_rx) = oneshot::channel();
        let providers = self.providers.lock().await;
        let provider = providers
            .get(&network)
            .ok_or_else(|| Status::invalid_argument("Unsupported network"))?;

        tracing::debug!("Sending command to fetch blocks");

        provider
            .1
            .lock()
            .await
            .send(Command::FetchBlocks {
                start_block,
                end_block,
                res: res_tx,
            })
            .await
            .map_err(|e| {
                tracing::error!("Failed to send command: {}", e);
                Status::internal("Failed to send command")
            })?;

        let blocks = res_rx.await.map_err(|e| {
            tracing::error!("Failed to receive command response: {}", e);
            Status::internal("Failed to receive command response")
        })??;

        Ok(Response::new(BlockRangeReply {
            block_updates: blocks,
        }))
    }
}
