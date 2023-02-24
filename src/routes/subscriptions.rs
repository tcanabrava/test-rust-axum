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
    let res = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.email,
        user.user_name,
        Utc::now()
    )
    .execute(&state.db)
    .await;

     match res {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::UNPROCESSABLE_ENTITY
        }
    }
}
