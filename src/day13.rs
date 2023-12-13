use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, QueryBuilder};

#[derive(Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[get("/13/sql")]
async fn sql(pool: web::Data<PgPool>) -> String {
    let sql = sqlx::query!("SELECT 20231213 as output")
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    sql.output.unwrap().to_string()
}

#[post("/13/reset")]
async fn reset(pool: web::Data<PgPool>) -> HttpResponse {
    sqlx::query!(
        r#"
        DROP TABLE IF EXISTS orders;
    "#
    )
    .execute(pool.get_ref())
    .await
    .unwrap();
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
    .unwrap();

    HttpResponse::Ok().finish()
}

#[post("/13/orders")]
async fn add_orders(pool: web::Data<PgPool>, orders: web::Json<Vec<Order>>) -> HttpResponse {
    let mut query_builder =
        QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity)");

    query_builder.push_values(orders.0, |mut b, order| {
        b.push_bind(order.id)
            .push_bind(order.region_id)
            .push_bind(order.gift_name)
            .push_bind(order.quantity);
    });

    let query = query_builder.build();
    query.execute(pool.as_ref()).await.unwrap();

    HttpResponse::Ok().finish()
}

#[get("/13/orders/total")]
async fn total(pool: web::Data<PgPool>) -> web::Json<serde_json::Value> {
    let total = sqlx::query!("SELECT SUM(quantity) as total FROM orders")
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    web::Json(json!({
        "total": total.total.unwrap()
    }))
}

#[get("/13/orders/popular")]
async fn ayo(pool: web::Data<PgPool>) -> web::Json<serde_json::Value> {
    let orders = sqlx::query!("SELECT SUM(quantity) as quantity, gift_name FROM orders GROUP BY gift_name ORDER BY quantity DESC LIMIT 2").fetch_all(pool.as_ref()).await.unwrap();

    if orders.first().is_some_and(|f| {
        orders.get(1).is_some_and(|s| {
            f.quantity
                .is_some_and(|fq| s.quantity.is_some_and(|sq| fq == sq))
        })
    }) {
        web::Json(json!({
            "popular": Option::<String>::None
        }))
    } else {
        web::Json(json!({
            "popular": orders.first().and_then(|o| o.gift_name.clone())
        }))
    }
}

pub fn day13(cfg: &mut ServiceConfig) {
    cfg.service(sql);
    cfg.service(reset);
    cfg.service(add_orders);
    cfg.service(total);
    cfg.service(ayo);
}
