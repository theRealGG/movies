use std::borrow::Borrow;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::models::settings::DatabaseSettings;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(settings: DatabaseSettings) -> Self {
        let connection_options = settings.to_connection_options();
        Self {
            pool: PgPoolOptions::new().connect_lazy_with(connection_options),
        }
    }
}

impl Borrow<PgPool> for Database {
    fn borrow(&self) -> &PgPool {
        &self.pool
    }
}

impl AsRef<PgPool> for Database {
    fn as_ref(&self) -> &PgPool {
        &self.pool
    }
}
