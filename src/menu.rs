mod services;

use lib::config::Config;
use lib::db::create_pool;
use services::menu::menu_proto::menu_server::MenuServer;
use services::menu::MenuService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new();
    let addr = config.menu_uri.parse()?;
    let menu_service = MenuService::new(create_pool(config.db_uri, config.db_pool_size).await?);

    Server::builder()
        .add_service(MenuServer::new(menu_service))
        .serve(addr)
        .await?;
    Ok(())
}
