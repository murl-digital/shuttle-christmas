use actix_web::{web::{ServiceConfig, self}, HttpResponse, post};
use lazy_static::lazy_static;
use onig::Regex;
use serde::Deserialize;
use serde_json::json;
use unic_emoji::char::is_emoji;

lazy_static! {
    static ref VOWEL_REGEX: Regex = Regex::new(r"[aeiouyAEIOUY]").unwrap();
    static ref DOUBLE_REGEX: Regex = Regex::new(r"([a-zA-Z])\1").unwrap();
    static ref PHRASE_REGEX: Regex = Regex::new("(ab|cd|pq|xy)").unwrap();
    static ref UPPERCASE_REGEX: Regex = Regex::new("[A-Z]").unwrap();
    static ref LOWERCASE_REGEX: Regex = Regex::new("[a-z]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    static ref INTEGER_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
    static ref SANDWICH_REGEX: Regex = Regex::new(r"([a-zA-z])[a-zA-z]\1").unwrap();
    static ref JOY_REGEX: Regex = Regex::new(r"j.+o.+y").unwrap();
    static ref UNICODE_REGEX: Regex = Regex::new(r"[\u2980-\u2BFF]").unwrap();
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
        HttpResponse::Ok().json(json!({"result": "nice"}))
    } else {
        HttpResponse::BadRequest().json(json!({"result": "naughty"}))
    }
}

#[post("/15/game")]
async fn vibecheck_thegame(input: web::Json<Input>) -> HttpResponse {
    let input = &input.input;
    //rule 1
    if input.len() < 8 {
        return HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "8 chars"}));
    };
    //rule 2
    if UPPERCASE_REGEX.find(input).is_none() || LOWERCASE_REGEX.find(input).is_none() || DIGIT_REGEX.find(input).is_none() {
        return HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "more types of chars"}));
    };
    //rule 3
    if DIGIT_REGEX.captures_iter(input).count() < 5 {
        return HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "55555"}));
    }
    //rule 4
    if INTEGER_REGEX.captures_iter(input).map(|c| c.at(1).map(|s| s.parse::<i32>().ok()).unwrap_or_default().unwrap_or_default()).sum::<i32>() != 2023 {
        return HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "math is hard"}));
    }
    // rule 5
    if JOY_REGEX.find(input).is_none() || input.matches('j').count() != 1 || input.matches('o').count() != 1 || input.matches('y').count() != 1 {
        return HttpResponse::NotAcceptable().json(json!({"result": "naughty", "reason": "not joyful enough"}));
    }
    //rule 6
    if SANDWICH_REGEX.find(input).is_none() {
        return HttpResponse::UnavailableForLegalReasons().json(json!({"result": "naughty", "reason": "illegal: no sandwich"}));
    };
    //rule 7
    if UNICODE_REGEX.find(input).is_none() {
        return HttpResponse::RangeNotSatisfiable().json(json!({"result": "naughty", "reason": "outranged"}));
    }
    //rule 8
    if !input.chars().filter(|c| !c.is_alphanumeric()).any(is_emoji) {
        return HttpResponse::UpgradeRequired().json(json!({"result": "naughty", "reason": "😳"}));
    }
    //rule 9
    if !sha256::digest(input).ends_with('a') {
        return HttpResponse::ImATeapot().json(json!({"result": "naughty", "reason": "not a coffee brewer"}));
    }

    HttpResponse::Ok().json(json!({"result": "nice", "reason": "that's a nice password"}))
}

pub fn day15(cfg: &mut ServiceConfig) {
    cfg.service(vibecheck);
    cfg.service(vibecheck_thegame);
}