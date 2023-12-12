use std::{collections::HashMap, time::Instant};

use actix_web::{post, HttpResponse, web::{self, ServiceConfig}, get};
use chrono::{Utc, DateTime, Datelike};
use serde_json::json;
use tokio::sync::RwLock;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Default)]
struct Day12State {
    packets: RwLock<HashMap<String, Instant>>
}

#[post("/12/save/{id}")]
async fn save(data: web::Data<Day12State>, id: web::Path<String>) -> HttpResponse {
    let mut packets = data.packets.write().await;
    packets.insert(id.into_inner(), Instant::now());
    HttpResponse::Ok().finish()
}

#[get("/12/load/{id}")]
async fn load(data: web::Data<Day12State>, id: web::Path<String>) -> HttpResponse {
    let packets = data.packets.read().await;
    if let Some(instant) = packets.get(&id.into_inner()) {
        HttpResponse::Ok().body(instant.elapsed().as_secs().to_string())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/12/ulids")]
async fn ulids(ulids: web::Json<Vec<Ulid>>) -> web::Json<Vec<Uuid>> {
    let mut uuids: Vec<Uuid> = ulids.iter().map(|u| Uuid::from(*u)).collect();
    uuids.reverse();
    web::Json(uuids)
}

#[post("/12/ulids/{weekday}")]
async fn ulids_stats(other_ulids: web::Json<Vec<Ulid>>, weekday: web::Path<u32>) -> web::Json<serde_json::Value> {
    let christmas_eve = other_ulids.iter().filter(|u| {
        let utc: DateTime<Utc> = u.datetime().into();
        utc.month() == 12 && utc.day() == 24
    }).count();
    let weekday = other_ulids.iter().filter(|u| {
        let utc: DateTime<Utc> = u.datetime().into();
        utc.weekday().num_days_from_monday() == *weekday
    }).count();
    let future = other_ulids.iter().filter(|u| {
        let utc: DateTime<Utc> = u.datetime().into();
        utc > Utc::now()
    }).count();
    let broil = other_ulids.iter().filter(|u| {
        u.random() & 1 == 1
    }).count();

    web::Json(
        json!({
            "christmas eve": christmas_eve,
            "weekday": weekday,
            "in the future": future,
            "LSB is 1": broil
        })
    )
}

pub fn day12(cfg: &mut ServiceConfig) {
    cfg.app_data(web::Data::new(Day12State::default()));
    cfg.service(save);
    cfg.service(load);
    cfg.service(ulids);
    cfg.service(ulids_stats);
}