use axum:: {
    Router,
    routing::{get, post, IntoMakeService},

};

use hyper::{server::conn::AddrIncoming};

use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> axum::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
    .route("/health_check", get(health_check))
    .route("/subscribe", post(subscribe));

    let server = axum::Server::from_tcp(listener)
        .expect("Could not bind to TcpListener")
        .serve(app.into_make_service());

    server
}
