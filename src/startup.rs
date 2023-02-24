use axum::{
    routing::{get, post, IntoMakeService},
    Router,
};

use hyper::server::conn::AddrIncoming;

use std::net::TcpListener;

use crate::{
    routes::{health_check, subscribe},
    state::AppState,
};

pub fn run(
    listener: TcpListener,
    state: AppState,
) -> axum::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .with_state(state);

    let server = axum::Server::from_tcp(listener)
        .expect("Could not bind to TcpListener")
        .serve(app.into_make_service());

    server
}
