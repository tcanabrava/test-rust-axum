use zero2prod::run;
use std::{net::TcpListener, thread::spawn};

fn spanw_app() -> String {
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
    let addr = spanw_app();

    let client = reqwest::Client::new();
    let response = client.get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute command");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}