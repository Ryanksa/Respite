mod services;

use lib::config::Config;
use lib::db::create_pool;
use services::store::store_proto::store_server::StoreServer;
use services::store::StoreService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new();
    let addr = config.store_uri.parse()?;
    let store_service = StoreService::new(
        create_pool(config.db_uri, config.db_pool_size).await?,
        config.img_path,
    );

    Server::builder()
        .add_service(StoreServer::new(store_service))
        .serve(addr)
        .await?;
    Ok(())
}
