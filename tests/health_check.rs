use zero2prod::startup::run;

use std::{net::TcpListener};
use sqlx::{PgConnection, Connection};

use zero2prod::configuration::get_config;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to find random port");

    let address = listener.local_addr()
        .expect("Failed to get local address");

    let server = run(listener);
    let _ = tokio::spawn(server);
    
    format!("http://127.0.0.1:{}", address.port())
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();
    let response = client.get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute command");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_valid_form_data() {
    let addr = spawn_app();
    let config = get_config()
        .expect("Failed to read config file");

    let connection_str = config.database.connection_string();

    let connection = PgConnection::connect(&connection_str)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let body = "user_name=le%20test&email=test@gmail.com";
    let response = client
        .post(format!("{}/subscribe", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_missing_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("user_name=tomaz", "Missing Email"),
        ("email=test@gmail.com", "Missing Name"),
        ("", "Missing Name and Email")
    ];

    for (body, error_msg) in test_cases {
        let response = client
        .post(format!("{}/subscribe", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

        assert_eq!(422, response.status().as_u16(), "{}", error_msg);
    }
}