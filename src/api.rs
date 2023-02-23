mod services;

use lib::config::Config;
use services::api::api_proto::api_server::ApiServer;
use services::api::ApiService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new();
    let addr = config.api_uri.parse()?;

    let api_service = ApiService::new(
        config.protocol,
        config.auth_uri,
        config.store_uri,
        config.menu_uri,
        config.waiter_uri,
    );

    Server::builder()
        .add_service(ApiServer::new(api_service))
        .serve(addr)
        .await?;
    Ok(())
}
