use std::fmt::format;

use actix_web::web::Data;
use actix_web::Responder;

use crate::ShortyContext;

#[actix_web::post("/add")]
pub async fn post(ctx: Data<ShortyContext>) -> impl Responder {
    format!(
        "{}",
        ctx.redirect_repo
            .create(String::from("https://google.com"))
            .await
    )
}
