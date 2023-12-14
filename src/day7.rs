use std::collections::HashMap;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get,
    web::{self, ServiceConfig},
    HttpRequest, Result,
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
async fn cookies(req: HttpRequest) -> Result<String> {
    let cookie = req
        .cookie("recipe")
        .ok_or(ErrorBadRequest("recipe cookie not present"))?;
    decode(cookie.value())
}

#[get("/7/bake")]
async fn weed(req: HttpRequest) -> Result<web::Json<serde_json::Value>> {
    let cookie = req
        .cookie("recipe")
        .ok_or(ErrorBadRequest("recipe cookie not present"))?;
    let decoded = decode(cookie.value())?;
    let mut input: Recipe = serde_json::from_str(&decoded)?;
    input.recipe.retain(|_, &mut v| v != 0);

    let mut possible_amts = Vec::new();

    for (item, qty) in input.recipe.iter() {
        match input.pantry.get(item) {
            Some(p_qty) => possible_amts.push(p_qty / qty),
            None => {
                return Ok(web::Json(json!({
                    "cookies": 0,
                    "pantry": input.pantry,
                })))
            }
        }
    }

    possible_amts.sort();
    let baked_cookies = possible_amts[0];

    for (item, qty) in input.recipe.iter() {
        let pantry_amt = input.pantry.get(item).ok_or(ErrorInternalServerError(
            "pantry items magically dissapeared, please report to the reality police",
        ))?;
        input
            .pantry
            .insert(item.clone(), pantry_amt - (qty * baked_cookies));
    }

    Ok(web::Json(json!({
        "cookies": baked_cookies,
        "pantry": input.pantry,
    })))
}

fn decode(input: &str) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(ErrorInternalServerError)?;
    String::from_utf8(decoded).map_err(ErrorInternalServerError)
}

pub fn day7(cfg: &mut ServiceConfig) {
    cfg.service(cookies);
    cfg.service(weed);
}
