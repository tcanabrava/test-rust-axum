use serde::Deserialize;
use secrecy::Secret;
use secrecy::ExposeSecret;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub db_name: String,
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration.yaml"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{user}:{pass}@{host}:{port}/{db_name}",
            user = self.username,
            pass = self.password.expose_secret(),
            host = self.host,
            port = self.port,
            db_name = self.db_name
        ))
    }

    pub fn connection_string_no_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{user}:{pass}@{host}:{port}/",
            user = self.username,
            pass = self.password.expose_secret(),
            host = self.host,
            port = self.port,
        ))
    }
}
