use tonic::{Request, Response, Status};

use health_check::health_check_server::{HealthCheck, HealthCheckServer};
use health_check::{HealthCheckReply, HealthCheckRequest};

pub mod health_check {
    tonic::include_proto!("healthcheck");
    tonic::include_proto!("orderbook");
}

#[derive(Debug, Default)]
pub struct BasicHealthCheck {}

#[tonic::async_trait]
impl HealthCheck for BasicHealthCheck {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckReply>, Status> {
        println!("healthcheck: {:?}", request);

        let reply = health_check::HealthCheckReply {
            status: format!("Server is up and running!").into(),
            binance: format!("Connected - last read xx/xx/xx xx:xx").into(),
            bitstamp: format!("Connected - last read xx/xx/xx xx:xx").into(),
        };
        Ok(Response::new(reply))
    }
}

pub fn build_server() -> HealthCheckServer<BasicHealthCheck> {
    let healthcheck: BasicHealthCheck = BasicHealthCheck::default();
    HealthCheckServer::new(healthcheck)
}