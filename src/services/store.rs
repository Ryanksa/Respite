use lib::db::{
    delete_item, delete_restaurant, get_item, get_items, get_restaurant, get_restaurants,
    insert_item, insert_restaurant, upload_item_image, upload_restaurant_logo,
};
use sqlx::{Pool, Postgres, Row};
use std::fs;
use std::sync::Arc;
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

pub struct StoreService {
    pool: Arc<Pool<Postgres>>,
}

impl StoreService {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        StoreService { pool: pool }
    }
}

#[tonic::async_trait]
impl Store for StoreService {
    async fn add_item(
        &self,
        request: Request<AddItemRequest>,
    ) -> Result<Response<AddItemResponse>, Status> {
        let req = request.into_inner();
        let item_id = Uuid::new_v4();

        let db_result = insert_item(
            item_id.to_string(),
            req.name,
            req.description,
            req.category,
            req.rest_id,
        )
        .execute(self.pool.as_ref())
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

        let db_result = delete_item(req.item_id).execute(self.pool.as_ref()).await;

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

        let db_result = upload_item_image(req.item_id, path)
            .execute(self.pool.as_ref())
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

        let db_result = get_item(req.item_id)
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                rest_id: row.get("rest_id"),
                image: row.get("image"),
            })
            .fetch_one(self.pool.as_ref())
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

        let db_result = get_items(req.rest_id, req.category)
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                rest_id: row.get("rest_id"),
                image: row.get("image"),
            })
            .fetch_all(self.pool.as_ref())
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

        let db_result =
            insert_restaurant(rest_id.to_string(), req.name, req.description, req.owner_id)
                .execute(self.pool.as_ref())
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

        let db_result = delete_restaurant(req.rest_id)
            .execute(self.pool.as_ref())
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

        let db_result = upload_restaurant_logo(req.rest_id, path)
            .execute(self.pool.as_ref())
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

        let db_result = get_restaurant(req.rest_id)
            .map(|row| Restaurant {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                owner_id: row.get("owner_id"),
                logo: row.get("logo"),
            })
            .fetch_one(self.pool.as_ref())
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

        let db_result = get_restaurants(req.owner_id)
            .map(|row| Restaurant {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                owner_id: row.get("owner_id"),
                logo: row.get("logo"),
            })
            .fetch_all(self.pool.as_ref())
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
