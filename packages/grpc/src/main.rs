mod services;

pub mod pb {
    use tonic::include_proto;

    include_proto!("gas");
}

use std::sync::Arc;

use ethers::providers::{Middleware, Provider};
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::pb::gas_server::GasServer;
use crate::services::gas::GasService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // WSS url of the ethereum provider
    let provider_url = std::env::var("PROVIDER_WSS_URL")?;
    // A shared provider instance
    let provider = Arc::new(Mutex::new(Provider::connect(provider_url).await?));

    // Sqlite database for caching
    let db_url = std::env::var("DATABASE_URL")?;
    let database = Arc::new(sqlx::sqlite::SqlitePool::connect(&db_url).await?);

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<GasServer<GasService>>().await;

    let addr = "[::1]:50051".parse().unwrap();
    let gas = GasService { provider, database };

    println!("HealthServer + GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GasServer::new(gas))
        .serve(addr)
        .await?;

    Ok(())
}
