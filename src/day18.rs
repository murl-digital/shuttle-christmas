use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Result,
};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, QueryBuilder};

use crate::orders::{save_orders, Order};

#[derive(Deserialize)]
struct Region {
    id: i32,
    name: String,
}

#[post("/18/reset")]
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
        DROP TABLE IF EXISTS regions;
    "#
    )
    .execute(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    sqlx::query!(
        r#"
        CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        );
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

#[post("/18/orders")]
async fn add_orders(
    pool: web::Data<PgPool>,
    orders: web::Json<Vec<Order>>,
) -> Result<HttpResponse> {
    save_orders(orders.into_inner(), &pool).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/18/regions")]
async fn add_regions(
    pool: web::Data<PgPool>,
    regions: web::Json<Vec<Region>>,
) -> Result<HttpResponse> {
    if regions.is_empty() {
        return Ok(HttpResponse::Ok().finish());
    }

    let mut query_builder = QueryBuilder::new("INSERT INTO regions (id, name)");

    query_builder.push_values(regions.0, |mut b, order| {
        b.push_bind(order.id).push_bind(order.name);
    });

    let query = query_builder.build();
    query
        .execute(pool.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/18/regions/total")]
async fn total(pool: web::Data<PgPool>) -> Result<web::Json<Vec<serde_json::Value>>> {
    let regions = sqlx::query!("SELECT SUM(quantity) as quantity, name FROM orders INNER JOIN public.regions r on r.id = orders.region_id GROUP BY r.name ORDER BY r.name")
        .fetch_all(pool.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(web::Json(
        regions
            .iter()
            .map(|r| json!({"region": r.name, "total": r.quantity}))
            .collect(),
    ))
}

#[get("/18/regions/top_list/{amt}")]
async fn top(
    pool: web::Data<PgPool>,
    amt: web::Path<i64>,
) -> Result<web::Json<Vec<serde_json::Value>>> {
    let rows = sqlx::query!("SELECT name, gift_name FROM (
        SELECT name, gift_name, SUM(quantity) as qty, row_number() OVER (PARTITION BY regions.name order by regions.name ASC, SUM(quantity) DESC, gift_name ASC) as row_num
        FROM regions
            LEFT JOIN orders o on regions.id = o.region_id
        GROUP BY regions.name, o.gift_name
        ORDER BY name, SUM(o.quantity) DESC, gift_name
    ) as deep
    WHERE row_num <= $1
    GROUP BY name, qty, gift_name
    ORDER BY name, qty DESC, gift_name", *amt + 1)
        .fetch_all(pool.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    // spot the silly workaround
    Ok(web::Json(rows.iter()
    .group_by(|r| r.name.as_ref().unwrap())
    .into_iter()
    .map(|(region, group)| json!({"region": region, "top_gifts": group.filter_map(|r| r.gift_name.clone()).take(*amt as usize).collect::<Vec<String>>()})).collect()))
}

pub fn day18(cfg: &mut ServiceConfig) {
    cfg.service(reset);
    cfg.service(add_orders);
    cfg.service(add_regions);
    cfg.service(total);
    cfg.service(top);
}
