use std::collections::HashMap;

use actix_web::{get, HttpRequest, web};
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RecipeInput {
    recipe: HashMap<String, i32>,
    pantry: HashMap<String, i32>
}

#[derive(Serialize)]
struct RecipeOutput {
    cookies: i32,
    pantry: HashMap<String, i32>
}

#[get("/7/decode")]
pub async fn cookies(req: HttpRequest) -> String {
    let cookie = req.cookie("recipe").unwrap();
    decode(cookie.value())
}

#[get("/7/bake")]
pub async fn weed(req: HttpRequest) -> web::Json<RecipeOutput> {
    let cookie = req.cookie("recipe").unwrap();
    let decoded = decode(cookie.value());
    let mut input: RecipeInput = serde_json::from_str(&decoded).unwrap();

    let mut possible_amts = Vec::new();
    
    for (item, qty) in input.recipe.iter() {
        match input.pantry.get(item) {
           Some(p_qty) => possible_amts.push(p_qty / qty),
           None => return web::Json(RecipeOutput {
                cookies: 0,
                pantry: input.pantry
           })
        }
    }
    
    possible_amts.sort();
    let baked_cookies = possible_amts[0];

    for (item, qty) in input.recipe.iter() {
        let pantry_amt = input.pantry.get(item).unwrap();
        input.pantry.insert(item.clone(), pantry_amt - (qty * baked_cookies));
    }

    web::Json(RecipeOutput { cookies: baked_cookies, pantry: input.pantry })
}

fn decode(input: &str) -> String {
    let decoded = base64::engine::general_purpose::STANDARD.decode(input).unwrap();
    String::from_utf8(decoded).unwrap()
}