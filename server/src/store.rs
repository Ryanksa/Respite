mod config;
mod services;

use config::Config;
use services::store::store_proto::store_server::StoreServer;
use services::store::StoreService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::new();
    let addr = config.store_socket_addr.parse()?;

    let store_service = StoreService::default();

    Server::builder()
        .add_service(StoreServer::new(store_service))
        .serve(addr)
        .await?;
    Ok(())
}
