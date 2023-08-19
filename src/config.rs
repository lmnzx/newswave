use config::{Config, ConfigError, Environment, File};

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::env;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
            .log_statements(tracing::log::LevelFilter::Trace)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedisStore {
    host: String,
    port: u16,
}

impl RedisStore {
    pub fn connection_string(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct SmtpServer {
    pub host: String,
    pub username: String,
    pub key: String,
    pub from_email: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis: RedisStore,
    pub smtp: SmtpServer,
}

impl Settings {
    pub fn get_config() -> Result<Self, ConfigError> {
        let mode = env::var("MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/base.yaml"))
            .add_source(File::with_name(&format!("config/{}", mode)).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        tracing::info!("Running in {} mode", mode);

        s.try_deserialize()
    }
}
