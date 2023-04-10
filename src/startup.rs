use axum::{
    routing::{get, post, IntoMakeService},
    Router,
    body::Body
};

use hyper::server::conn::AddrIncoming;

use std::net::TcpListener;

use crate::{
    routes::{health_check, subscribe},
    state::AppState,
};

use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use http::Request;

pub fn run(
    listener: TcpListener,
    state: AppState,
) -> axum::Server<AddrIncoming, IntoMakeService<Router>> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".into());

        tracing::error_span!(
            "request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .with_state(state)
        .layer(trace_layer)
        .layer(RequestIdLayer);

    let server = axum::Server::from_tcp(listener)
        .expect("Could not bind to TcpListener")
        .serve(app.into_make_service());

    server
}
