use lib::db::{delete_item, get_item, get_items, insert_item, upload_item_image};
use menu_proto::menu_server::Menu;
use menu_proto::{
    AddItemRequest, AddItemResponse, GetItemRequest, GetItemResponse, GetItemsRequest,
    GetItemsResponse, Item, RemoveItemRequest, RemoveItemResponse, UploadItemImageRequest,
    UploadItemImageResponse,
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
}

impl MenuService {
    #[allow(dead_code)]
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        MenuService { pool: pool }
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

        match get_item(req.item_id.clone())
            .fetch_one(self.pool.as_ref())
            .await
        {
            Ok(row) => {
                let image_path: String = row.get("image");
                if let Err(err) = fs::remove_file(image_path) {
                    log::warn!("Menu Service: {}", err);
                }
            }
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::NotFound, ""));
            }
        }

        let db_result = delete_item(req.item_id).execute(self.pool.as_ref()).await;

        let res = match db_result {
            Ok(_) => RemoveItemResponse { success: true },
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
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
            log::error!("Menu Service: {}", err);
            return Err(Status::new(Code::Internal, ""));
        };

        let db_result = upload_item_image(req.item_id, path)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => UploadItemImageResponse { success: true },
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
            Err(err) => {
                log::error!("Menu Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }
}