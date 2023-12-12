use actix_web::{get, web::{ServiceConfig, self}, HttpResponse};
use day1::packet_math;
use day11::{decoration, magic_goggles};
use day12::day12;
use day4::{strength, contest};
use day6::elf;
use day7::{cookies, weed};
use day8::{vaporeon_breeding, vaporeon_splat};
use rustemon::client::RustemonClient;
use shuttle_actix_web::ShuttleActixWeb;

mod day1;
mod day4;
mod day6;
mod day7;
mod day8;
mod day11;
mod day12;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn die() -> HttpResponse {
    HttpResponse::InternalServerError().body("die lmao")
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(die);
        cfg.service(packet_math);
        cfg.service(strength);
        cfg.service(contest);
        cfg.service(elf);
        cfg.service(cookies);
        cfg.service(weed);
        cfg.service(vaporeon_breeding);
        cfg.service(vaporeon_splat);
        cfg.service(decoration);
        cfg.service(magic_goggles);
        cfg.app_data(web::Data::new(RustemonClient::default()));
        day12(cfg);
    };

    Ok(config.into())
}
