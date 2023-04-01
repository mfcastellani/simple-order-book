mod grpc;
mod binance;

use tonic::transport::Server;
use grpc::healthcheck;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("Server started [{:?}]! We are good to go!", addr);

    tokio::spawn(async move {
        binance::processor::process().await;
    });

    Server::builder()
        .add_service(healthcheck::build_server())
        .serve(addr)
        .await?;

    Ok(())
}