#![allow(private_in_public)]

use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::validate_url;

use crate::ShortyContext;

#[derive(Serialize, Deserialize)]
struct RedirectCreatedResponse {
    slug: String,
}

#[derive(Deserialize)]
struct AddRedirectRequest {
    url: String,
}

#[actix_web::post("/add")]
pub async fn post(ctx: Data<ShortyContext>, req: Json<AddRedirectRequest>) -> HttpResponse {
    let url = req.url.clone();
    if !validate_url(url.clone()) {
        return HttpResponse::BadRequest().finish();
    }

    let slug_option = ctx.redirect_repo.create(url).await;
    return match slug_option {
        None => HttpResponse::BadRequest().finish(),
        Some(slug) => HttpResponse::Ok().json(RedirectCreatedResponse { slug }),
    };
}
