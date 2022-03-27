use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::{Pool, Postgres};

use crate::models::redirect::Redirect;

#[derive(Clone, Debug)]
pub struct RedirectRepository {
    pg: Pool<Postgres>,
}

impl RedirectRepository {
    pub fn new(pg: Pool<Postgres>) -> Self {
        Self { pg }
    }

    pub async fn get_redirect_by_slug(&self, slug: &str) -> Option<Redirect> {
        let result = sqlx::query_as::<_, Redirect>("SELECT * FROM redirects WHERE slug = $1")
            .bind(slug)
            .fetch_one(&self.pg)
            .await;

        // TODO: Error logging if error isn't non-existent row

        if result.is_ok() {
            Some(result.unwrap())
        } else {
            None
        }
    }

    pub async fn is_slug_in_use(&self, slug: &str) -> bool {
        let result = sqlx::query("SELECT * FROM redirects WHERE slug = $1")
            .bind(slug)
            .fetch_one(&self.pg)
            .await;

        // If result is ok then row does exist, concluding that the slug is in use
        result.is_ok()
    }

    pub async fn create(&self, url: &str) -> Option<String> {
        let mut slug = gen_slug();
        while self.is_slug_in_use(&slug).await {
            slug = gen_slug();
        }

        let result = sqlx::query("INSERT INTO redirects (slug, url) VALUES ($1, $2)")
            .bind(&slug)
            .bind(url)
            .execute(&self.pg)
            .await;

        return match result {
            Ok(_) => Some(slug),
            Err(e) => {
                println!("err = {:?}", e);
                None
            }
        };
    }
}

// From https://stackoverflow.com/a/54277357
fn gen_slug() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}
