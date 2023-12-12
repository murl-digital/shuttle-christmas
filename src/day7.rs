use std::collections::HashMap;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpRequest,
};
use base64::Engine;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Recipe {
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[get("/7/decode")]
async fn cookies(req: HttpRequest) -> String {
    let cookie = req.cookie("recipe").unwrap();
    decode(cookie.value())
}

#[get("/7/bake")]
async fn weed(req: HttpRequest) -> web::Json<serde_json::Value> {
    let cookie = req.cookie("recipe").unwrap();
    let decoded = decode(cookie.value());
    let mut input: Recipe = serde_json::from_str(&decoded).unwrap();
    input.recipe.retain(|_, &mut v| v != 0);

    let mut possible_amts = Vec::new();

    for (item, qty) in input.recipe.iter() {
        match input.pantry.get(item) {
            Some(p_qty) => possible_amts.push(p_qty / qty),
            None => {
                return web::Json(json!({
                    "cookies": 0,
                    "pantry": input.pantry,
                }))
            }
        }
    }

    possible_amts.sort();
    let baked_cookies = possible_amts[0];

    for (item, qty) in input.recipe.iter() {
        let pantry_amt = input.pantry.get(item).unwrap();
        input
            .pantry
            .insert(item.clone(), pantry_amt - (qty * baked_cookies));
    }

    web::Json(json!({
        "cookies": baked_cookies,
        "pantry": input.pantry,
    }))
}

fn decode(input: &str) -> String {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .unwrap();
    String::from_utf8(decoded).unwrap()
}

pub fn day7(cfg: &mut ServiceConfig) {
    cfg.service(cookies);
    cfg.service(weed);
}
