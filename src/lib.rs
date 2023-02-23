use axum::{
    routing::{get, IntoMakeService},
    Router,
    extract::Path,
};

use hyper::{StatusCode, server::conn::AddrIncoming};

use std::net::TcpListener;

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

pub fn run(listener: TcpListener) -> axum::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
    .route("/",  get(hello_world))
    .route("/greet/:user_name", get(greet))
    .route("/health_check", get(health_check));

    let server = axum::Server::from_tcp(listener)
        .expect("Could not bind to TcpListener")
        .serve(app.into_make_service());

    server
}