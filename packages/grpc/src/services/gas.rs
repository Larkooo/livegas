use std::{collections::HashMap, pin::Pin, sync::Arc};

use ethers::{providers::Middleware, types::BlockNumber};
use tokio::sync::Mutex;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::pb::{
    self, gas_server::Gas, BlockRangeReply, BlockRangeRequest, BlockUpdate, Network,
    SubscriptionRequest,
};
use sqlx::sqlite::SqlitePool;

pub struct GasService {
    // A hashmap of providers for each of our supported networks
    pub(crate) providers:
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
        // Req payload
        let request = request.into_inner();
        // Our channel for our mapped block stream - of block updates.
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        // Shared provider instance for the block stream
        let provider = self
            .providers
            .get(&request.network())
            .ok_or_else(|| Status::invalid_argument("unsupported network"))?
            .clone();

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
        let req = request.into_inner();
        let network = req.network(); // Ensure correct mapping to your database's network representation
        let start_block = req.start_block as i64;
        let end_block = req.end_block as i64;

        let provider = self
            .providers
            .get(&network)
            .ok_or_else(|| Status::invalid_argument("Unsupported network"))?
            .clone();

        let mut conn = self.database.acquire().await.map_err(|e| {
            tracing::error!("Failed to acquire database connection: {}", e);
            Status::internal("Failed to access database")
        })?;

        let blocks = query_blocks_from_db(&mut conn, &network, start_block, end_block)
            .await
            .map_err(|e| {
                tracing::error!("Failed to query blocks from database: {}", e);
                e
            })?;

        if blocks.len() == (end_block - start_block + 1) as usize {
            return Ok(Response::new(BlockRangeReply {
                block_updates: blocks,
            }));
        }

        // Fetch missing blocks from the provider
        let blocks = fetch_blocks_from_provider(network, &provider, start_block, end_block).await?;
        // Insert the fetched blocks into the database
        insert_blocks_into_db(network, &mut conn, &blocks).await?;

        Ok(Response::new(BlockRangeReply {
            block_updates: blocks,
        }))
    }
}

async fn fetch_blocks_from_provider(
    network: Network,
    provider: &Arc<Mutex<ethers::providers::Provider<ethers::providers::Ws>>>,
    start_block: i64,
    end_block: i64,
) -> Result<Vec<pb::BlockUpdate>, Status> {
    let provider = provider.lock().await;
    let mut blocks = Vec::new();

    for block_num in start_block..=end_block {
        // Fetch block details from the provider
        // This is simplified; adjust based on your actual provider API
        if let Ok(block) = provider
            .get_block(BlockNumber::Number(block_num.into()))
            .await
        {
            let block_update = pb::BlockUpdate {
                network: network.into(),
                block_number: block.clone().unwrap().number.unwrap().as_u64(),
                block_hash: block.clone().unwrap().hash.unwrap().to_string(),
                gas_fee: block.clone().unwrap().base_fee_per_gas.unwrap().as_u64() as f64 / 1e9,
            };
            blocks.push(block_update);
        }
    }

    Ok(blocks)
}

async fn query_blocks_from_db(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    network: &Network,
    start_block: i64,
    end_block: i64,
) -> Result<Vec<pb::BlockUpdate>, Status> {
    let network_str = network.as_str_name();
    let blocks = sqlx::query!(
        r#"
        SELECT network, block_number, block_hash, gas_fee
        FROM blocks
        WHERE network = ?
          AND block_number BETWEEN ? AND ?
        ORDER BY block_number ASC;
        "#,
        network_str,
        start_block,
        end_block
    )
    .fetch_all(conn.as_mut())
    .await
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        Status::internal("Database query failed")
    })?
    .iter()
    .map(|row| pb::BlockUpdate {
        network: Network::from_str_name(&row.network).unwrap().into(),
        block_number: row.block_number as u64,
        block_hash: row.block_hash.clone(),
        gas_fee: row.gas_fee,
    })
    .collect();

    Ok(blocks)
}

async fn insert_blocks_into_db(
    network: Network,
    conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    blocks: &[pb::BlockUpdate],
) -> Result<(), Status> {
    let network = network.as_str_name();

    for block in blocks {
        let block_number = block.block_number as i64;
        let block_hash = block.block_hash.clone();
        sqlx::query!(
            r#"
            INSERT INTO blocks (network, block_number, block_hash, gas_fee)
            VALUES (?, ?, ?, ?)
            ON CONFLICT (network, block_number) DO NOTHING;
            "#,
            network,
            block_number,
            block_hash,
            block.gas_fee
        )
        .execute(conn.as_mut())
        .await
        .map_err(|e| {
            tracing::error!("Database insert failed: {}", e);
            Status::internal("Database insert failed")
        })?;
    }
    Ok(())
}
