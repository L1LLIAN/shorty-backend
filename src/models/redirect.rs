#[derive(sqlx::FromRow, Debug)]
pub struct Redirect {
    pub id: i32,
    pub slug: String,
    pub url: String,
}
