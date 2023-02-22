use crate::services::auth::auth_proto::auth_client::AuthClient;
use crate::services::auth::auth_proto::*;
use crate::services::menu::menu_proto::menu_client::MenuClient;
use crate::services::menu::menu_proto::*;
use crate::services::store::store_proto::store_client::StoreClient;
use crate::services::store::store_proto::*;
use crate::services::waiter::waiter_proto::waiter_client::WaiterClient;
use crate::services::waiter::waiter_proto::*;
use api_proto::api_server::Api;
use api_proto::*;
use tonic::{Code, Request, Response, Status};
use tonic::transport::Channel;

pub mod api_proto {
    tonic::include_proto!("api");
}

pub struct ApiService {
    auth_client: AuthClient<Channel>,
    store_client: StoreClient<Channel>,
    menu_client: MenuClient<Channel>,
    waiter_client: WaiterClient<Channel>,
}

impl ApiService {
    pub fn new(
        auth_client: AuthClient<Channel>,
        store_client: StoreClient<Channel>,
        menu_client: MenuClient<Channel>,
        waiter_client: WaiterClient<Channel>,
    ) -> Self {
        ApiService {
            auth_client: auth_client,
            store_client: store_client,
            menu_client: menu_client,
            waiter_client: waiter_client,
        }
    }
}

#[tonic::async_trait]
impl Api for ApiService {}
