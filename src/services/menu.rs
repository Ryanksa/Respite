use lib::db::{delete_item, get_item, get_items, insert_item, update_item};
use menu_proto::menu_server::Menu;
use menu_proto::{
    AddItemRequest, AddItemResponse, GetItemRequest, GetItemResponse, GetItemsRequest,
    GetItemsResponse, Item, RemoveItemRequest, RemoveItemResponse, UpdateItemRequest,
    UpdateItemResponse,
};
use sqlx::{Pool, Postgres, Row};
use std::fs;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

pub mod menu_proto {
    tonic::include_proto!("menu");
}

pub struct MenuService {
    pool: Arc<Pool<Postgres>>,
    img_path: String,
}

impl MenuService {
    pub fn new(pool: Arc<Pool<Postgres>>, img_path: String) -> Self {
        MenuService {
            pool: pool,
            img_path: img_path,
        }
    }
}

#[tonic::async_trait]
impl Menu for MenuService {
    async fn add_item(
        &self,
        request: Request<AddItemRequest>,
    ) -> Result<Response<AddItemResponse>, Status> {
        let req = request.into_inner();
        let item_id = Uuid::new_v4();
        let image_path = format!("{}/{}", self.img_path, item_id.to_string());

        let db_result = insert_item(
            &item_id.to_string(),
            &req.name,
            &req.price,
            &req.description,
            &req.category,
            &image_path,
            &req.rest_id,
            &req.owner_id,
        )
        .execute(self.pool.as_ref())
        .await;

        let res = match db_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Status::new(Code::PermissionDenied, ""));
                }
                if let Err(err) = fs::write(image_path, req.image) {
                    log::error!("Menu Service: {}", err);
                    return Err(Status::new(Code::Internal, ""));
                };
                AddItemResponse {
                    item_id: item_id.to_string(),
                }
            }
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn remove_item(
        &self,
        request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>, Status> {
        let req = request.into_inner();
        let image_path = format!("{}/{}", self.img_path, req.item_id);

        let db_result = delete_item(&req.item_id, &req.owner_id)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Status::new(Code::NotFound, ""));
                }
                if let Err(err) = fs::remove_file(image_path) {
                    log::warn!("Menu Service: {}", err);
                }
                RemoveItemResponse { success: true }
            }
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn update_item(
        &self,
        request: Request<UpdateItemRequest>,
    ) -> Result<Response<UpdateItemResponse>, Status> {
        let req = request.into_inner();
        let image_path = format!("{}/{}", self.img_path, req.item_id);

        let db_result = update_item(
            &req.item_id,
            &req.name,
            &req.price,
            &req.description,
            &req.category,
            &image_path,
            &req.owner_id,
        )
        .execute(self.pool.as_ref())
        .await;

        let res = match db_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Status::new(Code::NotFound, ""));
                }
                if let Err(err) = fs::write(image_path, req.image) {
                    log::error!("Menu Service: {}", err);
                    return Err(Status::new(Code::Internal, ""));
                };
                UpdateItemResponse { success: true }
            }
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<GetItemResponse>, Status> {
        let req = request.into_inner();

        let db_result = get_item(&req.item_id)
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                price: row.get("price"),
                description: row.get("description"),
                category: row.get("category"),
                image: row.get("image"),
                rest_id: row.get("rest_id"),
            })
            .fetch_one(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(item) => {
                let image = match fs::read(&item.image) {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        log::error!("Menu Service: {}", err);
                        return Err(Status::new(Code::Internal, ""));
                    }
                };
                GetItemResponse {
                    item: Some(item),
                    image: image,
                }
            }
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn get_items(
        &self,
        request: Request<GetItemsRequest>,
    ) -> Result<Response<GetItemsResponse>, Status> {
        let req = request.into_inner();

        let db_result = get_items(&req.rest_id, &req.category)
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                price: row.get("price"),
                description: row.get("description"),
                category: row.get("category"),
                image: row.get("image"),
                rest_id: row.get("rest_id"),
            })
            .fetch_all(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(items) => GetItemsResponse { items: items },
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }
}
