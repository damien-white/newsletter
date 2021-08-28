//! Manages the configuration for the service.

use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    pub fn build_url(&self) -> String {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            url
        } else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.name
            )
        }
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn load() -> Result<Settings, ConfigError> {
        // initialize config reader
        let mut settings = Config::default();

        // Add config values from file in config directory named `default`.
        // It will search for any top-level file with an extension
        // that `config` knows how to parse: *.{json, toml, yaml} etc.
        settings.merge(config::File::with_name("config/default"))?;

        // Try to convert config values read into our `Settings` type
        settings.try_into()
    }
}
