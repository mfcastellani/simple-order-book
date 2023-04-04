mod grpc;
mod binance;
//mod bitstamp;

use tonic::transport::Server;
use grpc::healthcheck;
use simple_logger::SimpleLogger;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let addr = "[::1]:50051".parse()?;

    log::info!("Server started [{:?}]! We are good to go!", addr);

    tokio::spawn(async move {
        binance::processor::process().await;
    });

    Server::builder()
        .add_service(healthcheck::build_server())
        .serve(addr)
        .await?;

    Ok(())
}