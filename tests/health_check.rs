use zero2prod::{startup::run, state::AppState};

use std::net::TcpListener;

use sqlx::postgres::{PgPoolOptions};

use zero2prod::configuration::get_config;

struct TestApp {
    pub address: String,
    pub state: AppState,
}

async fn spawn_app() -> TestApp {
    let config = get_config().expect("Failed to read config file");

    let connection_str = config.database.connection_string();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await
        .expect("can't connect to database");

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
