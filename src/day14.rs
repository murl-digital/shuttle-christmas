use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Result, error::ErrorInternalServerError
};
use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "safe.html")]
struct Day14Input {
    content: String,
}

#[post("/14/unsafe")]
async fn yuck(input: web::Json<Day14Input>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(mime::TEXT_HTML_UTF_8)
        .body(format!(
            r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
            input.content
        ))
}

#[post("/14/safe")]
async fn not_yuck(input: web::Json<Day14Input>) -> Result<HttpResponse> {
    let rendered_text = input.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
    .content_type(mime::TEXT_HTML_UTF_8)
    .body(rendered_text))
}

pub fn day14(cfg: &mut ServiceConfig) {
    cfg.service(yuck);
    cfg.service(not_yuck);
}
