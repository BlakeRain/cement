use std::cell::RefCell;

use gpw::PasswordGenerator;
use poem::web::RemoteAddr;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use sqlx::SqlitePool;
use time::OffsetDateTime;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub content: String,
    #[serde(with = "time::serde::rfc2822")]
    pub added: OffsetDateTime,
    pub remote: String,
}

impl Post {
    pub async fn create(
        pool: &SqlitePool,
        remote: &RemoteAddr,
        content: String,
    ) -> sqlx::Result<String> {
        let slug = generate_slug();
        sqlx::query(
            "INSERT INTO posts (slug, content, added, remote)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(&slug)
        .bind(content)
        .bind(OffsetDateTime::now_utc())
        .bind(remote.to_string())
        .execute(pool)
        .await?;
        Ok(slug)
    }

    pub async fn get(pool: &SqlitePool, slug: &str) -> sqlx::Result<Option<Post>> {
        sqlx::query_as("SELECT * FROM posts WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }
}

fn generate_slug() -> String {
    thread_local! {
        static KEYGEN: RefCell<PasswordGenerator> = RefCell::default();
    }

    KEYGEN
        .with(|keygen| keygen.borrow_mut().next())
        .unwrap_or_else(|| {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect()
        })
}
