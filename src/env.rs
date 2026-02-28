use std::sync::Arc;

use anyhow::Context;
use libsql::{Builder, Database};

use crate::model::migrate;

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
    pub db: Database,
}

impl Env {
    pub async fn new(libsql_url: String, libsql_token: String) -> anyhow::Result<Self> {
        let db = Builder::new_remote(libsql_url, libsql_token)
            .build()
            .await
            .context("failed to connect to libSQL database")?;

        migrate(&db).await?;

        let inner = Inner { db };
        let inner = Arc::new(inner);

        Ok(Self { inner })
    }
}
