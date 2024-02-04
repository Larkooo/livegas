mod provider;
mod services;

pub mod pb {
    use tonic::include_proto;

    include_proto!("gas");
}

use std::collections::HashMap;
use std::sync::Arc;

use ethers::providers::{Middleware, Provider, Ws};
use provider::service::ProviderService;
use sqlx::sqlite::SqliteQueryResult;
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::pb::gas_server::GasServer;
use crate::pb::Network;
use crate::services::gas::GasService;

use dotenv::dotenv;

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

async fn setup_ethereum_provider(
    database: Arc<sqlx::sqlite::SqlitePool>,
    providers: Arc<Mutex<HashMap<Network, Arc<Mutex<ProviderService>>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider_url = std::env::var("PROVIDER_ETH_WSS_URL").unwrap();
    let provider = Arc::new(Mutex::new(Provider::connect(provider_url).await?));
    let network = Network::EthMainnet;
    providers.lock().await.insert(
        network,
        Arc::new(Mutex::new(ProviderService::new(
            network,
            provider,
            database.clone(),
        ))),
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable logging
    tracing_subscriber::fmt::init();
    // Load the .env file
    dotenv().ok();

    // Sqlite database for caching
    let db_url = std::env::var("DATABASE_URL")?;
    let database = Arc::new(sqlx::sqlite::SqlitePool::connect(&db_url).await?);

    // Hashmap of supported providers
    let providers = Arc::new(Mutex::new(HashMap::new()));

    // Setup the Ethereum provider
    setup_ethereum_provider(database, providers.clone()).await?;

    // Start listening to block updates for each provider
    for (_, provider) in providers.clone().lock().await.iter() {
        let provider = provider.clone();
        let event_loop = provider.lock().await.event_loop.clone();
        tokio::spawn(async move {
            event_loop.lock().await.run().await;
        });
    }

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<GasServer<GasService>>().await;
    let gas = GasService { providers };
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
