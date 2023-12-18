use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};
use day1::day1;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day18::day18;
use day4::day4;
use day6::day6;
use day7::day7;
use day8::day8;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod orders;

mod day1;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day4;
mod day6;
mod day7;
mod day8;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn die() -> HttpResponse {
    HttpResponse::InternalServerError().body("die lmao")
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(pool));
        cfg.service(hello_world);
        cfg.service(die);
        day1(cfg);
        day4(cfg);
        day6(cfg);
        day7(cfg);
        day8(cfg);
        day11(cfg);
        day12(cfg);
        day13(cfg);
        day14(cfg);
        day15(cfg);
        day18(cfg);
    };

    Ok(config.into())
}
