use std::{str::FromStr, sync::Arc};

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use crate::model::MIGRATOR;

pub struct Env {
    inner: Arc<Inner>,
}

impl Clone for Env {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);
        Self { inner }
    }
}

impl std::ops::Deref for Env {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Inner {
    pub pool: SqlitePool,
}

impl Env {
    pub async fn new(sqlite_url: &str) -> sqlx::Result<Self> {
        let opts = SqliteConnectOptions::from_str(sqlite_url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(opts).await?;
        MIGRATOR.run(&pool).await?;

        let inner = Inner { pool };
        let inner = Arc::new(inner);

        Ok(Self { inner })
    }
}
