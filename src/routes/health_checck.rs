use hyper::StatusCode;

pub async fn health_check() -> StatusCode {
    tracing::info!("Health Check!");
    StatusCode::OK
}
