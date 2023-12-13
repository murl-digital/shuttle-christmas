use actix_files::NamedFile;
use actix_multipart::form::{bytes::Bytes, MultipartForm, MultipartFormConfig};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Responder,
};
use image::GenericImageView;

#[derive(MultipartForm)]
struct FormData {
    image: Bytes,
}

#[get("/11/assets/decoration.png")]
async fn decoration() -> impl Responder {
    NamedFile::open_async("assets/decoration.png").await
}

#[post("/11/red_pixels")]
async fn magic_goggles(form: MultipartForm<FormData>) -> String {
    let img = image::load_from_memory(&form.image.data).unwrap();

    // i have to do this cast otherwise the value overflows
    img.pixels()
        .filter(|(_, _, pixel)| pixel.0[0] as i32 > pixel.0[1] as i32 + pixel.0[2] as i32)
        .count()
        .to_string()
}

pub fn day11(cfg: &mut ServiceConfig) {
    cfg.service(decoration);
    cfg.service(magic_goggles);
    // you can solve this without upping the memory limit, i just chose to because i wanted to play with larger images
    cfg.app_data(web::Data::new(
        MultipartFormConfig::default().memory_limit(52_428_800),
    ));
}
