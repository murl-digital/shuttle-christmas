use actix_web::{get, web::{self, ServiceConfig}};
use rustemon::client::RustemonClient;

// (2.0f64 * 9.825f64 * 10.0f64).sqrt()
const ROOT_2GH: f64 = 14.0178457689;


#[get("/8/weight/{idx}")]
pub async fn vaporeon_breeding(client: web::Data<RustemonClient>, idx: web::Path<i64>) -> String {
    let mon = rustemon::pokemon::pokemon::get_by_id(*idx, &client).await.unwrap();

    (mon.weight as f64 / 10_f64).to_string()
}

#[get("/8/drop/{idx}")]
pub async fn vaporeon_splat(client: web::Data<RustemonClient>, idx: web::Path<i64>) -> String {
    let mon = rustemon::pokemon::pokemon::get_by_id(*idx, &client).await.unwrap();

    let weight = mon.weight as f64 / 10_f64;

    (ROOT_2GH * weight).to_string()
}

pub fn day8(cfg: &mut ServiceConfig) {
    cfg.service(vaporeon_breeding);
    cfg.service(vaporeon_splat);
    cfg.app_data(web::Data::new(RustemonClient::default()));
}