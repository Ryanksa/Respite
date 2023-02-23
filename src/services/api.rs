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

    async fn authenticate(self, jwt: String) -> Result<Owner, Status> {
        let result = self.auth_client.verify(VerifyRequest { jwt: jwt }).await;
        match result {
            Ok(response) => {
                let res = response.into_inner();
                if let None = res.owner {
                    return Err(Status::new(Code::PermissionDenied, ""));
                }
                return Ok(res.owner.unwrap_or_default());
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
            .auth_client
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
            .auth_client
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
                let owner = res.owner.unwrap_or_default();
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
            .store_client
            .create_restaurant(CreateRestaurantRequest {
                owner_id: owner.id,
                name: req.name,
                description: req.description,
                image: req.image,
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
            .store_client
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
            .store_client
            .update_restaurant(UpdateRestaurantRequest {
                rest_id: req.rest_id,
                name: req.name,
                description: req.description,
                image: req.image,
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
    ) -> Result<Response<ApiGetRestaurantResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .store_client
            .get_restaurant(GetRestaurantRequest {
                rest_id: req.rest_id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                if let None = res.restaurant {
                    return Err(Status::new(Code::NotFound, ""));
                }
                let restaurant = res.restaurant.unwrap_or_default();
                ApiGetRestaurantResponse {
                    restaurant: Some(ApiRestaurant {
                        id: restaurant.id,
                        name: restaurant.name,
                        description: restaurant.description,
                        logo: res.image,
                    }),
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn get_restaurants(
        &self,
        request: Request<ApiGetRestaurantsRequest>,
    ) -> Result<Response<ApiGetRestaurantsResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .store_client
            .get_restaurants(GetRestaurantsRequest {
                owner_id: req.owner_id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                let restaurants = res
                    .restaurants
                    .iter()
                    .map(|&restaurant| ApiRestaurant {
                        id: restaurant.id,
                        name: restaurant.name,
                        description: restaurant.description,
                        logo: vec![],
                    })
                    .collect();
                ApiGetRestaurantsResponse {
                    restaurants: restaurants,
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn add_item(
        &self,
        request: Request<ApiAddItemRequest>,
    ) -> Result<Response<ApiAddItemResponse>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;

        let result = self
            .menu_client
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
            .menu_client
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
            .menu_client
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
    ) -> Result<Response<ApiGetItemResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .menu_client
            .get_item(GetItemRequest {
                item_id: req.item_id,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                if let None = res.item {
                    return Err(Status::new(Code::NotFound, ""));
                }
                let item = res.item.unwrap_or_default();
                ApiGetItemResponse {
                    item: Some(ApiItem {
                        id: item.id,
                        name: item.name,
                        price: item.price,
                        description: item.description,
                        category: item.category,
                        image: res.image,
                    }),
                }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn get_items(
        &self,
        request: Request<ApiGetItemsRequest>,
    ) -> Result<Response<ApiGetItemsResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .menu_client
            .get_items(GetItemsRequest {
                rest_id: req.rest_id,
                category: req.category,
            })
            .await;

        let res = match result {
            Ok(response) => {
                let res = response.into_inner();
                let items = res
                    .items
                    .iter()
                    .map(|&item| ApiItem {
                        id: item.id,
                        name: item.name,
                        price: item.price,
                        description: item.description,
                        category: item.category,
                        image: vec![],
                    })
                    .collect();
                ApiGetItemsResponse { items: items }
            }
            Err(status) => return Err(status),
        };

        Ok(Response::new(res))
    }

    async fn make_order(
        &self,
        request: Request<ApiMakeOrderRequest>,
    ) -> Result<Response<ApiMakeOrderResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .waiter_client
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
            .waiter_client
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

    type getOrdersStream = ReceiverStream<Result<ApiOrder, Status>>;

    async fn get_orders(
        &self,
        request: Request<ApiGetOrdersRequest>,
    ) -> Result<Response<Self::getOrdersStream>, Status> {
        let req = request.into_inner();
        let owner = self.authenticate(req.jwt).await?;
        let (tx, rx) = mpsc::channel(3);

        let result = self
            .waiter_client
            .get_orders(GetOrdersRequest {
                rest_id: req.rest_id,
                since: req.since,
                owner_id: owner.id,
            })
            .await;

        if let Err(status) = result {
            return Err(status);
        }

        tokio::spawn(async move {
            let stream = result.unwrap().into_inner();
            while let Ok(Some(order)) = stream.message().await {
                let api_order = ApiOrder {
                    id: order.id,
                    item_name: order.item_name,
                    requested_at: order.requested_at,
                    completed: order.completed,
                    table_number: order.table_number,
                    description: order.description,
                };
                if let Err(err) = tx.send(Ok(api_order)).await {
                    log::warn!("Api Service: {}", err);
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
