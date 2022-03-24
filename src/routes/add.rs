use std::fmt::format;

use actix_web::web::{Data, Form, Path};
use actix_web::Responder;

use crate::ShortyContext;

#[actix_web::post("/add/{url}")]
pub async fn post(ctx: Data<ShortyContext>, url: Path<String>) -> impl Responder {
    format!("{}", ctx.redirect_repo.create(url.into_inner()).await)
}
