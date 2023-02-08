use store_proto::store_server::Store;
use store_proto::{
    AddItemRequest, AddItemResponse, CreateRestaurantRequest, CreateRestaurantResponse,
    DeleteRestaurantRequest, DeleteRestaurantResponse, GetItemsRequest, GetItemsResponse, Item,
    RemoveItemRequest, RemoveItemResponse,
};
use tonic::{Request, Response, Status};

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
        println!("Database: add {} to restaurant {}", req.name, req.rest_id);

        let res = AddItemResponse {
            item_id: "unique-item-id".to_owned(),
        };
        Ok(Response::new(res))
    }

    async fn remove_item(
        &self,
        request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>, Status> {
        let req = request.into_inner();
        println!(
            "Database: remove item {} from restaurant {}",
            req.item_id, req.rest_id
        );

        let res = RemoveItemResponse { success: true };
        Ok(Response::new(res))
    }

    async fn get_items(
        &self,
        request: Request<GetItemsRequest>,
    ) -> Result<Response<GetItemsResponse>, Status> {
        let req = request.into_inner();
        println!("Database: get items from restaurant {}", req.rest_id);
        let items = vec![Item {
            id: "unique-item-id".to_owned(),
            name: "Milk Tea".to_owned(),
            description: "Sweet and creamy tea.".to_owned(),
            category: "Tea".to_owned(),
            rest_id: "unique-restaurant-id".to_owned(),
        }];

        let res = GetItemsResponse { items: items };
        Ok(Response::new(res))
    }

    async fn create_restaurant(
        &self,
        request: Request<CreateRestaurantRequest>,
    ) -> Result<Response<CreateRestaurantResponse>, Status> {
        let req = request.into_inner();
        println!(
            "Database: create restaurant {} for owner {}",
            req.name, req.owner_id
        );

        let res = CreateRestaurantResponse {
            rest_id: "unqiue-restaurant-id".to_owned(),
        };
        Ok(Response::new(res))
    }

    async fn delete_restaurant(
        &self,
        request: Request<DeleteRestaurantRequest>,
    ) -> Result<Response<DeleteRestaurantResponse>, Status> {
        let req = request.into_inner();
        println!("Database: Delete restaurant {}", req.rest_id);

        let res = DeleteRestaurantResponse { success: true };
        Ok(Response::new(res))
    }
}
