use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::ShortyContext;

#[derive(Serialize, Deserialize)]
struct RedirectCreatedResponse {
    slug: String,
}

#[actix_web::post("/add/{url}")]
pub async fn post(ctx: Data<ShortyContext>, url: Path<String>) -> HttpResponse {
    let slug_option = ctx.redirect_repo.create(url.into_inner()).await;
    return match slug_option {
        None => HttpResponse::BadRequest().finish(),
        Some(slug) => HttpResponse::Ok().json(RedirectCreatedResponse { slug }),
    };
}
