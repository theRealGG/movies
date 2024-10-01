use secrecy::SecretString;
use serde::Deserialize;

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
    pub username: SecretString,
    pub password: SecretString,
}
