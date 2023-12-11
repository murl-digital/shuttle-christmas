use actix_web::{web, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Reindeer {
    strength: i32
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
    candies_eaten_yesterday: i32
}

#[derive(Serialize)]
struct Summary {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String
}

#[post("/4/strength")]
pub async fn strength(reindeer: web::Json<Vec<Reindeer>>) -> String {
    reindeer.iter().map(|r| r.strength).sum::<i32>().to_string()
}

#[post("/4/contest")]
pub async fn contest(reindeer: web::Json<Vec<ContestReindeer>>) -> web::Json<Summary> {
    let mut fastest_contenst = reindeer.clone();
    fastest_contenst.sort_by(|a, b| a.speed.total_cmp(&b.speed));
    fastest_contenst.reverse();
    let fastest = fastest_contenst.first().expect("empty result?");

    let mut tallest_contest = reindeer.clone();
    tallest_contest.sort_by(|a, b| a.height.cmp(&b.height));
    tallest_contest.reverse();
    let tallest = reindeer.first().expect("empty result?");

    let mut magician_contest = reindeer.clone();
    magician_contest.sort_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power));
    magician_contest.reverse();
    let magician = magician_contest.first().expect("empty result?");

    let mut cursed_contenst = reindeer.clone();
    cursed_contenst.sort_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday));
    cursed_contenst.reverse();
    let consumer = cursed_contenst.first().expect("empty result?");

    web::Json(Summary { 
        fastest: format!("Speeding past the finish line with a strength of {1} is {0}", fastest.name, fastest.strength), 
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width), 
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power), 
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food) 
    })
}