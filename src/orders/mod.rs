use actix_web::{error::ErrorInternalServerError, Result};
use serde::Deserialize;
use sqlx::{PgPool, QueryBuilder};

#[derive(Deserialize)]
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

pub async fn save_orders(orders: Vec<Order>, pool: &PgPool) -> Result<()> {
    if orders.is_empty() {
        return Ok(());
    }

    let mut query_builder =
        QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity)");

    query_builder.push_values(orders, |mut b, order| {
        b.push_bind(order.id)
            .push_bind(order.region_id)
            .push_bind(order.gift_name)
            .push_bind(order.quantity);
    });

    let query = query_builder.build();
    query
        .execute(pool)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(())
}
