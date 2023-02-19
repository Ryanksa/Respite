mod services;

use lib::config::Config;
use services::waiter::waiter_proto::waiter_server::WaiterServer;
use services::waiter::WaiterService;
use tonic::transport::Server;
use lib::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new();
    let addr = config.waiter_uri.parse()?;
    let waiter_service = WaiterService::new(create_pool().await?);

    Server::builder()
        .add_service(WaiterServer::new(waiter_service))
        .serve(addr)
        .await?;
    Ok(())
}
