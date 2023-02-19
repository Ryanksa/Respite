mod services;

use lib::config::Config;
use services::waiter::waiter_proto::waiter_server::WaiterServer;
use services::waiter::WaiterService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let addr = config.waiter_uri.parse()?;

    let waiter_service = WaiterService::default();

    Server::builder()
        .add_service(WaiterServer::new(waiter_service))
        .serve(addr)
        .await?;
    Ok(())
}