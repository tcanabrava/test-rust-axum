use hyper::{StatusCode};
use axum::extract::{State, Form};
use serde::Deserialize;
use crate::{
    state::AppState,
};

#[derive(Deserialize)]
pub struct UserSubscribe {
    user_name: String,
    email: String,
}

pub async fn subscribe(State(state): State<AppState>, Form(user): Form<UserSubscribe>) -> StatusCode {
    println!("{}, {}, {}", user.user_name, user.email, state.db.size());
    StatusCode::OK
}
