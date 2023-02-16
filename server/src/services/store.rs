use lib::db::get_pool_grpc;
use sqlx::postgres::PgRow;
use sqlx::{query, Row};
use std::fs;
use store_proto::store_server::Store;
use store_proto::{
    AddItemRequest, AddItemResponse, CreateRestaurantRequest, CreateRestaurantResponse,
    DeleteRestaurantRequest, DeleteRestaurantResponse, GetItemRequest, GetItemResponse,
    GetItemsRequest, GetItemsResponse, GetRestaurantRequest, GetRestaurantResponse,
    GetRestaurantsRequest, GetRestaurantsResponse, Item, RemoveItemRequest, RemoveItemResponse,
    Restaurant, UploadItemImageRequest, UploadItemImageResponse, UploadRestaurantLogoRequest,
    UploadRestaurantLogoResponse,
};
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

pub mod store_proto {
    tonic::include_proto!("store");
}

#[derive(Debug, Default)]
pub struct StoreService {}

#[tonic::async_trait]
impl Store for StoreService {
    async fn add_item(
        &self,
        request: Request<AddItemRequest>,
    ) -> Result<Response<AddItemResponse>, Status> {
        let req = request.into_inner();
        let item_id = Uuid::new_v4();

        let pool = get_pool_grpc().await?;
        let db_result = query("INSERT INTO items VALUES ($1, $2, $3, $4, $5)")
            .bind(item_id.to_string())
            .bind(req.name)
            .bind(req.description)
            .bind(req.category)
            .bind(req.rest_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => AddItemResponse {
                item_id: item_id.to_string(),
            },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn remove_item(
        &self,
        request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("DELETE FROM items WHERE id = $1")
            .bind(req.item_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => RemoveItemResponse { success: true },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn upload_item_image(
        &self,
        request: Request<UploadItemImageRequest>,
    ) -> Result<Response<UploadItemImageResponse>, Status> {
        let req = request.into_inner();
        let path = format!("./img/{}", req.item_id);

        if let Err(err) = fs::write(path.clone(), req.image) {
            return Err(Status::new(Code::Internal, format!("{}", err)));
        };

        let pool = get_pool_grpc().await?;
        let db_result = query("UPDATE items SET image = $1 WHERE id = $2")
            .bind(path)
            .bind(req.item_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => UploadItemImageResponse { success: true },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<GetItemResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("SELECT * FROM items WHERE id = $1")
            .bind(req.item_id)
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                rest_id: row.get("rest_id"),
            })
            .fetch_one(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(item) => GetItemResponse { item: Some(item) },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn get_items(
        &self,
        request: Request<GetItemsRequest>,
    ) -> Result<Response<GetItemsResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result =
            query("SELECT * FROM items WHERE rest_id = $1 AND ($2 = '' OR category = $2)")
                .bind(req.rest_id)
                .bind(req.category)
                .map(|row: PgRow| Item {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    category: row.get("category"),
                    rest_id: row.get("rest_id"),
                })
                .fetch_all(pool.as_ref())
                .await;

        let res = match db_result {
            Ok(items) => GetItemsResponse { items: items },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn create_restaurant(
        &self,
        request: Request<CreateRestaurantRequest>,
    ) -> Result<Response<CreateRestaurantResponse>, Status> {
        let req = request.into_inner();
        let rest_id = Uuid::new_v4();

        let pool = get_pool_grpc().await?;
        let db_result = query("INSERT INTO restaurants VALUES ($1, $2, $3, $4)")
            .bind(rest_id.to_string())
            .bind(req.name)
            .bind(req.description)
            .bind(req.owner_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => CreateRestaurantResponse {
                rest_id: rest_id.to_string(),
            },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn delete_restaurant(
        &self,
        request: Request<DeleteRestaurantRequest>,
    ) -> Result<Response<DeleteRestaurantResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("DELETE FROM restaurants WHERE id = $1")
            .bind(req.rest_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => DeleteRestaurantResponse { success: true },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn upload_restaurant_logo(
        &self,
        request: Request<UploadRestaurantLogoRequest>,
    ) -> Result<Response<UploadRestaurantLogoResponse>, Status> {
        let req = request.into_inner();
        let path = format!("./img/{}", req.rest_id);

        if let Err(err) = fs::write(path.clone(), req.image) {
            return Err(Status::new(Code::Internal, format!("{}", err)));
        };

        let pool = get_pool_grpc().await?;
        let db_result = query("UPDATE restaurants SET image = $1 WHERE id = $2")
            .bind(path)
            .bind(req.rest_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => UploadRestaurantLogoResponse { success: true },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn get_restaurant(
        &self,
        request: Request<GetRestaurantRequest>,
    ) -> Result<Response<GetRestaurantResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("SELECT * FROM restaurants WHERE id = $1")
            .bind(req.rest_id)
            .map(|row| Restaurant {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                owner_id: row.get("owner_id"),
            })
            .fetch_one(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(restaurant) => GetRestaurantResponse {
                restaurant: Some(restaurant),
            },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn get_restaurants(
        &self,
        request: Request<GetRestaurantsRequest>,
    ) -> Result<Response<GetRestaurantsResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("SELECT * FROM restaurants WHERE owner_id = $1")
            .bind(req.owner_id)
            .map(|row| Restaurant {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                owner_id: row.get("owner_id"),
            })
            .fetch_all(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(restaurants) => GetRestaurantsResponse {
                restaurants: restaurants,
            },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }
}
