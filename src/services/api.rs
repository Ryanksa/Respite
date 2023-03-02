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
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;
use tonic::{Code, Request, Response, Status};

pub mod api_proto {
    tonic::include_proto!("api");
}

pub struct ApiService {
    protocol: String,
    auth_uri: String,
    store_uri: String,
    menu_uri: String,
    waiter_uri: String,
}

impl ApiService {
    pub fn new(
        protocol: String,
        auth_uri: String,
        store_uri: String,
        menu_uri: String,
        waiter_uri: String,
    ) -> Self {
        ApiService {
            protocol: protocol,
            auth_uri: auth_uri,
            store_uri: store_uri,
            menu_uri: menu_uri,
            waiter_uri: waiter_uri,
        }
    }

    async fn get_auth_client(&self) -> Result<AuthClient<Channel>, Status> {
        match AuthClient::connect(format!("{}://{}", self.protocol, self.auth_uri)).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                log::error!("Api Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        }
    }

    async fn get_store_client(&self) -> Result<StoreClient<Channel>, Status> {
        match StoreClient::connect(format!("{}://{}", self.protocol, self.store_uri)).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                log::error!("Api Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        }
    }

    async fn get_menu_client(&self) -> Result<MenuClient<Channel>, Status> {
        match MenuClient::connect(format!("{}://{}", self.protocol, self.menu_uri)).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                log::error!("Api Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        }
    }

    async fn get_waiter_client(&self) -> Result<WaiterClient<Channel>, Status> {
        match WaiterClient::connect(format!("{}://{}", self.protocol, self.waiter_uri)).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                log::error!("Api Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        }
    }

    async fn authenticate(&self, jwt: String) -> Result<Owner, Status> {
        let result = self
            .get_auth_client()
            .await?
            .verify(VerifyRequest { jwt: jwt })
            .await;
        match result {
            Ok(response) => {
                let res = response.into_inner();
                if let None = res.owner {
                    return Err(Status::new(Code::PermissionDenied, ""));
                }
                return Ok(res.owner.unwrap());
            }
            Err(status) => return Err(status),
        };
    }
}

#[tonic::async_trait]
impl Api for ApiService {
    async fn sign_up(
        &self,
        request: Request<ApiSignUpRequest>,
    ) -> Result<Response<ApiSignUpResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .get_auth_client()
            .await?
            .sign_up(SignUpRequest {
                email: req.email,
                password: req.password,
            })
            .await;

        let res = match result {
            Ok(response) => ApiSignUpResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn sign_in(
        &self,
        request: Request<ApiSignInRequest>,
    ) -> Result<Response<ApiSignInResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .get_auth_client()
            .await?
            .sign_in(SignInRequest {
                email: req.email,
                password: req.password,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                if let None = res.owner {
                    return Err(Status::new(Code::PermissionDenied, ""));
                }
                let owner = res.owner.unwrap();
                ApiSignInResponse {
                    owner: Some(ApiOwner {
                        id: owner.id,
                        email: owner.email,
                    }),
                    jwt: res.jwt,
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn create_restaurant(
        &self,
        request: Request<ApiCreateRestaurantRequest>,
    ) -> Result<Response<ApiCreateRestaurantResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_store_client()
            .await?
            .create_restaurant(CreateRestaurantRequest {
                owner_id: owner.id,
                name: req.name,
                description: req.description,
                logo: req.logo,
            })
            .await;

        let res = match result {
            Ok(response) => ApiCreateRestaurantResponse {
                rest_id: response.into_inner().rest_id,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn delete_restaurant(
        &self,
        request: Request<ApiDeleteRestaurantRequest>,
    ) -> Result<Response<ApiDeleteRestaurantResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_store_client()
            .await?
            .delete_restaurant(DeleteRestaurantRequest {
                rest_id: req.rest_id,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiDeleteRestaurantResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn update_restaurant(
        &self,
        request: Request<ApiUpdateRestaurantRequest>,
    ) -> Result<Response<ApiUpdateRestaurantResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_store_client()
            .await?
            .update_restaurant(UpdateRestaurantRequest {
                rest_id: req.rest_id,
                name: req.name,
                description: req.description,
                logo: req.logo,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiUpdateRestaurantResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn get_restaurant(
        &self,
        request: Request<ApiGetRestaurantRequest>,
    ) -> Result<Response<ApiRestaurant>, Status> {
        let req = request.into_inner();

        let result = self
            .get_store_client()
            .await?
            .get_restaurant(GetRestaurantRequest {
                rest_id: req.rest_id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let restaurant = response.into_inner();
                ApiRestaurant {
                    id: restaurant.id,
                    name: restaurant.name,
                    description: restaurant.description,
                    logo: restaurant.logo,
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    type getRestaurantsStream = ReceiverStream<Result<ApiRestaurant, Status>>;

    async fn get_restaurants(
        &self,
        request: Request<ApiGetRestaurantsRequest>,
    ) -> Result<Response<Self::getRestaurantsStream>, Status> {
        let req = request.into_inner();
        let (tx, rx) = mpsc::channel(3);

        let result = self
            .get_store_client()
            .await?
            .get_restaurants(GetRestaurantsRequest {
                owner_id: req.owner_id,
            })
            .await;

        if let Err(status) = result {
            return Err(status);
        }
        let mut stream = result.unwrap().into_inner();

        tokio::spawn(async move {
            while let Ok(Some(restaurant)) = stream.message().await {
                let api_restaurant = ApiRestaurant {
                    id: restaurant.id,
                    name: restaurant.name,
                    description: restaurant.description,
                    logo: restaurant.logo,
                };
                if let Err(err) = tx.send(Ok(api_restaurant)).await {
                    log::warn!("Api Service: {}", err);
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn add_item(
        &self,
        request: Request<ApiAddItemRequest>,
    ) -> Result<Response<ApiAddItemResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_menu_client()
            .await?
            .add_item(AddItemRequest {
                rest_id: req.rest_id,
                name: req.name,
                price: req.price,
                description: req.description,
                category: req.category,
                image: req.image,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiAddItemResponse {
                item_id: response.into_inner().item_id,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn remove_item(
        &self,
        request: Request<ApiRemoveItemRequest>,
    ) -> Result<Response<ApiRemoveItemResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_menu_client()
            .await?
            .remove_item(RemoveItemRequest {
                item_id: req.item_id,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiRemoveItemResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn update_item(
        &self,
        request: Request<ApiUpdateItemRequest>,
    ) -> Result<Response<ApiUpdateItemResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_menu_client()
            .await?
            .update_item(UpdateItemRequest {
                item_id: req.item_id,
                name: req.name,
                price: req.price,
                description: req.description,
                category: req.category,
                image: req.image,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiUpdateItemResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn get_item(
        &self,
        request: Request<ApiGetItemRequest>,
    ) -> Result<Response<ApiItem>, Status> {
        let req = request.into_inner();

        let result = self
            .get_menu_client()
            .await?
            .get_item(GetItemRequest {
                item_id: req.item_id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let item = response.into_inner();
                ApiItem {
                    id: item.id,
                    name: item.name,
                    price: item.price,
                    description: item.description,
                    category: item.category,
                    image: item.image,
                    rest_id: item.rest_id,
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    type getItemsStream = ReceiverStream<Result<ApiItem, Status>>;

    async fn get_items(
        &self,
        request: Request<ApiGetItemsRequest>,
    ) -> Result<Response<Self::getItemsStream>, Status> {
        let req = request.into_inner();
        let (tx, rx) = mpsc::channel(3);

        let result = self
            .get_menu_client()
            .await?
            .get_items(GetItemsRequest {
                rest_id: req.rest_id,
                category: req.category,
            })
            .await;

        if let Err(status) = result {
            return Err(status);
        }
        let mut stream = result.unwrap().into_inner();

        tokio::spawn(async move {
            while let Ok(Some(item)) = stream.message().await {
                let api_item = ApiItem {
                    id: item.id,
                    name: item.name,
                    price: item.price,
                    description: item.description,
                    category: item.category,
                    image: item.image,
                    rest_id: item.rest_id,
                };
                if let Err(err) = tx.send(Ok(api_item)).await {
                    log::warn!("Api Service: {}", err);
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn make_order(
        &self,
        request: Request<ApiMakeOrderRequest>,
    ) -> Result<Response<ApiMakeOrderResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .get_waiter_client()
            .await?
            .make_order(MakeOrderRequest {
                item_id: req.item_id,
                table_number: req.table_number,
                description: req.description,
            })
            .await;

        let res = match result {
            Ok(response) => ApiMakeOrderResponse {
                order_id: response.into_inner().order_id,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn complete_order(
        &self,
        request: Request<ApiCompleteOrderRequest>,
    ) -> Result<Response<ApiCompleteOrderResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_waiter_client()
            .await?
            .complete_order(CompleteOrderRequest {
                order_id: req.order_id,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => ApiCompleteOrderResponse {
                success: response.into_inner().success,
            },
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn get_orders(
        &self,
        request: Request<ApiGetOrdersRequest>,
    ) -> Result<Response<ApiGetOrdersResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .get_waiter_client()
            .await?
            .get_orders(GetOrdersRequest {
                rest_id: req.rest_id,
                since: req.since,
                owner_id: owner.id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                let orders = res
                    .orders
                    .iter()
                    .map(|order| ApiOrder {
                        id: order.id.clone(),
                        item_name: order.item_name.clone(),
                        requested_at: order.requested_at,
                        completed: order.completed,
                        table_number: order.table_number,
                        description: order.description.clone(),
                    })
                    .collect();
                ApiGetOrdersResponse { orders: orders }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }
}
