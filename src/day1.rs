use std::str::FromStr;

use actix_web::{
    error::ErrorBadRequest,
    get,
    web::{self, ServiceConfig},
    Result,
};

#[get("/1/{nums:.*}")]
async fn packet_math(nums: web::Path<String>) -> Result<String> {
    let nums: Vec<i32> = nums
        .into_inner()
        .split('/')
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as FromStr>::Err>>()
        .map_err(ErrorBadRequest)?;

    Ok(nums.iter().fold(0, |acc, x| acc ^ x).pow(3).to_string())
}

pub fn day1(cfg: &mut ServiceConfig) {
    cfg.service(packet_math);
}
