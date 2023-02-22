mod services;

use lib::config::Config;
use services::api::api_proto::api_server::ApiServer;
use services::api::ApiService;
use services::auth::auth_proto::auth_client::AuthClient;
use services::menu::menu_proto::menu_client::MenuClient;
use services::store::store_proto::store_client::StoreClient;
use services::waiter::waiter_proto::waiter_client::WaiterClient;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new();
    let addr = config.api_uri.parse()?;

    let auth_client =
        AuthClient::connect(format!("{}://{}", config.protocol, config.auth_uri)).await?;
    let store_client =
        StoreClient::connect(format!("{}://{}", config.protocol, config.store_uri)).await?;
    let menu_client =
        MenuClient::connect(format!("{}://{}", config.protocol, config.menu_uri)).await?;
    let waiter_client =
        WaiterClient::connect(format!("{}://{}", config.protocol, config.waiter_uri)).await?;

    let api_service = ApiService::new(auth_client, store_client, menu_client, waiter_client);

    Server::builder()
        .add_service(ApiServer::new(api_service))
        .serve(addr)
        .await?;
    Ok(())
}
