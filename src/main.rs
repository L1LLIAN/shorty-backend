use std::env;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, App, HttpServer};
use dotenv::dotenv;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

use crate::models::context::ShortyContext;
use crate::repositories::redirect::RedirectRepository;

mod models;
mod repositories;
mod routes;

static MIGRATOR: Migrator = sqlx::migrate!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    let pg_uri =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/shorty".to_string());
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(1))
        .connect(pg_uri.as_str())
        .await
        .unwrap();

    // Just yeet the errors
    let _ = MIGRATOR.run(&pg_pool).await;

    let redirect_repo = RedirectRepository::new(pg_pool);
    let ctx = ShortyContext { redirect_repo };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000/") // frontend port
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(ctx.clone()))
            .service(routes::index_get)
            .service(routes::index_get_redirect)
            .service(routes::add_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
