use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Result,
};
use serde_json::json;
use sqlx::PgPool;

use crate::orders::{save_orders, Order};

#[get("/13/sql")]
async fn sql(pool: web::Data<PgPool>) -> Result<String> {
    let sql = sqlx::query!("SELECT 20231213 as output")
        .fetch_one(pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(sql
        .output
        .ok_or(ErrorInternalServerError("no output"))?
        .to_string())
}

#[post("/13/reset")]
async fn reset(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    sqlx::query!(
        r#"
        DROP TABLE IF EXISTS orders;
    "#
    )
    .execute(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;
    sqlx::query!(
        r#"
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        );
    "#
    )
    .execute(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/13/orders")]
async fn add_orders(
    pool: web::Data<PgPool>,
    orders: web::Json<Vec<Order>>,
) -> Result<HttpResponse> {
    save_orders(orders.into_inner(), &pool).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/13/orders/total")]
async fn total(pool: web::Data<PgPool>) -> Result<web::Json<serde_json::Value>> {
    let total = sqlx::query!("SELECT SUM(quantity) as total FROM orders")
        .fetch_one(pool.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(web::Json(json!({
        "total": total.total.unwrap_or(0)
    })))
}

#[get("/13/orders/popular")]
async fn ayo(pool: web::Data<PgPool>) -> Result<web::Json<serde_json::Value>> {
    let orders = sqlx::query!("SELECT SUM(quantity) as quantity, gift_name FROM orders GROUP BY gift_name ORDER BY quantity DESC LIMIT 2").fetch_all(pool.as_ref()).await.map_err(ErrorInternalServerError)?;

    if orders.first().is_some_and(|f| {
        orders.get(1).is_some_and(|s| {
            f.quantity
                .is_some_and(|fq| s.quantity.is_some_and(|sq| fq == sq))
        })
    }) {
        Ok(web::Json(json!({
            "popular": Option::<String>::None
        })))
    } else {
        Ok(web::Json(json!({
            "popular": orders.first().and_then(|o| o.gift_name.clone())
        })))
    }
}

pub fn day13(cfg: &mut ServiceConfig) {
    cfg.service(sql);
    cfg.service(reset);
    cfg.service(add_orders);
    cfg.service(total);
    cfg.service(ayo);
}
