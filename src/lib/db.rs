use crate::config::Config;
use sqlx::postgres::PgArguments;
use sqlx::postgres::PgPoolOptions;
use sqlx::query::Query;
use sqlx::{query, Error, Pool, Postgres};
use std::sync::Arc;

pub async fn create_pool() -> Result<Arc<Pool<Postgres>>, Error> {
    let config = Config::new();
    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(config.db_uri.as_str())
        .await?;
    Ok(Arc::new(pool))
}

pub fn get_owner_by_id(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM owners WHERE id = $1").bind(id);
}

pub fn get_owner_by_email(email: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM owners WHERE email = $1").bind(email);
}

pub fn insert_owner(
    id: String,
    email: String,
    hashed_password: String,
) -> Query<'static, Postgres, PgArguments> {
    return query("INSERT INTO owners VALUES ($1, $2, $3)")
        .bind(id)
        .bind(email)
        .bind(hashed_password);
}

pub fn get_restaurant(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM restaurants WHERE id = $1").bind(id);
}

pub fn get_restaurants(owner_id: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM restaurants WHERE owner_id = $1").bind(owner_id);
}

pub fn insert_restaurant(
    id: String,
    name: String,
    description: String,
    owner_id: String,
) -> Query<'static, Postgres, PgArguments> {
    return query("INSERT INTO restaurants VALUES ($1, $2, $3, $4)")
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(owner_id);
}

pub fn delete_restaurant(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("DELETE FROM restaurants WHERE id = $1").bind(id);
}

pub fn upload_restaurant_logo(id: String, path: String) -> Query<'static, Postgres, PgArguments> {
    return query("UPDATE restaurants SET image = $1 WHERE id = $2")
        .bind(path)
        .bind(id);
}

pub fn get_item(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM items WHERE id = $1").bind(id);
}

pub fn get_items(rest_id: String, category: String) -> Query<'static, Postgres, PgArguments> {
    return query("SELECT * FROM items WHERE rest_id = $1 AND ($2 = '' OR category = $2)")
        .bind(rest_id)
        .bind(category);
}

pub fn insert_item(
    id: String,
    name: String,
    description: String,
    category: String,
    rest_id: String,
) -> Query<'static, Postgres, PgArguments> {
    return query("INSERT INTO items VALUES ($1, $2, $3, $4, $5)")
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(category)
        .bind(rest_id);
}

pub fn delete_item(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("DELETE FROM items WHERE id = $1").bind(id);
}

pub fn upload_item_image(id: String, path: String) -> Query<'static, Postgres, PgArguments> {
    return query("UPDATE items SET image = $1 WHERE id = $2")
        .bind(path)
        .bind(id);
}

pub fn get_orders(rest_id: String, since: f32) -> Query<'static, Postgres, PgArguments> {
    return query(
        "
        SELECT o.id, i.name, o.requested_at, o.completed, o.table_number, o.description 
        FROM orders as o JOIN items as i ON o.item_id = i.id 
        WHERE i.rest_id = $1 AND o.requested_at >= $2
        ",
    )
    .bind(rest_id)
    .bind(since);
}

pub fn insert_order(
    id: String,
    item_id: String,
    requested_at: f32,
    completed: bool,
    table_number: i32,
    description: String,
) -> Query<'static, Postgres, PgArguments> {
    return query("INSERT INTO orders VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(id)
        .bind(item_id)
        .bind(requested_at)
        .bind(completed)
        .bind(table_number)
        .bind(description);
}

pub fn complete_order(id: String) -> Query<'static, Postgres, PgArguments> {
    return query("UPDATE orders SET completed = TRUE WHERE id = $1").bind(id);
}
