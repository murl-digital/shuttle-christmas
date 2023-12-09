use actix_web::{get, web};
use lazy_static::lazy_static;
use rustemon::client::RustemonClient;

// yes i could just put this in the equation directly, but if it wasnt for the SQRT THIS COULD JUST BE A CONST
// LIKE WTF RUST THIS WAS AN ISSUE IN 2019, IT IS 2023
// eat shit and die
lazy_static! {
    static ref ROOT_2GH: f64 = (2.0f64 * 9.825f64 * 10.0f64).sqrt();
}


#[get("/8/weight/{idx}")]
pub async fn vaporeon_breeding(client: web::Data<RustemonClient>, idx: web::Path<i64>) -> String {
    let mon = rustemon::pokemon::pokemon::get_by_id(*idx, &client).await.unwrap();

    (mon.weight / 10).to_string()
}

#[get("/8/drop/{idx}")]
pub async fn vaporeon_splat(client: web::Data<RustemonClient>, idx: web::Path<i64>) -> String {
    let mon = rustemon::pokemon::pokemon::get_by_id(*idx, &client).await.unwrap();

    let weight = (mon.weight / 10) as f64;

    (*ROOT_2GH * weight).to_string()
}