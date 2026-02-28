use std::cell::RefCell;

use anyhow::Context;
use gpw::PasswordGenerator;
use libsql::Database;
use poem::web::RealIp;
use rand::{RngExt, distr::Alphanumeric, rng};
use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub async fn migrate(db: &Database) -> anyhow::Result<()> {
    static MIGRATIONS: [&str; 1] = [include_str!("../migrations/000_initial.sql")];

    let conn = db.connect().context("unable to connect to database")?;

    for migration in &MIGRATIONS {
        conn.execute(migration, ())
            .await
            .context("failed to run migration")?;
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub content: String,
    #[serde(with = "time::serde::rfc3339")]
    pub added: OffsetDateTime,
    pub remote: String,
    pub highlight: Option<String>,
}

impl Post {
    pub async fn create(
        db: &Database,
        remote: &RealIp,
        content: String,
        highlight: Option<String>,
    ) -> anyhow::Result<String> {
        let slug = generate_slug();
        let conn = db.connect()?;

        conn.execute(
            "INSERT INTO posts (slug, content, added, remote, highlight) \
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                slug.as_str(),
                content,
                OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .context("failed to format datetime")?,
                remote.0.map(|ip| ToString::to_string(&ip)),
                highlight,
            ),
        )
        .await
        .context("failed to create post")?;

        Ok(slug)
    }

    pub async fn get(db: &Database, slug: &str) -> anyhow::Result<Option<Post>> {
        let conn = db.connect()?;

        let mut rows = conn
            .query(
                "SELECT id, slug, content, added, remote, highlight \
                FROM posts WHERE slug = ?1",
                [slug],
            )
            .await
            .context("failed to get post")?;

        let Some(row) = rows.next().await.context("failed to get post")? else {
            return Ok(None);
        };

        eprintln!("{:?}", row);

        Ok(Some(Post {
            id: row.get(0)?,
            slug: row.get(1)?,
            content: row.get(2)?,
            added: OffsetDateTime::parse(&row.get::<String>(3)?, &Rfc3339)?,
            remote: row.get(4)?,
            highlight: row.get(5)?,
        }))
    }
}

fn generate_slug() -> String {
    thread_local! {
        static KEYGEN: RefCell<PasswordGenerator> = RefCell::default();
    }

    KEYGEN
        .with(|keygen| keygen.borrow_mut().next())
        .unwrap_or_else(|| {
            rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect()
        })
}
