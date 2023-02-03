mod config;
mod services;

use config::Config;
use services::store::store_proto::store_client::StoreClient;
use services::store::store_proto::{
    AddItemRequest, AddItemResponse, CreateRestaurantRequest, CreateRestaurantResponse,
    DeleteRestaurantRequest, DeleteRestaurantResponse, GetItemsRequest, GetItemsResponse,
    RemoveItemRequest, RemoveItemResponse,
};
use std::error::Error;
use tonic::{Request, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let mut client = StoreClient::connect(format!("https://{}", config.store_socket_addr)).await?;

    let request = Request::new(CreateRestaurantRequest {
        name: "ChaCha".to_owned(),
        description: "Come have some tea.".to_owned(),
    });
    let response = match client.create_restaurant(request).await {
        Ok(res) => res,
        Err(_) => Response::new(CreateRestaurantResponse {
            rest_id: "".to_owned(),
        }),
    };
    let rest_id = response.into_inner().rest_id;

    let request = Request::new(AddItemRequest {
        rest_id: rest_id.clone(),
        name: "Milk Tea".to_owned(),
        description: "Sweet and creamy tea.".to_owned(),
        category: "Tea".to_owned(),
    });
    let response = match client.add_item(request).await {
        Ok(res) => res,
        Err(_) => Response::new(AddItemResponse {
            item_id: "".to_owned(),
        }),
    };
    let item_id = response.into_inner().item_id;

    let request = Request::new(GetItemsRequest {
        rest_id: rest_id.clone(),
        category: "".to_owned(),
    });
    let response = match client.get_items(request).await {
        Ok(res) => res,
        Err(_) => Response::new(GetItemsResponse { items: vec![] }),
    };
    let items = response.into_inner().items;
    println!("Items: {:?}", items);

    if items.len() > 0 {
        let request = Request::new(RemoveItemRequest {
            rest_id: items[0].rest_id.clone(),
            item_id: items[0].id.clone(),
        });
        let response = match client.remove_item(request).await {
            Ok(res) => res,
            Err(_) => Response::new(RemoveItemResponse { success: false }),
        };
    }

    let request = Request::new(DeleteRestaurantRequest { rest_id: rest_id });
    let response = match client.delete_restaurant(request).await {
        Ok(res) => res,
        Err(_) => Response::new(DeleteRestaurantResponse { success: false }),
    };

    Ok(())
}
