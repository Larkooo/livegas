use std::{collections::HashMap, pin::Pin, sync::Arc};

use ethers::providers::Middleware;
use tokio::sync::Mutex;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::pb::{
    self, gas_server::Gas, BlockRangeReply, BlockRangeRequest, BlockUpdate, Network,
    SubscriptionRequest,
};
pub struct GasService {
    // A hashmap of providers for each of our supported networks
    pub(crate) provider:
        HashMap<Network, Arc<Mutex<ethers::providers::Provider<ethers::providers::Ws>>>>,
    pub(crate) database: Arc<sqlx::sqlite::SqlitePool>,
}

type GasResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Gas for GasService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockUpdate, Status>> + Send>>;
    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> GasResult<Self::SubscribeStream> {
        // Our channel for our mapped block stream - of block updates.
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        // Shared provider instance for the block stream
        let provider = self.provider.clone();

        tokio::spawn(async move {
            let provider = provider.lock().await;
            let block_stream = provider.subscribe_blocks().await.map_err(|e| {
                Status::internal(format!("failed to subscribe to blocks: {}", e.to_string()))
            });

            if let Err(e) = block_stream {
                if let Err(e) = tx.send(Err(e)).await {
                    tracing::error!("failed to send block update: {}", e);
                }
                return;
            }
            let block_stream = block_stream.unwrap();

            // Stream blocks and map them into our BlockUpdates to send over the channel
            let mut block_stream = block_stream;
            while let Some(block) = block_stream.next().await {
                let block_update = BlockUpdate {
                    network: Network::EthMainnet.into(),
                    block_number: block.number.unwrap().as_u64() as u64,
                    block_hash: block.hash.unwrap().to_string(),
                    // convert wei to gwei and u256 to f64
                    gas_fee: block
                        .base_fee_per_gas
                        .map(|fee| fee.as_u64() as f64 / 1e9)
                        .unwrap(),
                };

                if let Err(e) = tx.send(Ok(block_update)).await {
                    tracing::error!("failed to send block update: {}", e);
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
        // Extract the network and block range from the request
        let req = request.into_inner();
        let network = req.network as i32; // Assuming you have a direct mapping of enum to i32
        let start_block = req.start_block as i64;
        let end_block = req.end_block as i64;

        // Prepare a SQL query to select blocks within the specified range
        let query = "
            SELECT network, block_number, block_hash, gas_fee
            FROM blocks
            WHERE network = ?
              AND blockNumber BETWEEN ? AND ?
            ORDER BY blockNumber ASC;
        ";

        // Acquire a connection from the pool
        let mut conn = self.database.acquire().await.map_err(|e| {
            tracing::error!("Failed to acquire database connection: {}", e);
            Status::internal("Failed to access database")
        })?;

        // Execute the query
        let blocks: Vec<BlockUpdate> = sqlx::query_as(query)
            .bind(network)
            .bind(start_block)
            .bind(end_block)
            .fetch_all(&mut conn)
            .await
            .map_err(|e| {
                tracing::error!("Database query failed: {}", e);
                Status::internal("Database query failed")
            })?
            .iter()
            .map(|row| BlockUpdate {
                network: row.network, // You might need to map this integer back to your Network enum
                block_number: row.blockNumber as u64,
                block_hash: row.blockHash.clone(),
                gas_fee: row.gasFee,
            })
            .collect();

        Ok(Response::new(BlockRangeReply {
            block_updates: blocks,
        }))
    }
}
