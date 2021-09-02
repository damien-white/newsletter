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
    pub dbname: String,
}

impl DatabaseSettings {
    /// Sets the `url` of the database using the `DATABASE_URL` environment
    /// variable, falling back to the `.yaml` configuration file if not found.
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }

    pub fn test_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/",
            self.username, self.password, self.host, self.port
        )
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
