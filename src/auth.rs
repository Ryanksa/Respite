mod services;

use lib::config::Config;
use services::auth::auth_proto::auth_server::AuthServer;
use services::auth::AuthService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let addr = config.auth_uri.parse()?;

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
