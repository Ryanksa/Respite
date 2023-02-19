use lib::db::{
    delete_restaurant, get_restaurant, get_restaurants, insert_restaurant, upload_restaurant_logo,
};
use sqlx::{Pool, Postgres, Row};
use std::fs;
use std::sync::Arc;
use store_proto::store_server::Store;
use store_proto::{
    CreateRestaurantRequest, CreateRestaurantResponse, DeleteRestaurantRequest,
    DeleteRestaurantResponse, GetRestaurantRequest, GetRestaurantResponse, GetRestaurantsRequest,
    GetRestaurantsResponse, Restaurant, UploadRestaurantLogoRequest, UploadRestaurantLogoResponse,
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
    async fn create_restaurant(
        &self,
        request: Request<CreateRestaurantRequest>,
    ) -> Result<Response<CreateRestaurantResponse>, Status> {
        let req = request.into_inner();
        let rest_id = Uuid::new_v4();
        let path = format!("./img/{}", rest_id.to_string());

        if let Err(err) = fs::write(path.clone(), req.image) {
            log::error!("Store Service: {}", err);
            return Err(Status::new(Code::Internal, ""));
        };

        let db_result = insert_restaurant(
            rest_id.to_string(),
            req.name,
            req.description,
            path,
            req.owner_id,
        )
        .execute(self.pool.as_ref())
        .await;

        let res = match db_result {
            Ok(_) => CreateRestaurantResponse {
                rest_id: rest_id.to_string(),
            },
            Err(err) => {
                log::error!("Store Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn delete_restaurant(
        &self,
        request: Request<DeleteRestaurantRequest>,
    ) -> Result<Response<DeleteRestaurantResponse>, Status> {
        let req = request.into_inner();
        let logo_path = format!("./img/{}", req.rest_id.clone());

        let db_result = delete_restaurant(req.rest_id, req.owner_id)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Status::new(Code::NotFound, ""));
                }
                if let Err(err) = fs::remove_file(logo_path) {
                    log::warn!("Store Service: {}", err);
                }
                DeleteRestaurantResponse { success: true }
            }
            Err(err) => {
                log::error!("Store Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn upload_restaurant_logo(
        &self,
        request: Request<UploadRestaurantLogoRequest>,
    ) -> Result<Response<UploadRestaurantLogoResponse>, Status> {
        let req = request.into_inner();
        let path = format!("./img/{}", req.rest_id);

        let db_result = upload_restaurant_logo(req.rest_id, path.clone(), req.owner_id)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Status::new(Code::NotFound, ""));
                }
                if let Err(err) = fs::write(path, req.image) {
                    log::error!("Store Service: {}", err);
                    return Err(Status::new(Code::Internal, ""));
                };
                UploadRestaurantLogoResponse { success: true }
            }
            Err(err) => {
                log::error!("Store Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
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
                logo: row.get("logo"),
            })
            .fetch_one(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(restaurant) => GetRestaurantResponse {
                restaurant: Some(restaurant),
            },
            Err(err) => {
                log::error!("Store Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
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
                logo: row.get("logo"),
            })
            .fetch_all(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(restaurants) => GetRestaurantsResponse {
                restaurants: restaurants,
            },
            Err(err) => {
                log::error!("Store Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }
}
