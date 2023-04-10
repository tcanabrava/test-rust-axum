use zero2prod::{startup::run, state::AppState};

use std::net::TcpListener;

use sqlx::{
    PgConnection,
    Connection,
    Executor,
    postgres::{PgPoolOptions, PgPool},
};

use zero2prod::{
    configuration::{
        get_config,
        DatabaseSettings,
    },
    telemetry
};

use once_cell::sync::Lazy;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let has_log = std::env::var("TEST_LOG").is_ok();

    if has_log {
        let subscriber = telemetry::log_subscriber(
            "unittest".into(),
            "debug".into(),
            std::io::stdout
        );
        telemetry::init_logging(subscriber);
    } else {
        let subscriber = telemetry::log_subscriber(
            "unittest".into(),
            "debug".into(),
            std::io::sink
        );
        telemetry::init_logging(subscriber);
    }
});

struct TestApp {
    pub address: String,
    pub state: AppState,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let mut config = get_config().expect("Failed to read config file");

    config.database.db_name = Uuid::new_v4().to_string();

    let pool = configure_db(&config.database).await;
    let state = AppState { db: pool };
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to find random port");
    let address = listener.local_addr().expect("Failed to get local address");
    let server = run(listener, state.clone());
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", address.port()),
        state: state
    }
}

pub async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let conn_str = config.connection_string_no_db();
    let mut conn = PgConnection::connect(&conn_str)
        .await.expect("Error creating connection");

    conn.execute(format!(r#"CREATE DATABASE "{}"; "#, config.db_name).as_str())
        .await
        .expect("Failed to create database");


    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_string())
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Error creating tables");

    pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute command");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_valid_form_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "user_name=le%20test&email=test@gmail.com";
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());

    let saved_form = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.state.db)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved_form.email, "test@gmail.com");
    assert_eq!(saved_form.name, "le test");
}

#[tokio::test]
async fn subscribe_returns_400_missing_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("user_name=tomaz", "Missing Email"),
        ("email=test@gmail.com", "Missing Name"),
        ("", "Missing Name and Email"),
    ];

    for (body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(422, response.status().as_u16(), "{}", error_msg);
    }
}
