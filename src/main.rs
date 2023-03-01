use zero2prod::{configuration::get_config, startup::run, state::AppState};

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();


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
