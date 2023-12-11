use actix_web::{post, web};
use onig::*;
use serde::Serialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?<!elf on a) shelf").unwrap();
    static ref SHELF: Regex = Regex::new(r"^elf on a shelf").unwrap();
}

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    no_shelf: usize
}

#[post("/6")]
pub async fn elf(text: String) -> web::Json<ElfCount> {
    let no_shelf = RE.find_iter(&text).count();

    web::Json(ElfCount { 
        elf: text.matches("elf").count(), 
        shelf: text.char_indices().filter_map(|(i,_)| SHELF.captures(&text[i..])).count(), 
        no_shelf
    })
}