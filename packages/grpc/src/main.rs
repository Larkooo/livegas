mod services;

pub mod hello_world {
    use tonic::include_proto;

    include_proto!("helloworld");
}

use tonic::transport::Server;

use crate::hello_world::greeter_server::GreeterServer;
use crate::services::greeter::MyGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("HealthServer + GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}