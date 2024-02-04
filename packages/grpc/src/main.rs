mod services;

pub mod pb {
    use tonic::include_proto;

    include_proto!("gas");
}

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use ethers::providers::{Middleware, Provider, Ws};
use pb::BlockUpdate;
use sqlx::sqlite::SqliteQueryResult;
use tokio::sync::{oneshot, Mutex};
use tokio_stream::StreamExt;
use tonic::transport::Server;

use crate::pb::gas_server::GasServer;
use crate::pb::Network;
use crate::services::gas::GasService;

use dotenv::dotenv;
use ethers::utils::hex;

use http::header::HeaderName;
use tower_http::cors::{AllowOrigin, CorsLayer};

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 4] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

enum Command {
    /// Command to fetch blocks within a specific range.
    FetchBlocks {
        start_block: u64,
        end_block: u64,
        res: oneshot::Sender<Result<Vec<BlockUpdate>, tonic::Status>>,
    },
}

async fn insert_block(
    network: Network,
    block_number: u64,
    block_hash: String,
    block_timestamp: u64,
    gas_fee: f64,
    database: Arc<sqlx::sqlite::SqlitePool>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let network_str = network.as_str_name();
    let mut conn = database.acquire().await.unwrap();
    let mapped_block_number = block_number as i64;
    let mapped_block_timestamp = block_timestamp as i64;

    sqlx::query!(
        r#"
        INSERT INTO blocks (network, block_number, block_hash, block_timestamp, gas_fee)
        VALUES (?, ?, ?, ?, ?)
        "#,
        network_str,
        mapped_block_number,
        block_hash,
        mapped_block_timestamp,
        gas_fee
    )
    .execute(conn.as_mut())
    .await
}

// Start listening to block updates for the given provider
// Return a shared receiver for the block updates
async fn start_listening_blocks(
    provider: Arc<Mutex<Provider<Ws>>>,
    network: Network,
    database: Arc<sqlx::sqlite::SqlitePool>,
    cmd_rx: Arc<Mutex<tokio::sync::mpsc::Receiver<Command>>>,
) -> Arc<Mutex<tokio::sync::mpsc::Receiver<Result<BlockUpdate, tonic::Status>>>> {
    let provider = provider.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
        let mut provider = provider.lock().await;
        let mut cmd_rx = cmd_rx.lock().await;

        let mut block_stream = match provider.subscribe_blocks().await {
            Ok(stream) => stream,
            Err(e) => {
                let err_msg = format!("failed to subscribe to blocks: {}", e);
                tracing::error!("{}", &err_msg);
                tx.send(Err(tonic::Status::internal(err_msg)))
                    .await
                    .unwrap();
                return;
            }
        };

        // Use select to listen to both the block stream and command receiver
        tokio::select! {
            Some(block) = block_stream.next() => {
                    // Add the block to the database
                    let network_str = network.clone().as_str_name();
                    let block_number = block.number.unwrap().as_u64() as u64;
                    let block_timestamp = block.timestamp.as_u64() as u64;
                    // to hex string
                    let block_hash = block.hash.unwrap().to_fixed_bytes();
                    let block_hash = format!("0x{}", hex::encode(&block_hash));
                    let gas_fee = block
                        .base_fee_per_gas
                        .map(|fee| fee.as_u64() as f64 / 1e9)
                        .unwrap();

                    match insert_block(network, block_number, block_hash.clone(), block_timestamp, gas_fee, database.clone()).await {
                        Ok(_) => {
                            tracing::info!(
                                "inserted block {} for network {}",
                                block_number,
                                network_str
                            );
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

                    // Send the block update over the channel
                    if let Err(e) = tx
                        .send(Ok(BlockUpdate {
                            network: network.into(),
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
            },
            Some(command) = cmd_rx.recv() => {
                // Handle different commands, e.g., FetchBlocks
                match command {
                    Command::FetchBlocks { start_block, end_block, res } => {
                        let mut blocks = vec![];

                        // Fetch and process blocks in the given range
                        for block_number in start_block..=end_block {
                            match provider.get_block(block_number).await {
                                Ok(Some(block)) => {
                                    let network_str = network.clone().as_str_name();
                                    let block_number = block.number.unwrap().as_u64() as u64;
                                    let block_timestamp = block.timestamp.as_u64() as u64;
                                    // to hex string
                                    let block_hash = block.hash.unwrap().to_fixed_bytes();
                                    let block_hash = format!("0x{}", hex::encode(&block_hash));
                                    let gas_fee = block
                                        .base_fee_per_gas
                                        .map(|fee| fee.as_u64() as f64 / 1e9)
                                        .unwrap_or_else(|| 0.0);

                                    match insert_block(network, block_number, block_hash.clone(), block_timestamp, gas_fee, database.clone()).await {
                                        Ok(_) => {
                                            blocks.push(BlockUpdate {
                                                network: network.into(),
                                                block_number: block_number,
                                                block_hash: block_hash,
                                                timestamp: block_timestamp,
                                                gas_fee: gas_fee,
                                            });

                                            tracing::info!(
                                                "inserted block {} for network {}",
                                                block_number,
                                                network_str
                                            );
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
                                        network.as_str_name()
                                    );
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "failed to fetch block {} for network {}: {}",
                                        block_number,
                                        network.as_str_name(),
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
    });

    Arc::new(Mutex::new(rx))
}

async fn setup_ethereum_provider(
    providers: Arc<Mutex<HashMap<Network, Arc<Mutex<Provider<Ws>>>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider_url = std::env::var("PROVIDER_ETH_WSS_URL").unwrap();
    let provider = Arc::new(Mutex::new(Provider::connect(provider_url).await?));
    let network = Network::EthMainnet;
    providers.lock().await.insert(network, provider);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();
    // Hashmap of supported providers
    let providers = Arc::new(Mutex::new(HashMap::new()));
    // Hashmap of block update channels
    let block_recvs = Arc::new(Mutex::new(HashMap::new()));
    // Command channel for fetching blocks
    let command_tx = Arc::new(Mutex::new(HashMap::new()));

    // Setup the Ethereum provider
    setup_ethereum_provider(providers.clone()).await?;

    // Sqlite database for caching
    let db_url = std::env::var("DATABASE_URL")?;
    let database = Arc::new(sqlx::sqlite::SqlitePool::connect(&db_url).await?);

    // Start listening to block updates for each provider
    for (network, provider) in providers.clone().lock().await.iter() {
        let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(128);

        let block_recv = start_listening_blocks(
            provider.clone(),
            *network,
            database.clone(),
            Arc::new(Mutex::new(cmd_rx)),
        )
        .await;
        block_recvs
            .lock()
            .await
            .insert(network.clone(), block_recv.clone());
        command_tx.lock().await.insert(network.clone(), Arc::new(Mutex::new(cmd_tx)));
    }

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<GasServer<GasService>>().await;
    let gas = GasService {
        block_rx: block_recvs,
        command_tx,
    };
    let gas_service = GasServer::new(gas);

    let addr = "127.0.0.1:8081".parse().unwrap();

    Server::builder()
        .accept_http1(true)
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(tonic_web::enable(health_service))
        .add_service(tonic_web::enable(gas_service))
        .serve(addr)
        .await?;

    Ok(())
}
