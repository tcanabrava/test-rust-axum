use serde::Deserialize;
use secrecy::Secret;
use secrecy::ExposeSecret;

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub db_name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

pub enum Enviroment {
    Local,
    Production,
}

impl Enviroment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Enviroment::Local => "local",
            Enviroment::Production => "production"
        }
    }
}

impl TryFrom<String> for Enviroment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{other} is not a supported env. use local or production."))
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Can't acess current directory");

    let conf_dir =  base_path.join("configuration");
    let env: Enviroment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let env_file = format!("{env}.yaml", env=env.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(conf_dir.join("base.yaml")))
        .add_source(config::File::from(conf_dir.join(env_file)))
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
