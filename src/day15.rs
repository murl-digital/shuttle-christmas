use actix_web::{web::{ServiceConfig, self}, HttpResponse, post, HttpResponseBuilder};
use lazy_static::lazy_static;
use onig::Regex;
use serde::Deserialize;

lazy_static! {
    static ref VOWEL_REGEX: Regex = Regex::new(r"[aeiouyAEIOUY]").unwrap();
    static ref DOUBLE_REGEX: Regex = Regex::new(r"([a-zA-Z])\1").unwrap();
    static ref PHRASE_REGEX: Regex = Regex::new("(ab|cd|pq|xy)").unwrap();
}

#[derive(Deserialize)]
struct Input {
    input: String
}

#[post("/15/nice")]
async fn vibecheck(input: web::Json<Input>) -> HttpResponse {
    let vowel_check = VOWEL_REGEX.find_iter(&input.input).count() >= 3;
    let doule_check = DOUBLE_REGEX.captures_iter(&input.input).next().is_some();
    let phrase_check = PHRASE_REGEX.find(&input.input).is_none();
    if vowel_check && doule_check && phrase_check {
        HttpResponse::Ok().json(r#"{"result":"nice"}"#)
    } else {
        HttpResponse::BadRequest().json(r#"{"result":"naughty"}"#)
    }
}

#[post("/15/game")]
async fn vibecheck_thegame(input: web::Json<Input>) -> HttpResponse {
    //rule 1
    if input.input.len() < 8 {
        return HttpResponse::BadRequest().json(r#"{"result":"naughty","reason":"more types of chars"}"#);
    };

    HttpResponse::Ok().json(r#"{"result":"nice","reason":"that's a nice password"}"#)
}

trait Jsonify {
    fn json(&mut self, json: &'static str) -> HttpResponse;
}

impl Jsonify for HttpResponseBuilder {
    fn json(&mut self, json: &'static str) -> HttpResponse {
        self.content_type(mime::APPLICATION_JSON).body(json)
    }
}

pub fn day15(cfg: &mut ServiceConfig) {
    cfg.service(vibecheck);
}