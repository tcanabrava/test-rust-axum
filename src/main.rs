use zero2prod::{configuration::get_config, telemetry, startup::run, state::AppState};
use sqlx::postgres::PgPoolOptions;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() {
    let subscriber = telemetry::log_subscriber(
        "Zero2Prod".into(),
        "info".into(),
        std::io::stdout
    );

    telemetry::init_logging(subscriber);

    let config = match get_config() {
        Ok(config) => config,
        Err(err) => panic!("{}", err),
    };

    tracing::info!("Application starting!");

    let address = format!("{host}:{port}", 
        host = config.application.host,
        port=config.application.port);

    let listener = std::net::TcpListener::bind(&address)
        .expect("Could not bind to address");

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.connection_string().expose_secret())
        .await
        .expect("can't connect to database");

    // create our state object that holds database pools
    // and extra data.
    let state = AppState { db: pool };

    // start and run the server.
    let server = run(listener, state);
    server.await.unwrap();

    tracing::info!("Application finished");
}

