#[derive(sqlx::FromRow, Debug)]
pub struct Redirect {
    id: i32,
    slug: String,
    url: String,
}
