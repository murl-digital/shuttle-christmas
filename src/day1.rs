use actix_web::{get, web::{self}, Result, error::ErrorBadRequest};

#[get("/1/{nums:.*}")]
pub async fn packet_math(nums: web::Path<String>) -> Result<String> {
    let nums: Vec<Result<i32, _>> = nums.into_inner().split('/').map(|s| s.parse::<i32>()).collect();

    let mut result = 0;
    for n in nums {
        match n {
            Ok(n) => {result ^= n}
            Err(e) => return Err(ErrorBadRequest(e))
        }
    }

    Ok(result.pow(3).to_string())
}