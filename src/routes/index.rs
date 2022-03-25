use actix_web::web::{Data, Path};
use actix_web::Responder;

use crate::ShortyContext;

#[actix_web::get("/")]
pub async fn get() -> impl Responder {
    "Hello tiny gay person"
}

#[actix_web::get("/{slug}")]
pub async fn get_redirect(ctx: Data<ShortyContext>, slug: Path<String>) -> impl Responder {
    let slug = slug.into_inner();
    let redirect = ctx.redirect_repo.get_redirect_by_slug(&slug).await;
    return match redirect {
        Some(r) => r.url,
        None => format!("No redirect found for {}", slug),
    };
}
