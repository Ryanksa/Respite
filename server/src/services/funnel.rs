use funnel_proto::funnel_server::Funnel;
use funnel_proto::{
    CompleteOrderRequest, CompleteOrderResponse, GetOrdersRequest, MakeOrderRequest,
    MakeOrderResponse, Order,
};
use lib::db::get_pool_grpc;
use sqlx::{query, Row};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

pub mod funnel_proto {
    tonic::include_proto!("funnel");
}

#[derive(Debug, Default)]
pub struct FunnelService {}

#[tonic::async_trait]
impl Funnel for FunnelService {
    async fn make_order(
        &self,
        request: Request<MakeOrderRequest>,
    ) -> Result<Response<MakeOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = Uuid::new_v4();
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n,
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };

        let pool = get_pool_grpc().await?;
        let db_result = query("INSERT INTO orders VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(order_id.to_string())
            .bind(req.item_id)
            .bind(now.as_secs() as i64)
            .bind(false)
            .bind(req.table_number)
            .bind(req.description)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => MakeOrderResponse {
                order_id: order_id.to_string(),
            },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
        };
        Ok(Response::new(res))
    }

    async fn complete_order(
        &self,
        request: Request<CompleteOrderRequest>,
    ) -> Result<Response<CompleteOrderResponse>, Status> {
        let req = request.into_inner();

        let pool = get_pool_grpc().await?;
        let db_result = query("UPDATE orders SET completed = TRUE WHERE id = $1")
            .bind(req.order_id)
            .execute(pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => CompleteOrderResponse { success: true },
            Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
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
        let pool = get_pool_grpc().await?;

        tokio::spawn(async move {
            let mut db_stream = query(
                "
                SELECT o.id, i.name, o.requested_at, o.completed, o.table_number, o.description 
                FROM orders as o JOIN items as i ON o.item_id = i.id 
                WHERE i.rest_id = $1 AND o.requested_at >= $2
                ",
            )
            .bind(req.rest_id)
            .bind(req.since)
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
                tx.send(Ok(order.clone())).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
