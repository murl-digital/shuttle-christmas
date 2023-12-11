use actix_files::NamedFile;
use actix_multipart::form::{MultipartForm, bytes::Bytes};
use actix_web::{get, Responder, post};
use image::GenericImageView;

#[derive(MultipartForm)]
struct FormData {
    image: Bytes
}


#[get("/11/assets/decoration.png")]
pub async fn decoration() -> impl Responder {
    NamedFile::open_async("assets/decoration.png").await
}

#[post("/11/red_pixels")]
pub async fn magic_goggles(form: MultipartForm<FormData>) -> String {
    let img = image::load_from_memory(&form.image.data).unwrap();

    // i have to do this cast otherwise the value overflows
    img.pixels().filter(|(_, _, pixel)| pixel.0[0] as i32 > pixel.0[1] as i32 + pixel.0[2] as i32).count().to_string()
}