use crate::state::AppState;
use axum::extract::{Form, State};
use hyper::StatusCode;
use serde::Deserialize;

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
    let mut conn = state
        .db
        .acquire()
        .await
        .expect("Error retrieving connection from pool");

    let rex = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.email,
        user.user_name,
        Utc::now()
    )
    .execute(&mut conn)
    .await
    .expect("Error executing");

    StatusCode::OK
}
