use hyper::{StatusCode};
use axum::extract::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserSubscribe {
    user_name: String,
    email: String,
}

pub async fn subscribe(Form(user): Form<UserSubscribe>) -> StatusCode {
    println!("Test! {}, {}", user.user_name, user.email);
    StatusCode::OK
}
