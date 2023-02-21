
use axum::{
    routing::get,
    Router,
    extract::Path,
};

use hyper::StatusCode;

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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",  get(hello_world))
        .route("/greet/:user_name", get(greet))
        .route("/health_check", get(health_check));
    
    let addr: std::net::SocketAddr = "0.0.0.0:3000".parse().unwrap();

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    server.await.unwrap();
}
