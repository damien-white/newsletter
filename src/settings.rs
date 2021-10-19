//! Manages the configuration for the service.
use std::convert::{TryFrom, TryInto};

use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    /// Sets the `url` of the database using the `DATABASE_URL` environment
    /// variable, falling back to the `.yaml` configuration file if not found.
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    pub fn test_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    /// Load the application settings from configuration sources
    pub fn load() -> Result<Settings, ConfigError> {
        let mut settings = Config::default();
        let base_path = std::env::current_dir().expect("Failed to find current working directory.");
        let config_dir = base_path.join("config");

        settings.merge(config::File::from(config_dir.join("default")).required(true))?;

        let environment: RuntimeEnv = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("APP_ENV is not set or could not be parsed.");

        settings.merge(config::File::from(config_dir.join(environment.as_str())).required(true))?;

        // Add settings from environment variables with a prefix of 'APP__'
        // Example: `APP_APPLICATION__PORT=8120` would set `Settings.app.port`
        settings.merge(config::Environment::new().separator("_"))?;
        // settings.merge(config::Environment::with_prefix("app").separator("_"))?;
        settings.try_into()
    }
}

/// Runtime environment types for the application.
pub enum RuntimeEnv {
    Local,
    Production,
}

impl RuntimeEnv {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuntimeEnv::Local => "local",
            RuntimeEnv::Production => "production",
        }
    }
}

impl TryFrom<String> for RuntimeEnv {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => Err("Environment type not supported. Please use 'local' or 'production'.".into()),
        }
    }
}
