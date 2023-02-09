use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;
use tonic::{Code, Status};

#[allow(non_upper_case_globals)]
static mut pool_cache: Option<Arc<Pool<Postgres>>> = None;

async fn create_pool() -> Result<Arc<Pool<Postgres>>, Error> {
    let config = Config::new();
    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(config.db_uri.as_str())
        .await?;
    Ok(Arc::new(pool))
}

pub async fn get_pool() -> Result<Arc<Pool<Postgres>>, Error> {
    unsafe {
        if let Some(pool) = &pool_cache {
            return Ok(pool.clone());
        }
        let pool = create_pool().await?;
        pool_cache = Some(pool.clone());
        Ok(pool)
    }
}

pub async fn get_pool_grpc() -> Result<Arc<Pool<Postgres>>, Status> {
    let pool = match get_pool().await {
        Ok(pool) => pool,
        Err(err) => return Err(Status::new(Code::Internal, format!("{}", err))),
    };
    Ok(pool)
}
