use actix_web::{
    error::ErrorBadRequest,
    post,
    web::{self, ServiceConfig},
    Result,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Reindeer {
    strength: i32,
}

#[derive(Deserialize, Clone)]
struct ContestReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[post("/4/strength")]
async fn strength(reindeer: web::Json<Vec<Reindeer>>) -> String {
    reindeer.iter().map(|r| r.strength).sum::<i32>().to_string()
}

#[post("/4/contest")]
async fn contest(
    mut reindeer: web::Json<Vec<ContestReindeer>>,
) -> Result<web::Json<serde_json::Value>> {
    if reindeer.is_empty() {
        return Err(ErrorBadRequest(
            "cannot run a contest against an empty list",
        ));
    }

    reindeer.sort_by(|a, b| a.speed.total_cmp(&b.speed).reverse());
    let fastest = reindeer.first().expect("empty result?").clone();

    reindeer.sort_by(|a, b| a.height.cmp(&b.height).reverse());
    let tallest = reindeer.first().expect("empty result?").clone();

    reindeer.sort_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power).reverse());
    let magician = reindeer.first().expect("empty result?").clone();

    reindeer.sort_by(|a, b| {
        a.candies_eaten_yesterday
            .cmp(&b.candies_eaten_yesterday)
            .reverse()
    });
    let consumer = reindeer.first().expect("empty result?").clone();

    Ok(web::Json(json!({
        "fastest": format!(
            "Speeding past the finish line with a strength of {1} is {0}",
            fastest.name, fastest.strength
        ),
        "tallest": format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        "magician": format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        "consumer": format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })))
}

pub fn day4(cfg: &mut ServiceConfig) {
    cfg.service(strength);
    cfg.service(contest);
}
