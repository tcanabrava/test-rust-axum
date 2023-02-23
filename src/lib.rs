use axum::{
    routing::{get, post, IntoMakeService},
    Router,
    extract::{Form, Path},
};

use hyper::{StatusCode, server::conn::AddrIncoming};

use std::net::TcpListener;

use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello World\n"
}

async fn greet(Path(user_name): Path<String>) -> String {
    let res = format!("Hello {}\n", user_name);
    res
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize)]
struct UserSubscribe {
    user_name: String,
    email: String,
}

async fn subscribe(Form(user): Form<UserSubscribe>) -> StatusCode {
    println!("Test! {}, {}", user.user_name, user.email);
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> axum::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
    .route("/",  get(hello_world))
    .route("/greet/:user_name", get(greet))
    .route("/health_check", get(health_check))
    .route("/subscribe", post(subscribe));

    let server = axum::Server::from_tcp(listener)
        .expect("Could not bind to TcpListener")
        .serve(app.into_make_service());

    server
}

