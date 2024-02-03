mod services;

pub mod pb {
    use tonic::include_proto;

    include_proto!("gas");
}

use tonic::transport::Server;

use crate::pb::gas_server::GasServer;
use crate::services::gas::GasService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<GasServer<GasService>>().await;

    let addr = "[::1]:50051".parse().unwrap();
    let gas = GasService::default();

    println!("HealthServer + GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GasServer::new(gas))
        .serve(addr)
        .await?;

    Ok(())
}
