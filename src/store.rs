mod services;

use lib::config::Config;
use lib::db::create_pool;
use services::store::store_proto::store_server::StoreServer;
use services::store::StoreService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let addr = config.store_uri.parse()?;

    let store_service = StoreService::new(create_pool().await?);

    Server::builder()
        .add_service(StoreServer::new(store_service))
        .serve(addr)
        .await?;
    Ok(())
}
