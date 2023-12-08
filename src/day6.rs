use actix_web::{post, web};
use fancy_regex::Regex;
use serde::Serialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?<!elf on a) shelf").unwrap();
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
    let no_shelf = match RE.captures(&text) {
        Ok(Some(c)) => c.len(),
        _ => 0
    };

    web::Json(ElfCount { 
        elf: text.rmatches("elf").count(), 
        shelf: text.rmatches("elf on a shelf").count(), 
        no_shelf
    })
}