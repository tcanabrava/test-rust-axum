use zero2prod::{configuration::get_config, startup::run, state::AppState};
use sqlx::postgres::PgPoolOptions;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber)
        .expect("Failed to set global default");

    let config = match get_config() {
        Ok(config) => config,
        Err(err) => panic!("{}", err),
    };

    tracing::info!("Application starting!");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = std::net::TcpListener::bind(&address).expect("Could not bind to address");

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.connection_string())
        .await
        .expect("can't connect to database");

    let state = AppState { db: pool };

    let server = run(listener, state);
    server.await.unwrap();

    tracing::info!("Application finished");
}
