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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(state, user),
    fields(
        subscriber_email = %user.email,
        subscriber_name = %user.user_name
    )
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(user): Form<UserSubscribe>,
) -> StatusCode {
     match insert_subscriber(state, user).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNPROCESSABLE_ENTITY
    }
}

#[tracing::instrument(
    name = "Saving subscriber details on the database",
    skip(state, user)
)]
pub async fn insert_subscriber(state: AppState, user: UserSubscribe) -> Result<(), sqlx::Error> {
    let timestamp = Utc::now();

    let err = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.email,
        user.user_name,
        timestamp
    )
    .execute(&state.db).await;

    match err {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to execute db call {:?}", e);
            Err(e)
        }
    }
}
