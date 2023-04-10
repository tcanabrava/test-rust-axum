use crate::state::AppState;
use axum::extract::{Form, State};
use hyper::StatusCode;
use serde::Deserialize;
use tracing::Instrument;
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserSubscribe {
    user_name: String,
    email: String,
}

pub async fn subscribe(
    State(state): State<AppState>,
    Form(user): Form<UserSubscribe>,
) -> StatusCode {

    let request_id = Uuid::new_v4();
    let timestamp = Utc::now();
    let request_span = tracing::debug_span!("Adding a new subscriber", %request_id, timestamp = %timestamp);

    let res = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        request_id,
        user.email,
        user.user_name,
        timestamp
    )
    .execute(&state.db)
    .instrument(request_span)
    .await;

     match res {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::UNPROCESSABLE_ENTITY
        }
    }
}
