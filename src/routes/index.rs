use actix_web::Responder;

#[actix_web::get("/")]
pub async fn get() -> impl Responder {
    "Hello tiny gay person"
}
