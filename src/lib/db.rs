use sqlx::postgres::PgArguments;
use sqlx::postgres::PgPoolOptions;
use sqlx::query::Query;
use sqlx::{query, Error, Pool, Postgres};
use std::sync::Arc;

pub async fn create_pool(db_uri: String, db_pool_size: u32) -> Result<Arc<Pool<Postgres>>, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(db_pool_size)
        .connect(db_uri.as_str())
        .await?;
    Ok(Arc::new(pool))
}

pub fn get_owner_by_id<'q>(id: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM owners WHERE id = $1").bind(id);
}

pub fn get_owner_by_email<'q>(email: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM owners WHERE email = $1").bind(email);
}

pub fn insert_owner<'q>(
    id: &'q String,
    email: &'q String,
    hashed_password: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query("INSERT INTO owners VALUES ($1, $2, $3)")
        .bind(id)
        .bind(email)
        .bind(hashed_password);
}

pub fn get_restaurant<'q>(id: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM restaurants WHERE id = $1").bind(id);
}

pub fn get_restaurants<'q>(owner_id: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM restaurants WHERE owner_id = $1").bind(owner_id);
}

pub fn insert_restaurant<'q>(
    id: &'q String,
    name: &'q String,
    description: &'q String,
    logo: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query("INSERT INTO restaurants VALUES ($1, $2, $3, $4, $5)")
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(logo)
        .bind(owner_id);
}

pub fn delete_restaurant<'q>(
    id: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query("DELETE FROM restaurants WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id);
}

pub fn update_restaurant<'q>(
    id: &'q String,
    name: &'q String,
    description: &'q String,
    logo: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        UPDATE restaurants 
        SET name = $1, description = $2, image = $3 
        WHERE id = $4 AND owner_id = $5
        ",
    )
    .bind(name)
    .bind(description)
    .bind(logo)
    .bind(id)
    .bind(owner_id);
}

pub fn get_item<'q>(id: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM items WHERE id = $1").bind(id);
}

pub fn get_items<'q>(
    rest_id: &'q String,
    category: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query("SELECT * FROM items WHERE rest_id = $1 AND ($2 = '' OR category = $2)")
        .bind(rest_id)
        .bind(category);
}

pub fn insert_item<'q>(
    id: &'q String,
    name: &'q String,
    price: &'q f32,
    description: &'q String,
    category: &'q String,
    image: &'q String,
    rest_id: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        IF EXISTS (SELECT 1 FROM restaurants WHERE id = $6 AND owner_id = $7) 
        INSERT INTO items VALUES ($1, $2, $3, $4, $5, $6, $7)
        ",
    )
    .bind(id)
    .bind(name)
    .bind(price)
    .bind(description)
    .bind(category)
    .bind(image)
    .bind(rest_id)
    .bind(owner_id);
}

pub fn delete_item<'q>(id: &'q String, owner_id: &'q String) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        DELETE i FROM items i 
        JOIN restaurants r ON i.rest_id = r.id 
        WHERE i.id = $1 AND r.owner_id = $2
        ",
    )
    .bind(id)
    .bind(owner_id);
}

pub fn update_item<'q>(
    id: &'q String,
    name: &'q String,
    price: &'q f32,
    description: &'q String,
    category: &'q String,
    image: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        UPDATE i 
        SET i.name = $1, i.price = $2, i.description = $3, i.category = $4, i.image = $5 
        FROM items i JOIN restaurants r ON i.rest_id = r.id 
        WHERE i.id = $6 AND r.owner_id = $7
        ",
    )
    .bind(name)
    .bind(price)
    .bind(description)
    .bind(category)
    .bind(image)
    .bind(id)
    .bind(owner_id);
}

pub fn get_orders<'q>(
    rest_id: &'q String,
    since: &'q f32,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        SELECT o.id, i.name, o.requested_at, o.completed, o.table_number, o.description 
        FROM orders o 
        JOIN items i ON o.item_id = i.id 
        JOIN restaurants r ON i.rest_id = r.id
        WHERE i.rest_id = $1 
        AND r.owner_id = $2 
        AND o.requested_at >= $3
        ",
    )
    .bind(rest_id)
    .bind(owner_id)
    .bind(since);
}

pub fn insert_order<'q>(
    id: &'q String,
    item_id: &'q String,
    requested_at: &'q f32,
    completed: &'q bool,
    table_number: &'q i32,
    description: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query("INSERT INTO orders VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(id)
        .bind(item_id)
        .bind(requested_at)
        .bind(completed)
        .bind(table_number)
        .bind(description);
}

pub fn complete_order<'q>(
    id: &'q String,
    owner_id: &'q String,
) -> Query<'q, Postgres, PgArguments> {
    return query(
        "
        UPDATE o SET o.completed = TRUE FROM orders o 
        JOIN items i ON o.item_id = i.id 
        JOIN restaurants r ON i.rest_id = r.id 
        WHERE o.id = $1 AND r.owner_id = $2
        ",
    )
    .bind(id)
    .bind(owner_id);
}
