use std::{collections::HashMap, sync::Arc};

use ethers::{
    providers::{Middleware, Provider, Ws},
    utils::hex,
};
use sqlx::{Sqlite, SqlitePool};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

use crate::{
    insert_block,
    pb::{BlockUpdate, Network},
};

use super::storage::ProviderStorage;

pub enum Command {
    /// Command to fetch blocks within a specific range.
    FetchBlocks {
        start_block: u64,
        end_block: u64,
        res: tokio::sync::oneshot::Sender<Result<Vec<BlockUpdate>, tonic::Status>>,
    },
}

pub struct ProviderService {
    pub event_loop: Arc<Mutex<EventLoop>>,
    pub block_rx: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<BlockUpdate, tonic::Status>>>>,
    pub command_tx: Arc<Mutex<tokio::sync::mpsc::Sender<Command>>>,
}

pub struct EventLoop {
    network: Network,
    provider: Arc<Mutex<Provider<Ws>>>,
    block_tx: tokio::sync::mpsc::Sender<Result<BlockUpdate, tonic::Status>>,
    command_rx: tokio::sync::mpsc::Receiver<Command>,
    storage: ProviderStorage,
}

impl ProviderService {
    pub fn new(
        network: Network,
        provider: Arc<Mutex<Provider<Ws>>>,
        database: Arc<SqlitePool>,
    ) -> Self {
        let (block_tx, block_rx) = tokio::sync::mpsc::channel(128);
        let (command_tx, command_rx) = tokio::sync::mpsc::channel(128);

        Self {
            event_loop: Arc::new(Mutex::new(EventLoop::new(
                network, provider, database, block_tx, command_rx,
            ))),
            block_rx: Arc::new(Mutex::new(block_rx)),
            command_tx: Arc::new(Mutex::new(command_tx)),
        }
    }
}

impl EventLoop {
    pub fn new(
        network: Network,
        provider: Arc<Mutex<Provider<Ws>>>,
        database: Arc<SqlitePool>,
        block_tx: tokio::sync::mpsc::Sender<Result<BlockUpdate, tonic::Status>>,
        command_rx: tokio::sync::mpsc::Receiver<Command>,
    ) -> Self {
        Self {
            network,
            provider,
            block_tx,
            command_rx,
            storage: ProviderStorage::new(database),
        }
    }

    pub async fn run(&mut self) {
        let provider = self.provider.lock().await;

        let mut block_stream = match provider.subscribe_blocks().await {
            Ok(stream) => stream,
            Err(e) => {
                let err_msg = format!("failed to subscribe to blocks: {}", e);
                tracing::error!("{}", &err_msg);
                self.block_tx
                    .send(Err(tonic::Status::internal(err_msg)))
                    .await
                    .unwrap();
                return;
            }
        };

        loop {
            tokio::select! {
                Some(block) = block_stream.next() => {
                        // Add the block to the database
                        let network_str = self.network.clone().as_str_name();
                        let block_number = block.number.unwrap().as_u64() as u64;
                        let block_timestamp = block.timestamp.as_u64() as u64;
                        // to hex string
                        let block_hash = block.hash.unwrap().to_fixed_bytes();
                        let block_hash = format!("0x{}", hex::encode(&block_hash));
                        let gas_fee = block
                            .base_fee_per_gas
                            .map(|fee| fee.as_u64() as f64 / 1e9)
                            .unwrap();

                        tracing::info!(
                            "received block {} for network {}",
                            block_number,
                            network_str
                        );

                        match self.storage.insert_block(
                            network_str,
                            block_number as i64,
                            &block_hash,
                            block_timestamp as i64,
                            gas_fee,
                        ).await {
                            Ok(_) => {
                                tracing::info!(
                                    "inserted block {} for network {}",
                                    block_number,
                                    network_str
                                );

                                // Send the block update over the channel
                                if let Err(e) = self.block_tx
                                    .send(Ok(BlockUpdate {
                                        network: self.network.into(),
                                        block_number: block_number,
                                        block_hash: block_hash,
                                        timestamp: block_timestamp,
                                        gas_fee: gas_fee,
                                    }))
                                    .await
                                {
                                    tracing::error!("failed to send block update: {}", e);
                                    return;
                                }
                            }
                            Err(e) => {
                                tracing::error!(
                                    "failed to insert block {} for network {}: {}",
                                    block_number,
                                    network_str,
                                    e
                                );
                            }
                        }
                },
                Some(command) = self.command_rx.recv() => {
                    // Handle different commands, e.g., FetchBlocks
                    match command {
                        Command::FetchBlocks { start_block, end_block, res } => {
                            let mut blocks = vec![];

                            // Fetch and process blocks in the given range
                            for block_number in start_block..=end_block {
                                match self.storage.read_block(
                                    self.network.clone().as_str_name(),
                                    block_number as i64,
                                ).await {
                                    Ok(Some(block)) => {
                                        tracing::info!(
                                            "read block {} from storage for network {}",
                                            block_number,
                                            self.network.as_str_name()
                                        );

                                        blocks.push(BlockUpdate {
                                            network: self.network.into(),
                                            block_number: block.block_number,
                                            block_hash: block.block_hash,
                                            timestamp: block.timestamp,
                                            gas_fee: block.gas_fee,
                                        });

                                        continue;
                                    }
                                    Ok(None) => {
                                        tracing::warn!(
                                            "block {} not found for network {}",
                                            block_number,
                                            self.network.as_str_name()
                                        );
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            "failed to fetch block {} for network {}: {}",
                                            block_number,
                                            self.network.as_str_name(),
                                            e
                                        );
                                    }
                                }

                                match provider.get_block(block_number).await {
                                    Ok(Some(block)) => {
                                        tracing::info!(
                                            "fetched block {} for network {}",
                                            block_number,
                                            self.network.as_str_name()
                                        );

                                        let network_str = self.network.clone().as_str_name();
                                        let block_number = block.number.unwrap().as_u64() as u64;
                                        let block_timestamp = block.timestamp.as_u64() as u64;
                                        // to hex string
                                        let block_hash = block.hash.unwrap().to_fixed_bytes();
                                        let block_hash = format!("0x{}", hex::encode(&block_hash));
                                        let gas_fee = block
                                            .base_fee_per_gas
                                            .map(|fee| fee.as_u64() as f64 / 1e9)
                                            .unwrap_or_else(|| 0.0);

                                        match self.storage.insert_block(
                                            network_str,
                                            block_number as i64,
                                            &block_hash,
                                            block_timestamp as i64,
                                            gas_fee,
                                        ).await {
                                            Ok(_) => {
                                                tracing::info!(
                                                    "inserted block {} for network {}",
                                                    block_number,
                                                    network_str
                                                );

                                                blocks.push(BlockUpdate {
                                                    network: self.network.into(),
                                                    block_number: block_number,
                                                    block_hash: block_hash,
                                                    timestamp: block_timestamp,
                                                    gas_fee: gas_fee,
                                                });
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "failed to insert block {} for network {}: {}",
                                                    block_number,
                                                    network_str,
                                                    e
                                                );
                                            }
                                        }
                                    }
                                    Ok(None) => {
                                        tracing::warn!(
                                            "block {} not found for network {}",
                                            block_number,
                                            self.network.as_str_name()
                                        );
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            "failed to fetch block {} for network {}: {}",
                                            block_number,
                                            self.network.as_str_name(),
                                            e
                                        );
                                    }
                                }
                            }

                            let _ = res.send(Ok(blocks));
                        }
                    }
                }
            }
        }
    }
}
