use lib::db::{complete_order, get_orders, insert_order};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;
use waiter_proto::waiter_server::Waiter;
use waiter_proto::*;

pub mod waiter_proto {
    tonic::include_proto!("waiter");
}

pub struct WaiterService {
    pool: Arc<Pool<Postgres>>,
}

impl WaiterService {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        WaiterService { pool: pool }
    }
}

#[tonic::async_trait]
impl Waiter for WaiterService {
    async fn make_order(
        &self,
        request: Request<MakeOrderRequest>,
    ) -> Result<Response<MakeOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = Uuid::new_v4();
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n,
            Err(err) => {
                log::error!("Waiter Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };

        let db_result = insert_order(
            &order_id.to_string(),
            &req.item_id,
            &(now.as_secs() as f32),
            &false,
            &req.table_number,
            &req.description,
        )
        .execute(self.pool.as_ref())
        .await;

        let res = match db_result {
            Ok(_) => MakeOrderResponse {
                order_id: order_id.to_string(),
            },
            Err(err) => {
                log::error!("Waiter Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn complete_order(
        &self,
        request: Request<CompleteOrderRequest>,
    ) -> Result<Response<CompleteOrderResponse>, Status> {
        let req = request.into_inner();

        let db_result = complete_order(&req.order_id, &req.owner_id)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => CompleteOrderResponse { success: true },
            Err(err) => {
                log::error!("Waiter Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    type getOrdersStream = ReceiverStream<Result<Order, Status>>;

    async fn get_orders(
        &self,
        request: Request<GetOrdersRequest>,
    ) -> Result<Response<Self::getOrdersStream>, Status> {
        let req = request.into_inner();
        let (tx, rx) = mpsc::channel(3);
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let mut db_stream = get_orders(&req.rest_id, &req.since, &req.owner_id)
                .map(|row| Order {
                    id: row.get("id"),
                    item_name: row.get("name"),
                    requested_at: row.get("requested_at"),
                    completed: row.get("completed"),
                    table_number: row.get("table_number"),
                    description: row.get("description"),
                })
                .fetch(pool.as_ref());

            while let Ok(Some(order)) = db_stream.try_next().await {
                if let Err(err) = tx.send(Ok(order)).await {
                    log::warn!("Waiter Service: {}", err);
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
