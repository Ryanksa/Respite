mod services;

use lib::config::Config;
use services::funnel::funnel_proto::funnel_server::FunnelServer;
use services::funnel::FunnelService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let addr = config.funnel_uri.parse()?;

    let funnel_service = FunnelService::default();

    Server::builder()
        .add_service(FunnelServer::new(funnel_service))
        .serve(addr)
        .await?;
    Ok(())
}
