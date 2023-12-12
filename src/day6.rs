use actix_web::{
    post,
    web::{self, ServiceConfig},
};
use lazy_static::lazy_static;
use onig::*;
use serde_json::json;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?<!elf on a) shelf").unwrap();
    static ref SHELF: Regex = Regex::new(r"^elf on a shelf").unwrap();
}

#[post("/6")]
async fn elf(text: String) -> web::Json<serde_json::Value> {
    let no_shelf = RE.find_iter(&text).count();

    web::Json(json!({
        "elf": text.matches("elf").count(),
        "elf on a shelf": text
            .char_indices()
            .filter_map(|(i, _)| SHELF.captures(&text[i..]))
            .count(),
        "shelf with no elf on it": no_shelf,
    }))
}

pub fn day6(cfg: &mut ServiceConfig) {
    cfg.service(elf);
}
