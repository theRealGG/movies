use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub hostname: String,
    pub port: u16,
    pub reload: bool,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub hostname: String,
    pub port: u16,
    pub database: String,
    pub username: SecretString,
    pub password: SecretString,
}

impl DatabaseSettings {
    pub fn to_connection_options(&self) -> PgConnectOptions {
        PgConnectOptions::new_without_pgpass()
            .username(self.username.expose_secret())
            .password(self.password.expose_secret())
            .host(&self.hostname)
            .database(&self.database)
            .port(self.port)
    }
}
